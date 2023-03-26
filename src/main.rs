mod settings;

use std::thread;
use std::time::Duration;
use clap::{Command, Arg, value_parser};
use strum_macros::{EnumString};
use std::str::FromStr;

#[derive(Debug, PartialEq, EnumString)]
enum ClsType {
    #[strum(serialize = "baremetall", serialize = "b")]
    Baremetall,

    #[strum(serialize = "virtual", serialize = "v")]
    Virtual,
}


fn cli() -> Command {
    Command::new("Calculator")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("A simple calculator CLI")
        .subcommand(
            Command::new("add")
                .about("Adds two numbers")
                .arg(Arg::new("a_add")
                    .value_parser(value_parser!(f64))
                    .required(true).help("First number"))
                .arg(Arg::new("b_add")
                    .value_parser(value_parser!(f64))
                    .required(true).help("Second number")),
        )
        .subcommand(
            Command::new("sub")
                .about("Subtracts two numbers")
                .arg(Arg::new("a_sub")
                    .value_parser(value_parser!(f64))
                    .required(true).help("First number"))
                .arg(Arg::new("b_sub")
                    .value_parser(value_parser!(f64))
                    .required(true).help("Second number")),
        )
        .subcommand(
            Command::new("loop")
                .about("Loop for 5 times")
                .arg(Arg::new("mess_loop")
                    .required(true).help("Loop message")),
        )
        .subcommand(
            Command::new("show")
                .about("show type")
                .arg(Arg::new("type")
                    .required(true)
                    .help("showing type")),
        )
}

fn main() {
    let s = settings::load().expect("Failed to load settings");
    println!("{:?}", s);

    let _guard = sentry::init((s.sentry_dsn, sentry::ClientOptions {
        release: sentry::release_name!(),
        ..Default::default()
    }));

    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("add", sub_matches)) => {
            let a: f64 = *sub_matches.get_one("a_add").expect("required");
            let b: f64 = *sub_matches.get_one("b_add").expect("required");
            let result = a + b;
            println!("{} + {} = {}", a, b, result);
        }
        Some(("sub", sub_matches)) => {
            let a: f64 = *sub_matches.get_one("a_sub").expect("required");
            let b: f64 = *sub_matches.get_one("b_sub").expect("required");
            let result = a - b;
            println!("{} - {} = {}", a, b, result);
        }
        Some(("loop", sub_matches)) => {
            let mut cnt = 4;
            if let Some(message) = sub_matches.get_one::<String>("mess_loop") {
                loop {
                    println!("{:?}", message);
                    cnt -= 1;
                    if cnt == 0 {
                        break;
                    }
                    thread::sleep(Duration::from_millis(4000));
                }
            }
        }
        Some(("show", sub_matches)) => {
            if let Some(ctp) = sub_matches.get_one::<String>("type") {
                let tp = ClsType::from_str(ctp).unwrap();
                match tp {
                    ClsType::Baremetall => println!("Baremetall server"),
                    ClsType::Virtual => println!("Virtual server")
                }
            }
        }
        _ => unreachable!()
    }
}