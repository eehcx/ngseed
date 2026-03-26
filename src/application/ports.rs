use std::path::Path;
use std::path::PathBuf;

use anyhow::Result;

use crate::domain::project::PackageManager;
use crate::domain::project::ResolvedOptions;
use crate::domain::project::UiChoice;

pub trait UiSelector {
    fn select_ui(&self) -> Result<UiChoice>;
    fn select_package_manager(&self) -> Result<PackageManager>;
}

pub trait Environment {
    fn project_exists(&self, project_name: &str) -> bool;
    fn current_dir(&self) -> Result<PathBuf>;
}

pub trait Seeder {
    fn ensure_required_tools(&self, package_manager: PackageManager) -> Result<()>;
    fn scaffold_angular_project(&self, project_name: &str, options: ResolvedOptions) -> Result<()>;
    fn apply_clean_architecture_template(&self, project_dir: &Path) -> Result<()>;
    fn apply_ui_integration(
        &self,
        project_dir: &Path,
        ui: UiChoice,
        package_manager: PackageManager,
    ) -> Result<()>;
}

pub trait ProgressReporter {
    fn stage_start(&self, stage: &str, message: &str);
    fn stage_ok(&self, stage: &str, message: &str);
    fn stage_error(&self, stage: &str, message: &str);
    fn summary(&self, project_name: &str, project_dir: &Path, options: ResolvedOptions);
}
