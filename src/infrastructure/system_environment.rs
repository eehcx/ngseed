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
}
