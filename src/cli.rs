use crate::Method;
use clap::{
    crate_description, crate_name, crate_version, App, AppSettings, Arg, ArgGroup, Shell,
    SubCommand,
};
use itertools::Itertools;
use strum::{IntoEnumIterator, VariantNames};
use tracing::instrument;

pub mod args {
    pub const COMPLETIONS: &str = "completions";
    pub const SHELL: &str = "shell";
    pub const PATHS: &str = "paths";
    pub const STDIN: &str = "stdin";
    pub const INPUT: &str = "stdin_or_paths";
}

impl Method {
    fn helptext(&self) -> &'static str {
        match self {
            Method::Accessed => "Sort by access on this file or directory",
            Method::Created => "Sort by creation on this volume",
            Method::Modified => "Sort by modification of the file or directory",
        }
    }
}

#[instrument]
pub fn app<'a, 'b>() -> App<'a, 'b> {
    use args::*;
    App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .setting(AppSettings::ColorAuto)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .usage(
            format!(
                "{} <{}> <--stdin | PATH...>\n OR:\n    {} completions <{}>",
                crate_name!(),
                Method::VARIANTS.iter().format(" | "),
                crate_name!(),
                Shell::variants().iter().format(" | ")
            )
            .into_static_str(),
        )
        .subcommand(
            SubCommand::with_name(COMPLETIONS)
                .about("Print out a shell completion script for a given shell")
                .arg(
                    Arg::with_name(SHELL)
                        .takes_value(true)
                        .value_name("shell")
                        .possible_values(Shell::variants().as_ref()),
                ),
        )
        // A little ugly, but proves to be superior to argument groups
        .subcommands(Method::iter().map(|method| {
            SubCommand::with_name(method.as_ref())
                .about(method.helptext())
                .display_order(0)
                .arg(
                    Arg::with_name(PATHS)
                        .help("Candidate paths. May be files or folders")
                        .min_values(1),
                )
                .arg(
                    Arg::with_name(STDIN)
                        .short("s")
                        .long("stdin")
                        .help("Read candidate paths from stdin (one line per path)"),
                )
                .group(
                    ArgGroup::with_name(INPUT)
                        .arg(PATHS)
                        .arg(STDIN)
                        .multiple(false)
                        .required(true),
                )
        }))
}

trait IntoStaticStr {
    fn into_static_str(self) -> &'static str;
}

impl IntoStaticStr for String {
    fn into_static_str(self) -> &'static str {
        Box::leak(self.into_boxed_str())
    }
}
