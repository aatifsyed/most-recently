use std::{cmp::Ordering, path::PathBuf};
use strum::{AsRefStr, EnumIter, EnumString, EnumVariantNames};
pub mod cli;

pub trait IterExt: Iterator + Sized {
    /// Allows the user to provide a fallible key
    fn max_by_maybe_key<F, K>(self, key_fn: F) -> Option<Self::Item>
    where
        F: Fn(&Self::Item) -> Option<K>,
        K: Ord,
    {
        let winner = self
            .filter_map(|path| {
                let key = key_fn(&path)?;
                Some((path, key))
            })
            .reduce(|winner, challenger| match winner.1.cmp(&challenger.1) {
                Ordering::Greater => winner,
                Ordering::Equal | Ordering::Less => challenger,
            })?;
        Some(winner.0)
    }
}

impl<T> IterExt for T where T: Iterator + Sized {}

#[derive(Debug, EnumString, EnumVariantNames, AsRefStr, EnumIter)]
#[strum(serialize_all = "kebab_case")]
pub enum Method {
    Accessed,
    Created,
    Modified,
}

pub trait MostRecently: Iterator<Item = PathBuf> + Sized {
    fn most_recently(self, method: Method) -> Option<Self::Item> {
        match method {
            Method::Accessed => {
                self.max_by_maybe_key(|path_buf| path_buf.metadata().ok()?.accessed().ok())
            }
            Method::Created => {
                self.max_by_maybe_key(|path_buf| path_buf.metadata().ok()?.created().ok())
            }
            Method::Modified => {
                self.max_by_maybe_key(|path_buf| path_buf.metadata().ok()?.modified().ok())
            }
        }
    }
}

impl<T> MostRecently for T where T: Iterator<Item = PathBuf> {}
