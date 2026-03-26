use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};

use crate::domain::project::NewProjectRequest;
use crate::domain::project::PackageManager;
use crate::domain::project::UiChoice;

#[derive(Parser, Debug)]
#[command(
    name = "ngseed",
    version,
    about = "Initialize Angular projects with a clean architecture baseline",
    long_about = "A modern CLI to scaffold Angular projects, apply clean architecture structure, and integrate a UI stack.",
    after_help = "Examples:\n  ngseed new my-app\n  ngseed new my-app --yes --ui material --package-manager pnpm\n  ngseed new my-app --skip-install --ui none",
    arg_required_else_help = true
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    New(NewCommand),
}

#[derive(Parser, Debug)]
struct NewCommand {
    project_name: String,

    #[arg(long, value_enum)]
    ui: Option<CliUiChoice>,

    #[arg(long, value_enum)]
    package_manager: Option<CliPackageManager>,

    #[arg(long)]
    skip_install: bool,

    #[arg(long)]
    yes: bool,
}

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq)]
enum CliUiChoice {
    Material,
    Primeng,
    None,
}

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq)]
enum CliPackageManager {
    Npm,
    Pnpm,
    Yarn,
    Bun,
}

pub enum AppCommand {
    New(NewProjectRequest),
}

pub fn parse() -> Result<AppCommand> {
    Ok(map_cli_to_command(Cli::parse()))
}

#[cfg(test)]
pub fn parse_from<I, T>(itr: I) -> Result<AppCommand>
where
    I: IntoIterator<Item = T>,
    T: Into<std::ffi::OsString> + Clone,
{
    Ok(map_cli_to_command(Cli::parse_from(itr)))
}

fn map_cli_to_command(cli: Cli) -> AppCommand {
    match cli.command {
        Commands::New(cmd) => AppCommand::New(NewProjectRequest {
            project_name: cmd.project_name,
            ui: cmd.ui.map(Into::into),
            package_manager: cmd.package_manager.map(Into::into),
            skip_install: cmd.skip_install,
            yes: cmd.yes,
        }),
    }
}

impl From<CliUiChoice> for UiChoice {
    fn from(value: CliUiChoice) -> Self {
        match value {
            CliUiChoice::Material => UiChoice::Material,
            CliUiChoice::Primeng => UiChoice::Primeng,
            CliUiChoice::None => UiChoice::None,
        }
    }
}

impl From<CliPackageManager> for PackageManager {
    fn from(value: CliPackageManager) -> Self {
        match value {
            CliPackageManager::Npm => PackageManager::Npm,
            CliPackageManager::Pnpm => PackageManager::Pnpm,
            CliPackageManager::Yarn => PackageManager::Yarn,
            CliPackageManager::Bun => PackageManager::Bun,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_new_command_with_all_flags() {
        let command = parse_from([
            "ngseed",
            "new",
            "demo",
            "--yes",
            "--skip-install",
            "--ui",
            "primeng",
            "--package-manager",
            "pnpm",
        ])
        .unwrap();

        let AppCommand::New(request) = command;
        assert_eq!(request.project_name, "demo");
        assert_eq!(request.ui, Some(UiChoice::Primeng));
        assert_eq!(request.package_manager, Some(PackageManager::Pnpm));
        assert!(request.skip_install);
        assert!(request.yes);
    }
}
