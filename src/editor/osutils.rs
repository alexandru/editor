use std::io;
use std::os::unix::process::CommandExt;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

const TO_CHECK: [&str; 6] = [
    ".git", 
    "node_modules", 
    "Cargo.toml", 
    "package.json",
    "build.sbt",
    ".vscode",
];

pub fn exec(program: &str, args: Vec<&str>) -> Result<(), io::Error> {
    let r = Command::new(program).args(args).exec();
    Err(r)
}

pub fn handle_open_command(paths: Vec<String>, wait: bool) -> Result<(), io::Error> {
    if paths.is_empty() {
        println!("No paths provided. Opening default editor.");
        exec("vim", vec![])?;
    } else {
        exec(
            "vim", 
            paths.iter().map(|s| s.as_str()).collect()
        )?;
    }
    if wait {
        println!("Waiting for editor to close...");
    }
    Ok(())
}

pub fn find_project_root_of_file(path: &Path) -> PathBuf {
    let first_dir = if path.is_dir() {
        path
    } else {
        path.parent().unwrap_or(path)
    };

    let mut current_path = Some(first_dir.to_path_buf());
    while let Some(cp) = current_path {
        for &to_check in TO_CHECK.iter() {
            if cp.join(to_check).is_dir() {
                return cp;
            }
        }
        current_path = cp.parent().map(|p| p.to_path_buf());
    }

    return first_dir.to_path_buf();
}
