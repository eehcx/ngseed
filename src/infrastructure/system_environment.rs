use std::path::PathBuf;

use anyhow::{Context, Result};

use crate::application::ports::Environment;

pub struct SystemEnvironment;

impl Environment for SystemEnvironment {
    fn project_exists(&self, project_name: &str) -> bool {
        PathBuf::from(project_name).exists()
    }

    fn current_dir(&self) -> Result<PathBuf> {
        std::env::current_dir().context("unable to resolve current directory")
    }

    fn is_ci(&self) -> bool {
        std::env::var_os("CI").is_some()
    }

    fn is_interactive_terminal(&self) -> bool {
        use std::io::IsTerminal;
        std::io::stdout().is_terminal()
    }
}
