use anyhow::{self, Context};
use clap::crate_name;
use clap::Shell;
use itertools::Itertools;
use most_recent_file::{
    cli::{self, args::*},
    get_by_key, no,
};
use std::{fmt::Debug, path::PathBuf, str::FromStr};
use thiserror::Error;
use tracing::{debug, error, instrument};
use tracing_subscriber::{fmt::SubscriberBuilder, EnvFilter};

#[derive(Debug, Error)]
enum InconsistentArgument<A: Debug, B: Debug> {
    #[error("Expected to be one of {variants:?}, but found to be {got:?}")]
    ImpossibleVariant { variants: A, got: B },
    #[error("Mandatory argument {argument:?} wasn't present")]
    MandatoryArgumentNotPresent { argument: A },
}

/// Translates from the CLI to real arguments, and runs the business logic of the program
#[instrument]
fn main() -> anyhow::Result<()> {
    SubscriberBuilder::default()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let matches = cli::app().get_matches();

    if let Some(shell) = matches.value_of(COMPLETIONS) {
        debug!("Printing completions for {}", shell);
        let shell =
            Shell::from_str(shell).map_err(|_| InconsistentArgument::ImpossibleVariant {
                variants: Shell::variants(),
                got: shell.to_owned(),
            })?;
        cli::app().gen_completions_to(crate_name!(), shell, &mut std::io::stdout());
        return Ok(());
    }

    let include_hidden =
        matches.is_present(INCLUDE_HIDDEN) && !matches.is_present(no!(INCLUDE_HIDDEN));
    let include_gitignored =
        matches.is_present(INCLUDE_GITIGNORED) && !matches.is_present(no!(INCLUDE_GITIGNORED));
    let include_folders =
        matches.is_present(INCLUDE_FOLDERS) && !matches.is_present(no!(INCLUDE_FOLDERS));
    let paths = matches
        .values_of_os(PATHS)
        .with_context(
            || InconsistentArgument::<_, &str>::MandatoryArgumentNotPresent { argument: PATHS },
        )?
        .map(PathBuf::from)
        .collect_vec();

    debug!("arguments: {:?}", matches);
    debug!("include_hidden: {:?}", include_hidden);
    debug!("include_gitignored: {:?}", include_gitignored);
    debug!("include_folders: {:?}", include_folders);
    debug!("paths: {:?}", paths);

    let most_recent = get_by_key(
        paths,
        |path| {
            path.metadata()
                .and_then(|metadata| metadata.accessed())
                .ok()
        },
        include_hidden,
        include_gitignored,
    )
    .with_context(|| "No viable candidate")?;
    println!("{:?}", most_recent);
    Ok(())
}
