use anyhow::Result;

mod application;
mod domain;
mod infrastructure;
mod interfaces;

use application::use_cases::new_project::NewProjectUseCase;
use infrastructure::console_progress_reporter::ConsoleProgressReporter;
use infrastructure::dialoguer_ui_selector::DialoguerUiSelector;
use infrastructure::system_environment::SystemEnvironment;
use infrastructure::system_seeder::SystemSeeder;

pub fn run() -> Result<()> {
    let command = interfaces::cli::parse()?;

    match command {
        interfaces::cli::AppCommand::New(request) => {
            let env = SystemEnvironment;
            let ui_selector = DialoguerUiSelector;
            let seeder = SystemSeeder::default();
            let reporter = ConsoleProgressReporter::default();

            let use_case = NewProjectUseCase::new(&env, &ui_selector, &seeder, &reporter);
            use_case.execute(request)
        }
    }
}
