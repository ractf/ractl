use clap::{SubCommand, App};
use dialoguer::MultiSelect;
use dialoguer::theme::ColorfulTheme;

pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    return SubCommand::with_name("install").about("Installs RACTF");
}

pub fn run() {
    let possible_services = &[
        "Andromeda",
        "Core",
        "Shell"
    ];
    let defaults = &[false, true, true];
    let selected_services = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Which services would you like to install?")
        .items(&possible_services[..])
        .defaults(&defaults[..])
        .interact()
        .unwrap();

    if selected_services.is_empty() {
        println!("Please select at least one service to install.");
        return;
    }
}
