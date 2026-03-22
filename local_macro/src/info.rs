use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CargoToml {
    #[allow(dead_code)]
    pub package: Package,
    pub lib: Option<Lib>,
    pub workspace: Workspace,
}

#[derive(Debug, Deserialize)]
pub struct Package {
    #[allow(dead_code)]
    pub name: String,
    #[allow(dead_code)]
    pub version: String,
    #[allow(dead_code)]
    pub edition: String,
}

#[derive(Debug, Deserialize)]
pub struct Workspace {
    pub members: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Lib {
    pub name: String,
}
