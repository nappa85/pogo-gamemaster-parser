use std::ops::RangeInclusive;
use std::collections::HashMap;

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

    println!("{} movesets", movesets.len());

    let teams = movesets.iter()
        .par_bridge()
        .map_with(&movesets, |ms, m1| ms.iter().par_bridge().map(move |m2| (m1, m2)))
        .flatten()
        .map_with(&movesets, |ms, (m1, m2)| ms.iter().par_bridge().map(move |m3| (m1, m2, m3)))
        .flatten()
        .filter(|(t0, t1, t2)| {
            // unique pokemon
            t0.1.pokemon.unique_id != t1.1.pokemon.unique_id &&
                t0.1.pokemon.unique_id != t2.1.pokemon.unique_id &&
                t1.1.pokemon.unique_id != t2.1.pokemon.unique_id &&
            // additional filter: unique types
                t0.1.pokemon.type1 != t1.1.pokemon.type1 &&
                Some(&t0.1.pokemon.type1) != t1.1.pokemon.type2.as_ref() &&
                t0.1.pokemon.type1 != t2.1.pokemon.type1 &&
                Some(&t0.1.pokemon.type1) != t2.1.pokemon.type2.as_ref() &&
                t1.1.pokemon.type1 != t2.1.pokemon.type1 &&
                Some(&t1.1.pokemon.type1) != t2.1.pokemon.type2.as_ref()
        });

    let matches = teams.clone().map_with(teams, |ts, t1| ts.clone().map(move |t2| (t1, t2)))
        .flatten()
        .fold(HashMap::new, |mut dict, ((t0_0, t0_1, t0_2), (t1_0, t1_1, t1_2))| {
            let entry = match combat(&[t0_0.1, t0_1.1, t0_2.1], &[t1_0.1, t1_1.1, t1_2.1]) {
                CombatResult::First => dict.entry((*t0_0.0, *t0_1.0, *t0_2.0)).or_insert_with(|| 0),
                CombatResult::Second => dict.entry((*t1_0.0, *t1_1.0, *t1_2.0)).or_insert_with(|| 0),
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

    let mut results: Vec<((usize, usize, usize), usize)> = matches.into_iter().collect();

    results.par_sort_unstable_by(|(_, a), (_, b)| a.cmp(b));

    for ((m0, m1, m2), score) in &results {
        println!("{} => {{\n {} {} {} {} {}\n {} {} {} {} {}\n {} {} {} {} {}\n}}",
            score,
            movesets[m0].pokemon.unique_id,
            movesets[m0].pokemon.form.as_ref().map(|s| s.as_str()).unwrap_or_else(|| ""),
            movesets[m0].fast_move.unique_id,
            movesets[m0].charged_move1.unique_id,
            movesets[m0].charged_move2.unique_id,
            movesets[m1].pokemon.unique_id,
            movesets[m1].pokemon.form.as_ref().map(|s| s.as_str()).unwrap_or_else(|| ""),
            movesets[m1].fast_move.unique_id,
            movesets[m1].charged_move1.unique_id,
            movesets[m1].charged_move2.unique_id,
            movesets[m2].pokemon.unique_id,
            movesets[m2].pokemon.form.as_ref().map(|s| s.as_str()).unwrap_or_else(|| ""),
            movesets[m2].fast_move.unique_id,
            movesets[m2].charged_move1.unique_id,
            movesets[m2].charged_move2.unique_id
        );
    }

    Ok(())
}
