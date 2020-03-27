use std::ops::RangeInclusive;
use std::collections::HashMap;

use itertools::Itertools;

use rayon::{iter::{ParallelBridge, ParallelIterator}, slice::ParallelSliceMut};

use log::error;

use crate::entities::{Root, PokemonSettings, CombatMove};
use crate::moveset::Moveset;
use crate::combat::{combat, CombatResult};

// const MEGA_LEAGUE: RangeInclusive<u32> = 1400..=1500;
const ULTRA_LEAGUE: RangeInclusive<u32> = 2400..=2500;

pub async fn exec() -> Result<(), ()> {
    // load game master
    let root: Root = reqwest::get("https://raw.githubusercontent.com/pokemongo-dev-contrib/pokemongo-game-master/master/versions/latest/GAME_MASTER.json")
        .await
        .map_err(|e| error!("Game Master retrieve error: {}", e))?
        .json()
        .await
        .map_err(|e| error!("Game Master decode error: {}", e))?;

    // load CPM
    let player_level = root.item_template.iter()
        .find(|item| item.player_level.is_some())
        .map(|item| item.player_level.as_ref().unwrap())
        .unwrap();

    // create PVP moves dictionary
    let combat_moves: HashMap<&str, &CombatMove> = root.item_template.iter()
        .filter(|item| item.combat_move.is_some())
        .map(|item| {
            let combat_move = item.combat_move.as_ref().unwrap();
            (combat_move.unique_id.as_str(), combat_move)
        })
        .collect();

    // create Pokémon-Form dictionary
    let pokemon: HashMap<&str, HashMap<Option<&str>, &PokemonSettings>> = root.item_template.iter()
        .filter(|item| item.pokemon.is_some())
        .fold(HashMap::new(), |mut dict, item| {
            let pokemon = item.pokemon.as_ref().unwrap();
            let sub_dict = dict.entry(pokemon.unique_id.as_str()).or_insert_with(HashMap::new);
            sub_dict.insert(pokemon.form.as_ref().map(|s| s.as_str()), pokemon);
            dict
        });

    // create Pokémon-Moveset dictionary
    let movesets: HashMap<usize, Moveset> = pokemon.into_iter()
        // try to cleanup duplication given by forms
        .map(|(_, forms)| {
            let base_form = forms.get(&None);
            let base_stats = base_form.map(|p| p.stats);
            let base_type = base_form.map(|p| (p.type1.clone(), p.type2.clone()));

            forms.into_iter()
                .filter(move |(_, p)| {
                    p.form.is_none() ||
                    Some((&p.type1, p.type2.as_ref())) != base_type.as_ref().map(|(t1, t2)| (t1, t2.as_ref())) ||
                    Some(&p.stats) != base_stats.as_ref()
                })
                .map(|(_, p)| p)
        })
        .flatten()
        .map(|p| Moveset::from(p, &combat_moves, Some(&ULTRA_LEAGUE), &player_level))
        .flatten()
        .enumerate()
        .collect();

    // let matches = movesets.permutations(3)
    //     // you can't have the same pokemon in the team, despite the form
    //     .filter(|m| {
    //         // m[0].pokemon.unique_id != m[1].pokemon.unique_id &&
    //         //     m[0].pokemon.unique_id != m[2].pokemon.unique_id &&
    //         //     m[1].pokemon.unique_id != m[2].pokemon.unique_id
    //         m[0].unique_id != m[1].unique_id &&
    //             m[0].unique_id != m[2].unique_id &&
    //             m[1].unique_id != m[2].unique_id
    //     })
    //     // create every possible match
    //     .combinations(2)
    //     .enumerate();

    // create_dir(folder).await.map_err(|e| error!("Can't create matches folder: {}", e))?;
    // iter(matches).for_each_concurrent(Some(10), |(index, pvp)| async move {
    //     let mut filename = folder.to_path_buf();
    //     filename.push(index.to_string());
    //     if let Ok(mut file) = File::create(&filename).await.map_err(|e| error!("Can't create file {}: {}", filename.display(), e)) {
    //         if let Ok(contents) = bincode::serialize(&pvp).map_err(|e| error!("Can't convert match to json: {}", e)) {
    //             file.write_all(&contents).await.map_err(|e| error!("Can't write file {}: {}", filename.display(), e)).ok();
    //         }
    //     }
    // }).await;

    // info!("Folder created!");
    let matches = movesets.iter()
        .combinations(2)
        .par_bridge()
        .fold(HashMap::new, |mut dict, teams| {
            let entry = match combat(&[&teams[0].1], &[&teams[1].1]) {
                CombatResult::First => dict.entry(*teams[0].0).or_insert_with(|| 0),
                CombatResult::Second => dict.entry(*teams[1].0).or_insert_with(|| 0),
                _ => return dict,
            };
            *entry += 1;
            dict
        })
        .reduce(HashMap::new, |mut dict, fold| {
            for (index, value) in fold.into_iter() {
                let entry = dict.entry(index).or_insert_with(|| 0);
                *entry += value;
            }
            dict
        });

    let mut results: Vec<(Moveset, usize)> = movesets.into_iter()
        .map(|(index, moveset)| {
            (moveset, matches.get(&index).cloned().unwrap_or_else(|| 0))
        })
        .collect();

    results.par_sort_unstable_by(|(_, a), (_, b)| a.cmp(b));

    for result in &results {
        println!("{} => {} {} {} {} {}",
            result.1,
            result.0.pokemon.unique_id,
            result.0.pokemon.form.as_ref().map(|s| s.as_str()).unwrap_or_else(|| ""),
            result.0.fast_move.unique_id,
            result.0.charged_move1.unique_id,
            result.0.charged_move2.unique_id
        );
    }

    Ok(())
}
