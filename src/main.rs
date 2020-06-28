mod database;
mod domain;
mod repository;

use clap::{Arg, App, ArgMatches};

fn parse() -> ArgMatches {
    App::new("Pérolas")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("A CLI application for registering remarkable quotes")
        .arg(Arg::new("v")
            .short('v')
            .multiple(true)
            .about("Sets the level of verbosity"))
        .subcommand(App::new("reset")
            .about("Clean the database"))
        .subcommand(App::new("register")
            .about("Register new pérola")
            .arg(Arg::with_name("author")
                .about("The pérola author")
                .short('a')
                .long("author")
                .value_name("AUTHOR")
                .takes_value(true))
            .arg(Arg::with_name("content")
                .about("The pérola content")
                .short('c')
                .long("content")
                .value_name("CONTENT")
                .takes_value(true))
            .arg(Arg::with_name("context")
                .about("The pérola context")
                .short('o')
                .long("context")
                .value_name("CONTEXT")
                .takes_value(true)))
        .subcommand(App::new("list")
            .about("List the registered pérolas"))
        .get_matches()
}

fn reset() {
    database::teardown();
}

fn register(args: &ArgMatches) {
    let perl = domain::Perl::new(
        args.value_of("author").unwrap(), 
        args.value_of("content").unwrap(), 
        args.value_of("context"));
    repository::add(&perl);
}

fn list() {
    repository::list()
        .into_iter()
        .map(|entry| entry.to_string())
        .for_each(|entry| println!("{}\n\n", entry));
}

fn main() {
    let matches = parse();
    match matches.subcommand() {
        ("reset", Some(_)) => reset(),
        ("register", Some(args)) => register(args),
        ("list", Some(_)) => list(),
        _ => println!("Invalid subcommand. Check `help` for more information"),
    }
}
