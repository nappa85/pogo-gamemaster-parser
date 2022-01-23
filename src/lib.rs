use std::path::PathBuf;
use std::str::FromStr;
use std::ops::RangeInclusive;
use std::collections::HashMap;

use rayon::{iter::{ParallelBridge, ParallelIterator}, slice::ParallelSliceMut};

use itertools::Itertools;

use once_cell::sync::OnceCell;

use pogo_gamemaster_entities::{Root, PokemonSettings, CombatMove, PlayerLevel, CombatStatStageSettings};

use log::error;

use structopt::StructOpt;

mod import;
mod moveset;
mod combat;

use moveset::Moveset;
use combat::{combat, CombatResult};
use import::import;

pub static PLAYER_LEVEL: OnceCell<PlayerLevel> = OnceCell::new();
pub static COMBAT_STAT_STAGE_SETTINGS: OnceCell<CombatStatStageSettings> = OnceCell::new();

#[derive(Debug, StructOpt)]
pub enum League {
    Mega,
    Ultra,
    Master,
}

impl FromStr for League {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let temp = s.to_lowercase();
        match temp.as_str() {
            "mega" => Ok(League::Mega),
            "ultra" => Ok(League::Ultra),
            "master" => Ok(League::Master),
            _ => Err(format!("{} is not a valid league", s))
        }
    }
}

impl League {
    fn to_range(&self) -> RangeInclusive<u32> {
        match self {
            League::Mega => 1400..=1500,
            League::Ultra => 2400..=2500,
            League::Master => 2500..=9999,
        }
    }
}

// for testing purposes
fn set_player_level(value: PlayerLevel) {
    PLAYER_LEVEL.get_or_init(move || value);
}

// for testing purposes
fn set_combat_stat_stage_settings(value: CombatStatStageSettings) {
    COMBAT_STAT_STAGE_SETTINGS.get_or_init(move || value);
}

pub async fn exec(league: &League, team1: Option<&PathBuf>, team2: Option<&PathBuf>) -> Result<(), ()> {
    println!("Loading game master...");

    // load game master
    let root: Root = reqwest::get("https://raw.githubusercontent.com/pokemongo-dev-contrib/pokemongo-game-master/master/versions/latest/V2_GAME_MASTER.json")
        .await
        .map_err(|e| error!("Game Master retrieve error: {}", e))?
        .json()
        .await
        .map_err(|e| error!("Game Master decode error: {}", e))?;

    // load CPM
    set_player_level(root.template.iter()
        .filter_map(|item| item.data.player_level.clone())
        .next()
        .unwrap());

    //load Buffs multipliers
    set_combat_stat_stage_settings(root.template.iter()
        .filter_map(|item| item.data.combat_stat_stage_settings.clone())
        .next()
        .unwrap());

    // create PVP moves dictionary
    let combat_moves: HashMap<&str, &CombatMove> = root.template.iter()
        .filter(|item| item.data.combat_move.is_some())
        .map(|item| {
            let combat_move = item.data.combat_move.as_ref().unwrap();
            (combat_move.unique_id.as_str(), combat_move)
        })
        .collect();

    // create Pokémon-Form dictionary
    let pokemon: HashMap<&str, HashMap<Option<&str>, &PokemonSettings>> = root.template.iter()
        .filter(|item| item.data.pokemon.is_some())
        .fold(HashMap::new(), |mut dict, item| {
            let pokemon = item.data.pokemon.as_ref().unwrap();
            let sub_dict = dict.entry(pokemon.unique_id.as_str()).or_insert_with(HashMap::new);
            sub_dict.insert(pokemon.form.as_ref().map(|s| s.as_str()), pokemon);
            dict
        });

    println!("Loaded {} Pokémon", pokemon.len());

    // create Pokémon-Moveset dictionary
    let movesets: HashMap<usize, Moveset> = pokemon.into_iter()
        // try to cleanup duplication given by forms
        .map(|(_, forms)| {
            let base_form = forms.get(&None).map(|bf| (bf.type1.clone(), bf.type2.clone(), bf.stats.clone()));

            forms.into_iter()
                .filter(move |(_, p)| {
                    if p.form.is_some() {
                        if let Some((bf_type1, bf_type2, bf_stats)) = base_form.as_ref() {
                            return &p.type1 != bf_type1 || &p.type2 != bf_type2 || &p.stats != bf_stats;
                        }
                    }
                    true
                })
                .map(|(_, p)| p)
        })
        .flatten()
        .map(|p| Moveset::from(p, &combat_moves, Some(league.to_range())))
        .flatten()
        .enumerate()
        .collect();

    println!("Loaded {} Pokémon-Moveset combinations", movesets.len());

    let filter1 = if let Some(filename) = team1 {
        import(filename, &movesets).await?
    }
    else {
        movesets.clone()
    };
    println!("Team1 is made of {} Pokémon-Moveset combinations", filter1.len());
    // let teams1 = movesets.iter()
    //     .par_bridge()
    //     .map_with(&movesets, |ms, m1| ms.iter().par_bridge().map(move |m2| (m1, m2)))
    //     .flatten()
    //     .map_with(&movesets, |ms, (m1, m2)| ms.iter().par_bridge().map(move |m3| (m1, m2, m3)))
    //     .flatten()
    //     .filter(|(t0, t1, t2)| {
    //         // unique pokemon
    //         t0.1.pokemon.unique_id != t1.1.pokemon.unique_id &&
    //             t0.1.pokemon.unique_id != t2.1.pokemon.unique_id &&
    //             t1.1.pokemon.unique_id != t2.1.pokemon.unique_id &&
    //         // additional filter: unique types
    //             t0.1.pokemon.type1 != t1.1.pokemon.type1 &&
    //             Some(&t0.1.pokemon.type1) != t1.1.pokemon.type2.as_ref() &&
    //             t0.1.pokemon.type1 != t2.1.pokemon.type1 &&
    //             Some(&t0.1.pokemon.type1) != t2.1.pokemon.type2.as_ref() &&
    //             t1.1.pokemon.type1 != t2.1.pokemon.type1 &&
    //             Some(&t1.1.pokemon.type1) != t2.1.pokemon.type2.as_ref()
    //     });
    let teams1 = filter1.iter().par_bridge()
        .map_with(&filter1, |ms, m1| ms.iter().combinations(2).par_bridge().map(move |m2| (m1, m2[0], m2[1])))
        .flatten()
        .filter(|(t0, t1, t2)| {
            // unique pokemon
            t0.1.pokemon.unique_id != t1.1.pokemon.unique_id &&
                t0.1.pokemon.unique_id != t2.1.pokemon.unique_id &&
                t1.1.pokemon.unique_id != t2.1.pokemon.unique_id
        });

    let filter2 = if let Some(filename) = team2 {
        import(filename, &movesets).await?
    }
    else {
        movesets.clone()
    };
    println!("Team2 is made of {} Pokémon-Moveset combinations", filter2.len());
    let teams2 = filter2.iter().par_bridge()
        .map_with(&filter2, |ms, m1| ms.iter().combinations(2).par_bridge().map(move |m2| (m1, m2[0], m2[1])))
        .flatten()
        .filter(|(t0, t1, t2)| {
            // unique pokemon
            t0.1.pokemon.unique_id != t1.1.pokemon.unique_id &&
                t0.1.pokemon.unique_id != t2.1.pokemon.unique_id &&
                t1.1.pokemon.unique_id != t2.1.pokemon.unique_id
        });

    let matches = teams1.map_with(teams2, |ts, t1| ts.clone().map(move |t2| (t1, t2)))
        .flatten()
        .fold(HashMap::new, |mut dict, ((t0_0, t0_1, t0_2), (t1_0, t1_1, t1_2))| {
            let entry = match combat(&[t0_0.1, t0_1.1, t0_2.1], &[t1_0.1, t1_1.1, t1_2.1]) {
                CombatResult::First => dict.entry(*t0_0.0).or_insert_with(HashMap::new).entry(*t0_1.0).or_insert_with(HashMap::new).entry(*t0_2.0).or_insert_with(|| 0),
                // CombatResult::Second => dict.entry(*t1_0.0).or_insert_with(HashMap::new).entry(*t1_1.0).or_insert_with(HashMap::new).entry(*t1_2.0).or_insert_with(|| 0),
                _ => return dict,
            };
            *entry += 1;
            dict
        })
        .map(|fold| {
            let mut res = HashMap::new();
            for (index, value) in fold.into_iter().map(|(m0, hm)| hm.into_iter().map(move |(m1, hm)| hm.into_iter().map(move |(m2, v)| ((m0, m1, m2), v))).flatten()).flatten() {
                // let's put a barrier of minumun 10% won matches
                // if value > filter1.len() / 10 {
                    let entry = res.entry(index).or_insert_with(|| 0_usize);
                    *entry += value;
                // }
            }
            res
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
        println!("{} => {{\n {} {} {} {}\n {} {} {} {}\n {} {} {} {}\n}}",
            score,
            movesets[m0].pokemon.form.as_ref().unwrap_or(&movesets[m0].pokemon.unique_id),
            movesets[m0].fast_move.unique_id,
            movesets[m0].charged_move1.unique_id,
            movesets[m0].charged_move2.unique_id,
            movesets[m1].pokemon.form.as_ref().unwrap_or(&movesets[m1].pokemon.unique_id),
            movesets[m1].fast_move.unique_id,
            movesets[m1].charged_move1.unique_id,
            movesets[m1].charged_move2.unique_id,
            movesets[m2].pokemon.form.as_ref().unwrap_or(&movesets[m2].pokemon.unique_id),
            movesets[m2].fast_move.unique_id,
            movesets[m2].charged_move1.unique_id,
            movesets[m2].charged_move2.unique_id
        );
    }

    Ok(())
}
