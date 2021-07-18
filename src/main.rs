use anyhow::{self, Context};
use itertools::Itertools;
use most_recent_file::cli;
use std::path::PathBuf;
use tracing::{debug, instrument};
use tracing_subscriber;

/// Translates from the CLI to real arguments, and runs the business logic of the program
#[instrument]
fn main() -> anyhow::Result<()> {
    use tracing_subscriber::{fmt::SubscriberBuilder, EnvFilter};
    SubscriberBuilder::default()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let matches = cli::app().get_matches();

    use most_recent_file::{cli::args::*, no};

    let include_hidden =
        matches.is_present(INCLUDE_HIDDEN) && !matches.is_present(no!(INCLUDE_HIDDEN));
    let include_gitignored =
        matches.is_present(INCLUDE_GITIGNORED) && !matches.is_present(no!(INCLUDE_GITIGNORED));
    let include_folders =
        matches.is_present(INCLUDE_FOLDERS) && !matches.is_present(no!(INCLUDE_FOLDERS));
    let paths = matches
        .values_of_os(PATHS)
        .with_context(|| {
            format!(
                "Argument inconsistency - {} is mandatory, but was found to be None",
                PATHS
            )
        })?
        .map(PathBuf::from)
        .collect_vec();

    debug!("arguments: {:?}", matches);
    debug!("include_hidden: {:?}", include_hidden);
    debug!("include_gitignored: {:?}", include_gitignored);
    debug!("include_folders: {:?}", include_folders);
    debug!("paths: {:?}", paths);
    Ok(())
}
