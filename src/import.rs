use std::collections::HashMap;

use tokio::{fs::File, io::AsyncReadExt};

use serde::Deserialize;

use log::error;

use crate::moveset::Moveset;

#[derive(Clone, Deserialize)]
struct Input {
    pokemon: String,
    form: Option<String>,
    fast: Option<String>,
    charged: Option<Vec<String>>,
}

pub async fn import<'a>(filename: &str, movesets: &'a HashMap<usize, Moveset<'a>>) -> Result<impl Iterator<Item=&'a Moveset<'a>> + Clone, ()> {
    let mut file = File::open(filename).await.map_err(|e| error!("Error opening filter file \"{}\": {}", filename, e))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await.map_err(|e| error!("Error reading filter file \"{}\": {}", filename, e))?;
    let input: Vec<Input> = serde_json::from_str(&contents).map_err(|e| error!("Error decoding filter file \"{}\": {}", filename, e))?;
    Ok(movesets.iter()
        .filter(move |(_, mv)| {
            input.iter().any(|i| {
                i.pokemon == mv.pokemon.unique_id &&
                    (i.form.is_none() || i.form == mv.pokemon.form) &&
                    (i.fast.is_none() || i.fast.as_ref() == Some(&mv.fast_move.unique_id)) &&
                    (i.charged.is_none() || i.charged.as_ref().map(|cms| cms.iter().any(|cm| cm == &mv.charged_move1.unique_id || cm == &mv.charged_move2.unique_id)) == Some(true))
            })
        })
        .map(|(_, mv)| mv))
}