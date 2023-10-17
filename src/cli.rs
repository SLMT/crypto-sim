use std::io;

use dialoguer::Select;

pub struct Cli;

impl Cli {
    pub fn new() -> Cli {
        Cli
    }

    pub fn run(&mut self) -> Result<(), String> {
        let action = self.select_action();
        // TODO: act according to chosen action
        todo!()
    }

    pub fn select_action(&mut self) -> Result<usize, String> {
        let items = vec!["Create Portfolio", "List Portfolios", "Show Portfolio"];

        let selection = Select::new()
            .with_prompt("Action?")
            .items(&items)
            .interact()
            .map_err(|e| e.to_string())?;

        Ok(selection)
    }
}
