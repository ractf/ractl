use clap::{SubCommand, App};
use dialoguer::MultiSelect;
use dialoguer::theme::ColorfulTheme;
use std::fmt::Debug;

#[derive(Debug)]
struct Options {
    event_name: String,
    components: [bool; 3],
    secret_key: String,
    frontend_url: String,
    api_domain: String,
    internal_name: String,
    compose_path: String,
    user_email: String,
    andromeda_ip: String,
    andromeda_key: String,
    email_mode: String,
    aws_access_key_id: String,
    aws_access_key: String,
    sendgrid_api_key: String,
    email_server: String,
    email_user: String,
    email_pass: String,
    email_ssl: bool
}

impl Options {
    fn blank() -> Options {
        Options {
            event_name: "".to_string(),
            components: [false, false, false],
            secret_key: "".to_string(),
            frontend_url: "".to_string(),
            api_domain: "".to_string(),
            internal_name: "".to_string(),
            compose_path: "".to_string(),
            user_email: "".to_string(),
            andromeda_ip: "".to_string(),
            andromeda_key: "".to_string(),
            email_mode: "".to_string(),
            aws_access_key_id: "".to_string(),
            aws_access_key: "".to_string(),
            sendgrid_api_key: "".to_string(),
            email_server: "".to_string(),
            email_user: "".to_string(),
            email_pass: "".to_string(),
            email_ssl: false
        }
    }
}

pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    return SubCommand::with_name("install").about("Installs RACTF");
}

pub fn run() {
    if cfg!(windows) {
        println!("This script doesn't currently support windows.");
        println!("Maybe with your help, it could! Contributions to this script are welcome at https://github.com/ractf/ractl");
        return;
    }

    let mut options = Options::blank();

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
    for i in selected_services {
        options.components[i] = true;
    }
}
