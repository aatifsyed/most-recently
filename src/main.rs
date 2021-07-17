use most_recent_file::get_candidates;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(default_value = ".", parse(from_os_str))]
    path: Vec<PathBuf>,
    #[structopt(long)]
    include_hidden: bool,
    #[structopt(long)]
    include_gitignored: bool,
    #[structopt(long)]
    include_folders: bool,
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
    let winner = opt
        .path
        .iter()
        .map(|path| get_candidates(path, opt.include_hidden, opt.include_gitignored))
        .flatten()
        .filter(|path| path.is_file())
        .max_by_key(|path| path.metadata().unwrap().accessed().unwrap());
    println!("{:?}", winner);
}
