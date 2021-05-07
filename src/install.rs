use clap::{SubCommand, App};
use dialoguer::MultiSelect;
use dialoguer::theme::ColorfulTheme;
use std::fmt::Debug;
use console::style;
use std::{env, fs};

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
    validate_environment();
    println!("{} {} {}", style("Welcome to the").cyan(), style("RACTF").bold(), style("installer").cyan());

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

fn validate_environment() {
    if cfg!(windows) {
        println!("{}", style("This script doesn't currently support windows.").red());
        println!("{}", style("Maybe with your help, it could! Contributions to this script are welcome at https://github.com/ractf/ractl").red());
        std::process::exit(1);
    }

    require_command("docker");
    require_command("docker-compose");
}

fn require_command(command: &str) {
    if let Ok(path) = env::var("PATH") {
        for folder in path.split(":") {
            let file = format!("{}/{}", folder, command);
            if fs::metadata(file).is_ok() {
                return;
            }
        }
    }

    println!("{}", style(command.to_owned() + ", a dependency of this script, doesn't appear to be installed.").red());
    println!("{}", style("If it is, ensure its executable is in the current user's PATH.").red());
    std::process::exit(1);
}
