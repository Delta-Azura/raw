use std::path::Path;
use std::fs;
use goblin::elf::Elf;

/// Returns the DT_NEEDED entries (e.g. `libc.so.6`) of an ELF file,
/// or an empty Vec if the file can't be read or is not an ELF.
pub fn get_needed_libs(path: &Path) -> Vec<String> {
    let buf = match fs::read(path) {
        Ok(b) => b,
        Err(_) => return vec![],
    };
    if let Ok(elf) = Elf::parse(&buf) {
        return elf.libraries.iter().map(|s| s.to_string()).collect();
    }
    vec![]
}

/// Collects the unique libraries needed by every file under `pkg_dir`.
pub fn scan_pkg_dir(pkg_dir: &Path) -> Vec<String> {
    let mut needed = std::collections::HashSet::new();
    for entry in walkdir::WalkDir::new(pkg_dir).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let libs = get_needed_libs(entry.path());
            needed.extend(libs);
        }
    }
    needed.into_iter().collect()
}