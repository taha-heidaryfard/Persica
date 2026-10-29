import React from 'react';
import DocsLayout from '../../components/DocsLayout';
import CodeBlock from '../../components/CodeBlock';
import { Download, CheckCircle, Terminal, AlertCircle } from 'lucide-react';

export default function InstallationPage() {
  const meta = {
    title: 'Installation & Setup',
    category: 'Getting Started',
    description: 'Everything you need to get the Persica compiler up and running on Windows, Linux, and macOS.',
  };

  const toc = [
    { title: 'Windows Setup Installer (Recommended)', id: 'windows-installer', level: 2 },
    { title: 'Verifying Installation', id: 'verify', level: 2 },
    { title: 'Build from Source (Cargo)', id: 'source', level: 2 },
    { title: 'VS Code Extension Setup', id: 'vscode-install', level: 2 },
    { title: 'Troubleshooting & PATH', id: 'troubleshooting', level: 2 },
  ];

  return (
    <DocsLayout meta={meta} toc={toc}>
      <h2 id="windows-installer" className="flex items-center gap-2 text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-8">
        <Download className="w-6 h-6 text-emerald-400" />
        Windows Setup Installer (Recommended)
      </h2>
      <p>
        The quickest and most reliable way to install Persica on a Windows 64-bit machine is using the official 1-click graphical installer.
      </p>

      <div className="my-6 p-5 rounded-2xl border border-emerald-500/30 bg-emerald-500/10 flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4">
        <div>
          <h4 className="text-emerald-300 font-bold text-sm">Download Official Windows Installer</h4>
          <p className="text-xs text-slate-300 mt-1">Version 0.13.0 (x64) • Ultra-compact 2.45 MB • Zero runtime dependencies</p>
        </div>
        <a
          href="https://github.com/taha-heidaryfard/Persica/releases/download/v0.13.0/Persica-0.13.0-Windows-x64-Setup.exe"
          className="inline-flex items-center gap-2 px-5 py-2.5 rounded-xl bg-emerald-500 hover:bg-emerald-600 text-white font-bold text-xs shadow-lg shadow-emerald-500/30 transition transform hover:-translate-y-0.5 shrink-0"
        >
          <Download className="w-4 h-4" />
          <span>Download Setup.exe</span>
        </a>
      </div>

      <p className="text-sm font-semibold text-slate-200">What the installer configures automatically:</p>
      <ul className="space-y-2 my-3 text-xs text-slate-300 list-disc pl-5">
        <li><strong>Automatic PATH Configuration:</strong> Adds Persica to your environment PATH so you can immediately open PowerShell or Command Prompt and run <code className="text-emerald-300">persica</code>.</li>
        <li><strong>User & Admin Modes:</strong> Installs for the current user without requiring Administrator privileges, or system-wide for all users when elevated.</li>
        <li><strong>File Association:</strong> Registers the <code className="text-emerald-300">.persi</code> file extension with Windows so you can execute scripts directly.</li>
        <li><strong>Clean Uninstaller:</strong> Registered in Windows Installed Apps for clean, one-click removal and PATH cleanup.</li>
      </ul>

      <h2 id="verify" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        Verifying Installation
      </h2>
      <p>
        After the setup wizard finishes, launch a new <strong>PowerShell</strong> or <strong>Command Prompt</strong> terminal window and test the compiler:
      </p>

      <CodeBlock
        language="bash"
        filename="Terminal"
        code={`persica --version`}
        output={`persica 0.13.0`}
      />

      <p>
        You can also run a quick inline test expression using the <code className="text-emerald-400">--eval</code> flag:
      </p>

      <CodeBlock
        language="bash"
        filename="Terminal"
        code={`persica --eval 'print("Persica is up and running! 🌿");'`}
        output={`Persica is up and running! 🌿`}
      />

      <h2 id="source" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        Building from Source (Rust & Cargo)
      </h2>
      <p>
        If you are on Linux, macOS, or prefer compiling the exact binary yourself, you can build directly using Rust’s standard toolchain:
      </p>

      <CodeBlock
        language="bash"
        filename="Terminal"
        code={`# 1. Clone the repository
git clone https://github.com/taha-heidaryfard/Persica.git
cd Persica

# 2. Run all tests to verify your environment
cargo test --all-targets

# 3. Build optimized release binary
cargo build --release`}
        output={`Finished \`release\` profile [optimized] target(s) in 11.2s`}
      />

      <p>
        The compiled executable will be located at:
      </p>
      <ul className="text-xs text-slate-300 list-disc pl-5 space-y-1 my-2">
        <li>Windows: <code className="text-emerald-300">target\release\persica.exe</code></li>
        <li>Linux / macOS: <code className="text-emerald-300">target/release/persica</code></li>
      </ul>

      <h2 id="vscode-install" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        VS Code Extension Installation
      </h2>
      <p>
        For the best editing experience with syntax highlighting, IntelliSense auto-complete, live type checking on save, and hover documentation, install the official Persica extension:
      </p>

      <div className="my-4">
        <a
          href="https://github.com/taha-heidaryfard/Persica/releases/download/v0.13.0/persica-0.13.0.vsix"
          className="inline-flex items-center gap-2 px-4 py-2 rounded-xl border border-slate-700 bg-slate-900 hover:bg-slate-800 text-xs font-semibold text-slate-200 transition"
        >
          <Download className="w-4 h-4 text-emerald-400" />
          <span>Download persica-0.13.0.vsix</span>
        </a>
      </div>

      <p className="text-xs text-slate-400">Install via command line:</p>
      <CodeBlock
        language="bash"
        filename="Terminal"
        code={`code --install-extension persica-0.13.0.vsix`}
        output={`Extension 'persica-0.13.0.vsix' was successfully installed.`}
      />

      <h2 id="troubleshooting" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        Troubleshooting & PATH Issues
      </h2>
      <div className="p-4 rounded-xl border border-amber-500/30 bg-amber-500/10 text-xs text-amber-200 space-y-2">
        <div className="flex items-center gap-2 font-bold text-amber-300">
          <AlertCircle className="w-4 h-4" />
          <span>Command Not Found in Terminal?</span>
        </div>
        <p>
          If Windows displays <code>'persica' is not recognized as an internal or external command</code>, ensure you have closed and reopened your terminal window after installing so the updated system environment variables take effect.
        </p>
      </div>
    </DocsLayout>
  );
}
