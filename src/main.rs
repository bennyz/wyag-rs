use std::path::PathBuf;

use clap::Clap;
use wyag_rs::Repository;

#[derive(Clap)]
#[clap(name = "wyag-es")]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    #[clap(name = "init")]
    Init(Init),
}

#[derive(Clap)]
struct Init {
    path: String,
}

fn main() {
    let args = Opts::parse();
    match args.subcmd {
        SubCommand::Init(s) => {
            let mut repo = Repository::default();
            repo.init(&PathBuf::from(s.path));
        }
    }
}
