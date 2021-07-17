use ignore::{DirEntry, WalkBuilder};
use std::path::{Path, PathBuf};

pub fn get_candidates(
    root: impl AsRef<Path>,
    include_hidden: bool,
    include_gitignored: bool,
) -> impl Iterator<Item = PathBuf> {
    WalkBuilder::new(root)
        .hidden(include_hidden)
        .parents(true)
        .git_ignore(!include_gitignored)
        .git_exclude(!include_gitignored)
        .build()
        .filter_map(Result::ok)
        .map(DirEntry::into_path)
}
