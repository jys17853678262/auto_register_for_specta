// import use modules
use std::{
    env, fs,
    path::{Path, PathBuf},
};

use super::info::CargoToml;

#[allow(dead_code)]
/// find the workspace directory from the given directory
pub fn find_workspace_dir(start_dir: &Path) -> PathBuf {
    let mut current_dir = start_dir.to_path_buf();

    // 最多向上查找 20 层
    for _ in 0..20 {
        let cargo_toml_path = current_dir.join("Cargo.toml");

        if cargo_toml_path.exists() {
            if let Ok(contents) = fs::read_to_string(&cargo_toml_path) {
                // 检查是否是 workspace
                if contents.contains("[workspace]") {
                    return current_dir;
                }
            }
        }

        // 向上移动一层
        if !current_dir.pop() {
            break;
        }
    }

    panic!("Workspace root not found from {}", start_dir.display());
}

#[allow(dead_code)]
/// get the workspace members from the given workspace directory
pub fn get_workspace_members(workspace_root: &Path) -> Vec<String> {
    let cargo_toml = workspace_root.join("Cargo.toml");
    let contents = fs::read_to_string(&cargo_toml).unwrap_or_else(|_| {
        panic!(
            "Failed to read workspace Cargo.toml at {}",
            cargo_toml.display()
        );
    });

    let toml_content: CargoToml = toml::from_str(&contents).unwrap();

    toml_content.workspace.members
}

#[allow(dead_code)]
/// get the workspace information from the given workspace directory
pub fn get_workspace() -> CargoToml {
    let workspace_root = find_workspace_dir(Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()));

    let cargo_toml = workspace_root.join("Cargo.toml");
    let toml_contents = fs::read_to_string(&cargo_toml).unwrap_or_else(|_| {
        panic!(
            "Failed to read workspace Cargo.toml at {}",
            cargo_toml.display()
        );
    });

    let toml_content: CargoToml = toml::from_str(&toml_contents).unwrap();
    toml_content
}

#[allow(dead_code)]
/// get the workspace package name from the given workspace directory
pub fn get_workspace_pkg_name() -> String {
    let cont = get_workspace();
    if cont.lib.is_some() {
        return cont.lib.unwrap().name.clone();
    } else {
        return cont.package.name.clone();
    }
}
