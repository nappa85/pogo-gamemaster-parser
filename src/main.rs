#![deny(warnings)]
#![deny(missing_docs)]

//! # pogo-gamemaster-parser
//!
//! Pokémon GO GameMaster parser

mod entities;
mod import;
mod moveset;
mod combat;

#[tokio::main]
async fn main() -> Result<(), ()> {
    env_logger::init();

    import::exec().await?;

    Ok(())
}
