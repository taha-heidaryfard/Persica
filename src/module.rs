use crate::{
    ast::Program,
    diagnostic::{Diagnostic, Phase},
    lexer, parser,
    source::Span,
};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    sync::RwLock,
};

// Global in-memory virtual modules repository (useful for testing and embeds)
static VIRTUAL_MODULES: RwLock<Option<HashMap<String, String>>> = RwLock::new(None);
static PROJECT_SEARCH_PATHS: RwLock<Vec<PathBuf>> = RwLock::new(Vec::new());

pub fn set_project_search_paths(paths: Vec<PathBuf>) {
    let mut lock = PROJECT_SEARCH_PATHS.write().unwrap();
    *lock = paths;
}

pub fn get_project_search_paths() -> Vec<PathBuf> {
    let lock = PROJECT_SEARCH_PATHS.read().unwrap();
    lock.clone()
}

pub fn clear_project_search_paths() {
    let mut lock = PROJECT_SEARCH_PATHS.write().unwrap();
    lock.clear();
}

pub fn register_virtual_module(name: impl Into<String>, source: impl Into<String>) {
    let mut lock = VIRTUAL_MODULES.write().unwrap();
    if lock.is_none() {
        *lock = Some(HashMap::new());
    }
    lock.as_mut().unwrap().insert(name.into(), source.into());
}

pub fn get_virtual_module(name: &str) -> Option<String> {
    let lock = VIRTUAL_MODULES.read().unwrap();
    lock.as_ref().and_then(|map| map.get(name).cloned())
}

pub fn clear_virtual_modules() {
    let mut lock = VIRTUAL_MODULES.write().unwrap();
    *lock = None;
}

pub fn resolve_module_path(module_specifier: &str, base_dir: Option<&Path>) -> Option<PathBuf> {
    let rel_path = if module_specifier.contains('/')
        || module_specifier.contains('\\')
        || module_specifier.ends_with(".persi")
    {
        PathBuf::from(module_specifier)
    } else {
        let parts: Vec<&str> = module_specifier.split('.').collect();
        let mut p = PathBuf::new();
        for part in parts {
            p.push(part);
        }
        p
    };

    let mut candidates = Vec::new();
    if rel_path.extension().is_some() {
        candidates.push(rel_path.clone());
    } else {
        let mut with_ext = rel_path.clone();
        with_ext.set_extension("persi");
        candidates.push(with_ext);
        candidates.push(rel_path.join("mod.persi"));
        candidates.push(rel_path.join("index.persi"));
    }

    for candidate in candidates {
        if let Some(base) = base_dir {
            let full = base.join(&candidate);
            if full.is_file() {
                return Some(full);
            }
        }
        for search_path in get_project_search_paths() {
            let full = search_path.join(&candidate);
            if full.is_file() {
                return Some(full);
            }
        }
        if candidate.is_file() {
            if let Ok(abs) = std::env::current_dir() {
                return Some(abs.join(&candidate));
            }
            return Some(candidate);
        }
        if let Ok(cwd) = std::env::current_dir() {
            let full = cwd.join(&candidate);
            if full.is_file() {
                return Some(full);
            }
        }
    }
    None
}

pub fn load_module_source(
    module_specifier: &str,
    base_dir: Option<&Path>,
    span: Span,
) -> Result<(String, String, Option<PathBuf>), Diagnostic> {
    if let Some(source) = get_virtual_module(module_specifier) {
        return Ok((module_specifier.to_string(), source, None));
    }

    if let Some(path) = resolve_module_path(module_specifier, base_dir) {
        let key = path.to_string_lossy().to_string();
        let file_dir = path.parent().map(|p| p.to_path_buf());
        let source = fs::read_to_string(&path).map_err(|err| {
            Diagnostic::new(
                Phase::Semantic,
                format!("could not read module '{module_specifier}': {err}"),
                span,
            )
        })?;
        return Ok((key, source, file_dir));
    }

    if let Some(source) = crate::stdlib::get_stdlib_module(module_specifier) {
        return Ok((module_specifier.to_string(), source.to_string(), None));
    }

    Err(Diagnostic::new(
        Phase::Semantic,
        format!("module '{module_specifier}' not found"),
        span,
    ))
}

pub fn parse_module(source: &str, span: Span) -> Result<Program, Diagnostic> {
    let tokens = lexer::tokenize(source).map_err(|mut d| {
        d.span = Some(span);
        d
    })?;
    parser::parse(&tokens).map_err(|mut d| {
        d.span = Some(span);
        d
    })
}
