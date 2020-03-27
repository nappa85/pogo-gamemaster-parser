#![deny(warnings)]
#![deny(missing_docs)]

//! # pogo-gamemaster-parser
//!
//! Pokémon GO GameMaster parser

use std::path::Path;

mod entities;
mod export;
mod import;

// const MEGA_LEAGUE: RangeInclusive<u32> = 1400..=1500;
// const ULTRA_LEAGUE: RangeInclusive<u32> = 2400..=2500;

// static TARGET: Lazy<String> = Lazy::new(|| env::var("TARGET").expect("Missing TARGET env var"));

#[tokio::main]
async fn main() -> Result<(), ()> {
    env_logger::init();

    let folder = Path::new("./matches/");
    export::exec(&folder).await?;
    import::exec(&folder).await?;

    // let mut res: Vec<(String, usize)> = matches.par_bridge()
    //     .fold_with(HashMap::new(), |mut dict, (team1, team2)| {
    //         let entry = match combat(&team1, &team2) {
    //             CombatResult::Team1 => dict.entry(Moveset::team(&team1)).or_insert_with(|| 0),
    //             CombatResult::Team2 => dict.entry(Moveset::team(&team2)).or_insert_with(|| 0),
    //             _ => return dict,
    //         };
    //         *entry += 1;
    //         dict
    //     })
    //     .reduce(HashMap::new, |mut a, b| {
    //         for (key, value) in b.into_iter() {
    //             let temp = a.entry(key).or_insert_with(|| 0);
    //             *temp += value;
    //         }
    //         a
    //     })
    //     .into_iter()
    //     .collect();

    // res.par_sort_unstable_by(|(_, a), (_, b)| a.cmp(b));

    // println!("{:#?}", res);

    // // very aggressive
    // iter(movesets).for_each_concurrent(Some(10), |mvs| async move {
    //     if mvs.is_empty() {
    //         return;
    //     }

    //     let movesets = mvs.into_iter().map(|mv| json!({
    //         "pokemon_id": mv.pokemon.unique_id.as_str(),
    //         "pokemon_type1": mv.pokemon.type1.replacen("POKEMON_TYPE_", "", 1),
    //         "pokemon_type2": mv.pokemon.type2.as_ref().map(|s| s.replacen("POKEMON_TYPE_", "", 1)),
    //         "base_atk": mv.pokemon.stats.base_attack,
    //         "base_def": mv.pokemon.stats.base_defense,
    //         "base_sta": mv.pokemon.stats.base_stamina,
    //         "form": mv.pokemon.form.as_ref().map(|s| s.replacen(&format!("{}_", mv.pokemon.unique_id), "", 1)),
    //         "cp": mv.cp,
    //         "level": mv.level,
    //         "atk": mv.atk,
    //         "def": mv.def,
    //         "sta": mv.sta,
    //         "cpm": mv.cpm,
    //         "fast_move": mv.fast_move.unique_id.as_str(),
    //         "fast_type": mv.fast_move.r#type.replacen("POKEMON_TYPE_", "", 1),
    //         "fast_power": mv.fast_move.power,
    //         "fast_energy": mv.fast_move.energy_delta,
    //         "fast_duration": mv.fast_move.duration_turns,
    //         "fast_legacy": mv.fast_legacy,
    //         "charged_move": mv.charged_move.unique_id.as_str(),
    //         "charged_type": mv.charged_move.r#type.replacen("POKEMON_TYPE_", "", 1),
    //         "charged_power": mv.charged_move.power,
    //         "charged_energy": mv.charged_move.energy_delta,
    //         "charged_legacy": mv.charged_legacy,
    //         // "tpc": mv.tpc,
    //         // "dpc": mv.dpc,
    //     })).collect::<Vec<Value>>();

    //     let client = Client::new();// not good
    //     loop {
    //         if let Ok(res) = client.post(TARGET.as_str())
    //             .json(&movesets)
    //             .send()
    //             .await
    //             .map_err(|e| error!("Transmission error: {}", e)) {
    //             if res.status().is_success() {
    //                 info!("Inserted {} combinations: {:?}", movesets.len(), res.text().await);
    //                 break;
    //             }
    //             else {
    //                 error!("Creation error: {:?}", res.text().await);
    //             }
    //         }
    //     }
    // }).await;

    Ok(())
}
