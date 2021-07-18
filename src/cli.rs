use clap::{crate_authors, crate_description, crate_name, crate_version, App, AppSettings, Arg};
use tracing::instrument;

/// Extension trait for [`String`], allowing it to be use as a `&'static str`
trait IntoStaticStr {
    fn into_static_str(self) -> &'static str;
}

impl IntoStaticStr for String {
    fn into_static_str(self) -> &'static str {
        Box::leak(self.into_boxed_str())
    }
}

/// Extension trait for [`Arg`], which will add a `no-` version of the argument
trait WithNo: Sized {
    fn with_no(self, short: Option<impl AsRef<str>>) -> Vec<Self>;
}

impl<'a, 'b> WithNo for Arg<'a, 'b> {
    fn with_no(self, short: Option<impl AsRef<str>>) -> Vec<Self> {
        let yes = self;

        let yes_name = yes.b.name;
        let no_name = format!("no_{}", yes_name).into_static_str();

        let yes = yes.overrides_with(&no_name);

        let yes_long = yes.s.long.expect(&format!(
            "Argument {:?} does not have a long option",
            yes.b.name
        ));
        let no_long = format!("no-{}", yes_long).into_static_str();

        let mut no = Arg::with_name(&no_name)
            .long(&no_long)
            .overrides_with(yes_name);

        if let Some(short) = short {
            no = no.short(short)
        }

        vec![yes, no]
    }
}

pub mod args {
    pub const PATHS: &str = "paths";
    pub const INCLUDE_HIDDEN: &str = "include_hidden";
    pub const INCLUDE_GITIGNORED: &str = "include_gitignored";
    pub const INCLUDE_FOLDERS: &str = "include_folders";
}

#[macro_export]
macro_rules! no {
    ($ex:expr) => {{
        use const_format::concatcp;
        concatcp!("no_", $ex)
    }};
}

#[instrument]
pub fn app<'a, 'b>() -> App<'a, 'b> {
    use args::*;
    App::new(crate_name!())
        .author(crate_authors!())
        .version(crate_version!())
        .about(crate_description!())
        .setting(AppSettings::ColorAuto)
        .arg(
            Arg::with_name(PATHS)
                .help("A path to search for candidates in, or itself a candidate file. Defaults to current directory")
                .multiple(true).value_name("PATH").default_value("."),
        )
        .args(
            Arg::with_name(INCLUDE_HIDDEN)
                .short("h")
                .long("include-hidden")
                .help("Treat hidden files as candidates. Defaults to `no`")
                .with_no(Some("H"))
                .as_ref(),
        )
        .args(
            Arg::with_name(INCLUDE_GITIGNORED)
                .short("i")
                .long("include-gitignored")
                .help("Treat gitignored files as candidates. Defaults to `no`")
                .with_no(Some("I"))
                .as_ref(),
        )
        .args(
            Arg::with_name(INCLUDE_FOLDERS)
                .short("f")
                .long("include-folders")
                .help("Treat folders as candidates. Defaults to `no`")
                .with_no(Some("F"))
                .as_ref(),
        )
}
