use anyhow::{self, Context};
use clap::{crate_name, Shell};
use most_recently::{
    cli::{self, args::*},
    Method, MostRecently,
};
use std::io::{stdin, BufRead};
use std::{path::PathBuf, str::FromStr};
use tracing::{debug, instrument};
use tracing_subscriber::{fmt::SubscriberBuilder, EnvFilter};

/// Translates from the CLI to real arguments, and runs the business logic of the program
#[instrument]
fn main() -> anyhow::Result<()> {
    SubscriberBuilder::default()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let matches = cli::app().get_matches();
    debug!("Arguments: {:?}", matches);

    if let Some(matches) = matches.subcommand_matches(COMPLETIONS) {
        let shell = matches
            .value_of(SHELL)
            .expect("completions must specify shell");

        debug!("Printing completions for {}", shell);

        let shell = Shell::from_str(shell).expect("invalid shells are disallowed");
        cli::app().gen_completions_to(crate_name!(), shell, &mut std::io::stdout());
        Ok(())
    } else {
        let (method, matches) = matches.subcommand();
        let method = Method::from_str(method).expect("subcommand isn't a valid method");
        let matches = matches.expect("methods must have matches");

        debug!("Printing most recently {}", method.as_ref());

        let most_recent = match matches.values_of(PATHS) {
            None => stdin()
                .lock()
                .lines()
                .filter_map(Result::ok)
                .map(PathBuf::from)
                .most_recently(method),
            Some(values) => values.map(PathBuf::from).most_recently(method),
        };
        let most_recent = most_recent.with_context(|| "No viable candidate")?;

        debug!("{:?}", most_recent);
        println!("{}", most_recent.display());
        Ok(())
    }
}
