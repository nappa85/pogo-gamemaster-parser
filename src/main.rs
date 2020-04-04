#![deny(warnings)]
#![deny(missing_docs)]

//! # pogo-gamemaster-parser
//!
//! Pokémon GO GameMaster parser

use std::env;

mod executor;
mod import;
mod entities;
mod moveset;
mod combat;

#[tokio::main]
async fn main() -> Result<(), ()> {
    env_logger::init();

    if let Some(filename) = env::args().skip(1).next() {
        executor::exec(&filename).await?;
    }

    Ok(())
}
