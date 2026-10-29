; Script generated for Persica Programming Language
; Inno Setup 7 Script

#define MyAppName "Persica"
#define MyAppVersion "0.13.0"
#define MyAppPublisher "Persica Language Team"
#define MyAppURL "https://github.com/persica-lang/persica"
#define MyAppExeName "persica.exe"

[Setup]
AppId={{C6B8D321-4E65-4982-8BA2-6A1021469D51}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
AppVerName={#MyAppName} {#MyAppVersion}
AppPublisher={#MyAppPublisher}
AppPublisherURL={#MyAppURL}
AppSupportURL={#MyAppURL}
AppUpdatesURL={#MyAppURL}
DefaultDirName={autopf}\{#MyAppName}
DefaultGroupName={#MyAppName}
DisableProgramGroupPage=yes
OutputDir=..\dist
OutputBaseFilename=Persica-{#MyAppVersion}-Windows-x64-Setup
Compression=lzma2/ultra64
SolidCompression=yes
WizardStyle=modern
ArchitecturesInstallIn64BitMode=x64compatible
PrivilegesRequired=lowest
PrivilegesRequiredOverridesAllowed=commandline dialog
UsedUserAreasWarning=no
ChangesEnvironment=yes
ChangesAssociations=yes

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "addtopath"; Description: "Add Persica to PATH environment variable (Recommended)"; GroupDescription: "System Integration:"; Flags: checkedonce
Name: "assocpersi"; Description: "Associate .persi files with Persica"; GroupDescription: "File Associations:"; Flags: checkedonce

[Files]
Source: "..\target\release\{#MyAppExeName}"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\CHANGELOG.md"; DestDir: "{app}"; Flags: ignoreversion; DestName: "CHANGELOG.txt"

[Registry]
; Add to PATH for Admin (System) install
Root: HKLM; Subkey: "SYSTEM\CurrentControlSet\Control\Session Manager\Environment"; \
    ValueType: expandsz; ValueName: "Path"; ValueData: "{olddata};{app}"; \
    Check: NeedsAddPath(ExpandConstant('{app}')) and IsAdminInstallMode; Tasks: addtopath

; Add to PATH for Non-Admin (User) install
Root: HKCU; Subkey: "Environment"; \
    ValueType: expandsz; ValueName: "Path"; ValueData: "{olddata};{app}"; \
    Check: NeedsAddPathUser(ExpandConstant('{app}')) and not IsAdminInstallMode; Tasks: addtopath

; Associate .persi extension
Root: HKA; Subkey: "Software\Classes\.persi"; ValueType: string; ValueName: ""; ValueData: "PersicaScript"; Flags: uninsdeletevalue; Tasks: assocpersi
Root: HKA; Subkey: "Software\Classes\PersicaScript"; ValueType: string; ValueName: ""; ValueData: "Persica Source File"; Flags: uninsdeletekey; Tasks: assocpersi
Root: HKA; Subkey: "Software\Classes\PersicaScript\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\{#MyAppExeName},0"; Tasks: assocpersi
Root: HKA; Subkey: "Software\Classes\PersicaScript\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#MyAppExeName}"" ""%1"""; Tasks: assocpersi

[Run]
Filename: "{app}\{#MyAppExeName}"; Parameters: "--version"; Description: "Check installed Persica version"; Flags: nowait postinstall skipifsilent shellexec

[Code]
function NeedsAddPath(Param: string): boolean;
var
  OrigPath: string;
begin
  if not RegQueryStringValue(HKEY_LOCAL_MACHINE,
    'SYSTEM\CurrentControlSet\Control\Session Manager\Environment',
    'Path', OrigPath)
  then begin
    Result := True;
    exit;
  end;
  Result := Pos(';' + UpperCase(Param) + ';', ';' + UpperCase(OrigPath) + ';') = 0;
end;

function NeedsAddPathUser(Param: string): boolean;
var
  OrigPath: string;
begin
  if not RegQueryStringValue(HKEY_CURRENT_USER,
    'Environment',
    'Path', OrigPath)
  then begin
    Result := True;
    exit;
  end;
  Result := Pos(';' + UpperCase(Param) + ';', ';' + UpperCase(OrigPath) + ';') = 0;
end;

procedure RemovePath(RootKey: Integer; SubKey: string; PathToRemove: string);
var
  OrigPath, NewPath: string;
  P, L: Integer;
begin
  if RegQueryStringValue(RootKey, SubKey, 'Path', OrigPath) then
  begin
    OrigPath := ';' + OrigPath + ';';
    P := Pos(';' + UpperCase(PathToRemove) + ';', UpperCase(OrigPath));
    if P > 0 then
    begin
      L := Length(PathToRemove) + 1;
      Delete(OrigPath, P, L);
      NewPath := Copy(OrigPath, 2, Length(OrigPath) - 2);
      RegWriteExpandStringValue(RootKey, SubKey, 'Path', NewPath);
    end;
  end;
end;

procedure CurUninstallStepChanged(CurUninstallStep: TUninstallStep);
begin
  if CurUninstallStep = usPostUninstall then
  begin
    if IsAdminInstallMode then
      RemovePath(HKEY_LOCAL_MACHINE, 'SYSTEM\CurrentControlSet\Control\Session Manager\Environment', ExpandConstant('{app}'))
    else
      RemovePath(HKEY_CURRENT_USER, 'Environment', ExpandConstant('{app}'));
  end;
end;
