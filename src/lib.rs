use ignore::{DirEntry, WalkBuilder};
use std::{
    cmp::Ordering,
    path::{Path, PathBuf},
};
pub mod cli;
pub mod utils;
use std::fmt::Debug;
use tracing::{error, instrument, trace};

#[instrument]
pub fn get_candidates(
    root: impl AsRef<Path> + Debug, // For `tracing`. Could also #[instrument(skip(root))]
    include_hidden: bool,
    include_gitignored: bool,
) -> impl Iterator<Item = PathBuf> {
    let entries = WalkBuilder::new(root)
        .hidden(include_hidden)
        .parents(true)
        .git_ignore(!include_gitignored)
        .git_exclude(!include_gitignored)
        .build()
        .filter_map(|res| match res {
            Ok(ok) => {
                trace!("Found entry {:?}", ok);
                Some(ok)
            }
            Err(err) => {
                error!("Error from Walk: {:?}", err);
                None
            }
        });
    let mut paths = entries.map(DirEntry::into_path);

    if !include_gitignored {
        paths = paths.filter(predicate)
    }

    paths
}

#[instrument(skip(roots, key_fn))]
pub fn get_by_key<F, K>(
    roots: impl IntoIterator<Item = PathBuf>,
    key_fn: F,
    include_hidden: bool,
    include_gitignored: bool,
) -> Option<PathBuf>
where
    F: Fn(&PathBuf) -> Option<K>,
    K: Ord,
{
    let winner = roots
        .into_iter()
        .flat_map(|root| get_candidates(root, include_hidden, include_gitignored))
        .filter_map(|path| {
            if let Some(key) = key_fn(&path) {
                Some((path, key))
            } else {
                None
            }
        })
        .reduce(|winner, challenger| match winner.1.cmp(&challenger.1) {
            Ordering::Greater => winner,
            Ordering::Equal | Ordering::Less => challenger,
        })?;
    Some(winner.0)
}
