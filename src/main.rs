#![deny(warnings)]
#![deny(missing_docs)]

//! # pogo-gamemaster-parser
//!
//! Pokémon GO GameMaster parser

use std::path::PathBuf;

use structopt::StructOpt;

use pogo_gamemaster_parser::{League, exec};

#[derive(Debug, StructOpt)]
#[structopt(name = "pogo-gamemaster-parser", about = "Pokémon GO GameMaster Parser")]
struct Opt {
    #[structopt(short, long, help = "Mega, Ultra or Master")]
    league: League,

    /// team1 file, all if not present
    #[structopt(short, long, parse(from_os_str))]
    first_team: Option<PathBuf>,

    /// team2 file, all if not present
    #[structopt(short, long, parse(from_os_str))]
    second_team: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    env_logger::init();

    let opt = Opt::from_args();

    exec(&opt.league, opt.first_team.as_ref(), opt.second_team.as_ref()).await?;

    Ok(())
}
