use std::{env, fs, os::unix::fs::PermissionsExt, path::PathBuf};

pub fn get_exe_paths() -> Vec<String> {
    let mut exes = Vec::new();

    if let Ok(paths) = env::var("PATH") {
        for dir in env::split_paths(&paths) {
            if let Ok(entries) = fs::read_dir(dir) {
                for entry in entries.filter_map(|e| e.ok()) {
                    if is_exe(&entry.path()) {
                        exes.push(entry.file_name().to_string_lossy().to_string());
                    }
                }
            }
        }
    }

    exes
}

pub fn get_exe_path(command: &str) -> Option<String> {
    if let Ok(paths) = env::var("PATH") {
        for dir in env::split_paths(&paths) {
            let path = dir.join(command);
            if path.is_file() && is_exe(&path) {
                return Some(path.to_string_lossy().to_string());
            }
        }
    }

    None
}

pub fn is_exe(path: &PathBuf) -> bool {
    if let Ok(metadata) = path.metadata() {
        let permissions = metadata.permissions();
        return permissions.mode() & 0o111 != 0;
    }

    return false;
}
