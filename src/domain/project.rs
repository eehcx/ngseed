#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiChoice {
    Material,
    Primeng,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackageManager {
    Npm,
    Pnpm,
    Yarn,
    Bun,
}

#[derive(Debug, Clone, Copy)]
pub struct ResolvedOptions {
    pub ui: UiChoice,
    pub package_manager: PackageManager,
    pub skip_install: bool,
}

#[derive(Debug, Clone)]
pub struct NewProjectRequest {
    pub project_name: String,
    pub ui: Option<UiChoice>,
    pub package_manager: Option<PackageManager>,
    pub skip_install: bool,
    pub yes: bool,
}
