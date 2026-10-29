use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectConfig {
    pub name: String,
    pub version: String,
    pub entry: String,
    pub description: String,
    pub source_dir: String,
    pub packages_dir: String,
    pub root_dir: PathBuf,
}

impl Default for ProjectConfig {
    fn default() -> Self {
        Self {
            name: "my_project".to_string(),
            version: "0.1.0".to_string(),
            entry: "src/main.persi".to_string(),
            description: "A Persica application".to_string(),
            source_dir: "src".to_string(),
            packages_dir: ".persi_env".to_string(),
            root_dir: PathBuf::new(),
        }
    }
}

impl ProjectConfig {
    pub fn new(name: impl Into<String>, root_dir: PathBuf) -> Self {
        Self {
            name: name.into(),
            root_dir,
            ..Self::default()
        }
    }

    pub fn to_toml_string(&self) -> String {
        format!(
            "[package]\nname = \"{}\"\nversion = \"{}\"\nentry = \"{}\"\ndescription = \"{}\"\n\n[paths]\nsource = \"{}\"\npackages = \"{}\"\n",
            self.name,
            self.version,
            self.entry,
            self.description,
            self.source_dir,
            self.packages_dir
        )
    }

    pub fn parse_toml(content: &str, root_dir: PathBuf) -> Result<Self, String> {
        let mut config = Self {
            root_dir,
            ..Self::default()
        };

        let mut current_section = String::new();

        for (line_idx, raw_line) in content.lines().enumerate() {
            let line = raw_line.trim();
            if line.is_empty() || line.starts_with('#') || line.starts_with("//") {
                continue;
            }

            if line.starts_with('[') && line.ends_with(']') {
                current_section = line[1..line.len() - 1].trim().to_ascii_lowercase();
                continue;
            }

            if let Some((key_part, val_part)) = line.split_once('=') {
                let key = key_part.trim().to_ascii_lowercase();
                let mut val = val_part.trim();
                if ((val.starts_with('"') && val.ends_with('"'))
                    || (val.starts_with('\'') && val.ends_with('\'')))
                    && val.len() >= 2
                {
                    val = &val[1..val.len() - 1];
                }

                match (current_section.as_str(), key.as_str()) {
                    ("package", "name") => config.name = val.to_string(),
                    ("package", "version") => config.version = val.to_string(),
                    ("package", "entry") => config.entry = val.to_string(),
                    ("package", "description") => config.description = val.to_string(),
                    ("paths", "source") => config.source_dir = val.to_string(),
                    ("paths", "packages") => config.packages_dir = val.to_string(),
                    _ => {}
                }
            } else {
                return Err(format!(
                    "invalid syntax on line {}: '{}'",
                    line_idx + 1,
                    line
                ));
            }
        }

        Ok(config)
    }

    pub fn load_from_file(toml_path: &Path) -> Result<Self, String> {
        let content = fs::read_to_string(toml_path).map_err(|e| {
            format!(
                "could not read project config '{}': {e}",
                toml_path.display()
            )
        })?;
        let root_dir = toml_path
            .parent()
            .map(Path::to_path_buf)
            .unwrap_or_else(|| PathBuf::from("."));
        Self::parse_toml(&content, root_dir)
    }

    pub fn find_in_ancestors(start_dir: &Path) -> Option<PathBuf> {
        let mut current = if start_dir.is_file() {
            start_dir.parent()?.to_path_buf()
        } else {
            start_dir.to_path_buf()
        };

        loop {
            let candidate = current.join("persica.toml");
            if candidate.is_file() {
                return Some(candidate);
            }
            if !current.pop() {
                break;
            }
        }
        None
    }

    pub fn get_search_paths(&self) -> Vec<PathBuf> {
        let mut paths = Vec::new();
        paths.push(self.root_dir.clone());

        if !self.source_dir.is_empty() {
            paths.push(self.root_dir.join(&self.source_dir));
        }

        if !self.packages_dir.is_empty() {
            let pkg_dir = self.root_dir.join(&self.packages_dir);
            paths.push(pkg_dir.clone());
            paths.push(pkg_dir.join("packages"));
            paths.push(pkg_dir.join("modules"));
        }

        paths
    }

    pub fn entry_path(&self) -> PathBuf {
        self.root_dir.join(&self.entry)
    }
}
