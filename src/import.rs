use std::path::PathBuf;
use std::collections::HashMap;

use tokio::{fs::File, io::AsyncReadExt};

use serde::Deserialize;

use log::{error, info};

use crate::moveset::Moveset;

#[derive(Clone, Deserialize)]
struct Input {
    pokemon: String,
    form: Option<String>,
    fast: Option<String>,
    charged: Option<Vec<String>>,
    level: Option<u8>,
    atk: Option<u8>,
    def: Option<u8>,
    sta: Option<u8>,
}

pub async fn import<'a>(filename: &PathBuf, movesets: &'a HashMap<usize, Moveset<'a>>) -> Result<HashMap<usize, Moveset<'a>>, ()> {
    let mut file = File::open(filename).await.map_err(|e| error!("Error opening filter file \"{}\": {}", filename.display(), e))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await.map_err(|e| error!("Error reading filter file \"{}\": {}", filename.display(), e))?;
    let input: Vec<Input> = serde_json::from_str(&contents).map_err(|e| error!("Error decoding filter file \"{}\": {}", filename.display(), e))?;
    Ok(movesets.iter()
        .map(move |(index, mv)| {
            let found = input.iter().find(|i| {
                i.pokemon == mv.pokemon.unique_id &&
                    (i.form.is_none() || i.form == mv.pokemon.form) &&
                    (i.fast.is_none() || i.fast.as_ref() == Some(&mv.fast_move.unique_id)) &&
                    (i.charged.is_none() || i.charged.as_ref().map(|cms| cms.contains(&mv.charged_move1.unique_id) && cms.contains(&mv.charged_move2.unique_id)) == Some(true))
            });
            if let Some(p) = found {
                info!("Found {} {} {} {}", mv.pokemon.form.as_ref().unwrap_or(&mv.pokemon.unique_id), mv.fast_move.unique_id, mv.charged_move1.unique_id, mv.charged_move2.unique_id);
                let mut moveset = mv.clone();
                if let Some(l) = p.level {
                    moveset.level = l;
                }
                if let Some(a) = p.atk {
                    moveset.atk = a;
                }
                if let Some(d) = p.def {
                    moveset.def = d;
                }
                if let Some(s) = p.sta {
                    moveset.sta = s;
                }
                Some((*index, moveset))
            }
            else {
                None
            }
        })
        .filter(Option::is_some)
        .map(Option::unwrap)
        .collect())
}
