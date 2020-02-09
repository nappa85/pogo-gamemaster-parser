#![deny(warnings)]
#![deny(missing_docs)]

//! # pogo-gamemaster-parser
//!
//! Pokémon GO GameMaster parser

use std::env;
use std::collections::HashMap;
use std::ops::RangeInclusive;

use reqwest::Client;

use futures_util::stream::{iter, StreamExt};

use once_cell::sync::Lazy;

use serde_json::{json, value::Value};

use log::{error, info};

mod entities;

const CP_CAP: RangeInclusive<u32> = 1400..=1500;

static TARGET: Lazy<String> = Lazy::new(|| env::var("TARGET").expect("Missing TARGET env var"));

static LEGACY_QUICK_MOVES: Lazy<HashMap<String, Vec<&'static str>>> = Lazy::new(|| {
    let mut res = HashMap::new();
    res.insert(String::from("GRIMER"), vec!["ACID_FAST"]);
    res.insert(String::from("KOFFING"), vec!["ACID_FAST"]);
    res.insert(String::from("WEEZING"), vec!["ACID_FAST"]);
    res.insert(String::from("MUK"), vec!["ACID_FAST", "LICK_FAST"]);
    res.insert(String::from("ARCANINE"), vec!["BITE_FAST"]);
    res.insert(String::from("SHEDINJA"), vec!["BITE_FAST", "STRUGGLE_BUG_FAST"]);
    res.insert(String::from("BUTTERFREE"), vec!["BUG_BITE_FAST"]);
    res.insert(String::from("BEEDRILL"), vec!["BUG_BITE_FAST"]);
    res.insert(String::from("PARASECT"), vec!["BUG_BITE_FAST"]);
    res.insert(String::from("VENOMOTH"), vec!["BUG_BITE_FAST"]);
    res.insert(String::from("FARFETCHD"), vec!["CUT_FAST"]);
    res.insert(String::from("GYARADOS"), vec!["DRAGON_TAIL_FAST"]);
    res.insert(String::from("NINETALES"), vec!["EMBER_FAST"]);
    res.insert(String::from("RAPIDASH"), vec!["EMBER_FAST"]);
    res.insert(String::from("MOLTRES"), vec!["EMBER_FAST"]);
    res.insert(String::from("CHARIZARD"), vec!["EMBER_FAST", "WING_ATTACK_FAST"]);
    res.insert(String::from("SMOOCHUM"), vec!["FROST_BREATH_FAST"]);
    res.insert(String::from("NIDOKING"), vec!["FURY_CUTTER_FAST"]);
    res.insert(String::from("KABUTOPS"), vec!["FURY_CUTTER_FAST"]);
    res.insert(String::from("SUICUNE"), vec!["HIDDEN_POWER_FAST"]);
    res.insert(String::from("DEWGONG"), vec!["ICE_SHARD_FAST"]);
    res.insert(String::from("LAPRAS"), vec!["ICE_SHARD_FAST"]);
    res.insert(String::from("DELIBIRD"), vec!["ICE_SHARD_FAST", "QUICK_ATTACK_FAST"]);
    res.insert(String::from("PRIMEAPE"), vec!["KARATE_CHOP_FAST"]);
    res.insert(String::from("MACHAMP"), vec!["KARATE_CHOP_FAST"]);
    res.insert(String::from("HAUNTER"), vec!["LICK_FAST"]);
    res.insert(String::from("GENGAR"), vec!["LICK_FAST"]);
    res.insert(String::from("MACHOP"), vec!["LOW_KICK_FAST"]);
    res.insert(String::from("DIGLETT"), vec!["MUD_SHOT_FAST"]);
    res.insert(String::from("DUGTRIO"), vec!["MUD_SHOT_FAST"]);
    res.insert(String::from("GRAVELER"), vec!["MUD_SHOT_FAST"]);
    res.insert(String::from("GOLEM"), vec!["MUD_SHOT_FAST"]);
    res.insert(String::from("KINGLER"), vec!["MUD_SHOT_FAST"]);
    res.insert(String::from("SEAKING"), vec!["POISON_JAB_FAST"]);
    res.insert(String::from("CLEFABLE"), vec!["POUND_FAST"]);
    res.insert(String::from("JYNX"), vec!["POUND_FAST"]);
    res.insert(String::from("PIKACHU"), vec!["PRESENT_FAST"]);
    res.insert(String::from("STARYU"), vec!["QUICK_ATTACK_FAST"]);
    res.insert(String::from("PICHU"), vec!["QUICK_ATTACK_FAST"]);
    res.insert(String::from("STARMIE"), vec!["QUICK_ATTACK_FAST", "TACKLE_FAST"]);
    res.insert(String::from("PORYGON"), vec!["QUICK_ATTACK_FAST", "TACKLE_FAST", "ZEN_HEADBUTT_FAST"]);
    res.insert(String::from("WEEPINBELL"), vec!["RAZOR_LEAF_FAST"]);
    res.insert(String::from("HITMONCHAN"), vec!["ROCK_SMASH_FAST"]);
    res.insert(String::from("OMASTAR"), vec!["ROCK_THROW_FAST"]);
    res.insert(String::from("CHARMELEON"), vec!["SCRATCH_FAST"]);
    res.insert(String::from("TYRANITAR"), vec!["SMACK_DOWN_FAST"]);
    res.insert(String::from("SCYTHER"), vec!["STEEL_WING_FAST"]);
    res.insert(String::from("TOGETIC"), vec!["STEEL_WING_FAST", "ZEN_HEADBUTT_FAST"]);
    res.insert(String::from("GASTLY"), vec!["SUCKER_PUNCH_FAST"]);
    res.insert(String::from("ELECTRODE"), vec!["TACKLE_FAST"]);
    res.insert(String::from("ZAPDOS"), vec!["THUNDER_SHOCK_FAST"]);
    res.insert(String::from("SEEL"), vec!["WATER_GUN_FAST"]);
    res.insert(String::from("FERALIGATR"), vec!["WATER_GUN_FAST"]);
    res.insert(String::from("KINGDRA"), vec!["WATER_GUN_FAST"]);
    res.insert(String::from("PIDGEOT"), vec!["WING_ATTACK_FAST"]);
    res.insert(String::from("SNORLAX"), vec!["YAWN_FAST"]);
    res.insert(String::from("EXEGGUTOR"), vec!["ZEN_HEADBUTT_FAST"]);
    res.insert(String::from("TOGEPI"), vec!["ZEN_HEADBUTT_FAST"]);
    res
});

static LEGACY_CHARGED_MOVES: Lazy<HashMap<String, Vec<&'static str>>> = Lazy::new(|| {
    let mut res = HashMap::new();
    res.insert(String::from("BULBASAUR"), vec!["RETURN"]);
    res.insert(String::from("IVYSAUR"), vec!["RETURN"]);
    res.insert(String::from("VENUSAUR"), vec!["FRENZY_PLANT", "RETURN"]);
    res.insert(String::from("CHARMANDER"), vec!["RETURN"]);
    res.insert(String::from("SQUIRTLE"), vec!["RETURN"]);
    res.insert(String::from("WARTORTLE"), vec!["RETURN"]);
    res.insert(String::from("BLASTOISE"), vec!["HYDRO_CANNON", "RETURN"]);
    res.insert(String::from("RATTATA"), vec!["RETURN"]);
    res.insert(String::from("RATICATE"), vec!["RETURN"]);
    res.insert(String::from("SPEAROW"), vec!["TWISTER"]);
    res.insert(String::from("FEAROW"), vec!["TWISTER"]);
    res.insert(String::from("EKANS"), vec!["GUNK_SHOT"]);
    res.insert(String::from("RAICHU"), vec!["THUNDER"]);
    res.insert(String::from("SANDSHREW"), vec!["ROCK_TOMB"]);
    res.insert(String::from("JIGGLYPUFF"), vec!["PLAY_ROUGH", "BODY_SLAM"]);
    res.insert(String::from("ZUBAT"), vec!["SLUDGE_BOMB", "RETURN"]);
    res.insert(String::from("GOLBAT"), vec!["OMINOUS_WIND", "RETURN"]);
    res.insert(String::from("ODDISH"), vec!["RETURN"]);
    res.insert(String::from("GLOOM"), vec!["RETURN"]);
    res.insert(String::from("VILEPLUME"), vec!["RETURN"]);
    res.insert(String::from("MEOWTH"), vec!["BODY_SLAM"]);
    res.insert(String::from("PERSIAN"), vec!["NIGHT_SLASH"]);
    res.insert(String::from("POLIWAG"), vec!["RETURN"]);
    res.insert(String::from("POLIWAG"), vec!["SCALD", "RETURN"]);
    res.insert(String::from("POLIWRATH"), vec!["SUBMISSION", "RETURN"]);
    res.insert(String::from("ALAKAZAM"), vec!["DAZZLING_GLEAM", "PSYCHIC"]);
    res.insert(String::from("MACHOKE"), vec!["CROSS_CHOP"]);
    res.insert(String::from("PONYTA"), vec!["FIRE_BLAST"]);
    res.insert(String::from("DODUO"), vec!["SWIFT"]);
    res.insert(String::from("DODRIO"), vec!["AIR_CUTTER"]);
    res.insert(String::from("CLOYSTER"), vec!["BLIZZARD"]);
    res.insert(String::from("ONIX"), vec!["IRON_HEAD", "ROCK_SLIDE"]);
    res.insert(String::from("DROWZEE"), vec!["RETURN"]);
    res.insert(String::from("HYPNO"), vec!["PSYSHOCK", "RETURN"]);
    res.insert(String::from("VOLTORB"), vec!["SIGNAL_BEAM"]);
    res.insert(String::from("CUBONE"), vec!["RETURN"]);
    res.insert(String::from("MAROWAK"), vec!["RETURN"]);
    res.insert(String::from("HITMONLEE"), vec!["STOMP", "BRICK_BREAK"]);
    res.insert(String::from("RHYDON"), vec!["MEGAHORN"]);
    res.insert(String::from("CHANSEY"), vec!["PSYBEAM"]);
    res.insert(String::from("TANGELA"), vec!["POWER_WHIP"]);
    res.insert(String::from("KANGASKHAN"), vec!["BRICK_BREAK", "STOMP"]);
    res.insert(String::from("SEADRA"), vec!["BLIZZARD"]);
    res.insert(String::from("PINSIR"), vec!["SUBMISSION"]);
    res.insert(String::from("MAGIKARP"), vec!["RETURN"]);
    res.insert(String::from("EEVEE"), vec!["BODY_SLAM", "LAST_RESORT"]);
    res.insert(String::from("VAPOREON"), vec!["LAST_RESORT"]);
    res.insert(String::from("JOLTEON"), vec!["LAST_RESORT"]);
    res.insert(String::from("JOLTEON"), vec!["HEAT_WAVE", "LAST_RESORT"]);
    res.insert(String::from("OMANYTE"), vec!["ROCK_TOMB", "BRINE"]);
    res.insert(String::from("ARTICUNO"), vec!["HURRICANE", "RETURN"]);
    res.insert(String::from("DRATINI"), vec!["RETURN"]);
    res.insert(String::from("DRAGONAIR"), vec!["RETURN"]);
    res.insert(String::from("DRAGONAIR"), vec!["DRAGON_PULSE", "DRACO_METEOR", "RETURN"]);
    res.insert(String::from("MEWTWO"), vec!["SHADOW_BALL", "HYPER_BEAM", "PSYSTRIKE"]);
    res.insert(String::from("MEGANIUM"), vec!["FRENZY_PLANT"]);
    res.insert(String::from("TYPHLOSION"), vec!["BLAST_BURN"]);
    res.insert(String::from("CROBAT"), vec!["RETURN"]);
    res.insert(String::from("CLEFFA"), vec!["PSYCHIC", "BODY_SLAM"]);
    res.insert(String::from("IGGLYBUFF"), vec!["BODY_SLAM"]);
    res.insert(String::from("AMPHAROS"), vec!["DRAGON_PULSE"]);
    res.insert(String::from("BELLOSSOM"), vec!["RETURN"]);
    res.insert(String::from("POLITOED"), vec!["EARTHQUAKE", "RETURN"]);
    res.insert(String::from("ESPEON"), vec!["LAST_RESORT"]);
    res.insert(String::from("UMBREON"), vec!["LAST_RESORT"]);
    res.insert(String::from("HOUNDOUR"), vec!["RETURN"]);
    res.insert(String::from("HOUNDOOM"), vec!["RETURN"]);
    res.insert(String::from("ELEKID"), vec!["THUNDERBOLT"]);
    res.insert(String::from("MAGBY"), vec!["FLAMETHROWER"]);
    res.insert(String::from("SCEPTILE"), vec!["FRENZY_PLANT"]);
    res.insert(String::from("BLAZIKEN"), vec!["STONE_EDGE", "BLAST_BURN"]);
    res.insert(String::from("SWAMPERT"), vec!["HYDRO_CANNON"]);
    res.insert(String::from("RALTS"), vec!["RETURN"]);
    res.insert(String::from("KIRLIA"), vec!["RETURN"]);
    res.insert(String::from("GARDEVOIR"), vec!["SYNCHRONOISE", "RETURN"]);
    res.insert(String::from("BRELOOM"), vec!["GRASS_KNOT"]);
    res.insert(String::from("SLAKING"), vec!["BODY_SLAM"]);
    res.insert(String::from("LOUDRED"), vec!["CRUNCH"]);
    res.insert(String::from("FLYGON"), vec!["EARTH_POWER"]);
    res.insert(String::from("SALAMENCE"), vec!["OUTRAGE"]);
    res.insert(String::from("METAGROSS"), vec!["METEOR_MASH"]);
    res.insert(String::from("INFERNAPE"), vec!["BLAST_BURN"]);
    res.insert(String::from("EMPOLEON"), vec!["HYDRO_CANNON"]);
    res.insert(String::from("TORTERRA"), vec!["RETURN", "FRENZY_PLANT"]);
    res.insert(String::from("LEAFEON"), vec!["LAST_RESORT"]);
    res.insert(String::from("GLACEON"), vec!["LAST_RESORT"]);
    res.insert(String::from("MAMOSWINE"), vec!["ANCIENT_POWER"]);
    res.insert(String::from("GALLADE"), vec!["SYNCHRONOISE", "RETURN"]);
    res.insert(String::from("GASTRODON"), vec!["EARTHQUAKE"]);
    res.insert(String::from("GRIMER"), vec!["RETURN"]);
    res.insert(String::from("MUK"), vec!["RETURN"]);
    res.insert(String::from("ARCANINE"), vec!["BULLDOZE", "RETURN"]);
    res.insert(String::from("GYARADOS"), vec!["DRAGON_PULSE", "RETURN"]);
    res.insert(String::from("NINETALES"), vec!["FIRE_BLAST", "FLAMETHROWER"]);
    res.insert(String::from("MOLTRES"), vec!["SKY_ATTACK"]);
    res.insert(String::from("CHARIZARD"), vec!["FLAMETHROWER", "BLAST_BURN", "RETURN"]);
    res.insert(String::from("DEWGONG"), vec!["AQUA_JET", "ICY_WIND"]);
    res.insert(String::from("LAPRAS"), vec!["DRAGON_PULSE", "ICE_BEAM"]);
    res.insert(String::from("PRIMEAPE"), vec!["CROSS_CHOP"]);
    res.insert(String::from("MACHAMP"), vec!["STONE_EDGE", "SUBMISSION"]);
    res.insert(String::from("GENGAR"), vec!["SLUDGE_WAVE", "DARK_PULSE", "PSYCHIC"]);
    res.insert(String::from("GRAVELER"), vec!["ROCK_SLIDE"]);
    res.insert(String::from("GOLEM"), vec!["ANCIENT_POWER"]);
    res.insert(String::from("SEAKING"), vec!["ICY_WIND", "DRILL_RUN"]);
    res.insert(String::from("JYNX"), vec!["ICE_PUNCH"]);
    res.insert(String::from("PIKACHU"), vec!["SURF", "THUNDER"]);
    res.insert(String::from("STARMIE"), vec!["PSYBEAM"]);
    res.insert(String::from("PORYGON"), vec!["DISCHARGE", "PSYBEAM", "SIGNAL_BEAM"]);
    res.insert(String::from("HITMONCHAN"), vec!["BRICK_BREAK"]);
    res.insert(String::from("OMASTAR"), vec!["ROCK_SLIDE"]);
    res.insert(String::from("CHARMELEON"), vec!["RETURN"]);
    res.insert(String::from("SCYTHER"), vec!["BUG_BUZZ", "RETURN"]);
    res.insert(String::from("GASTLY"), vec!["OMINOUS_WIND"]);
    res.insert(String::from("ZAPDOS"), vec!["DISCHARGE", "RETURN"]);
    res.insert(String::from("SEEL"), vec!["AQUA_JET"]);
    res.insert(String::from("FERALIGATR"), vec!["HYDRO_CANNON"]);
    res.insert(String::from("PIDGEOT"), vec!["AIR_CUTTER"]);
    res.insert(String::from("SNORLAX"), vec!["RETURN"]);
    res
});

// intermediate struct
#[derive(Debug)]
struct Moveset<'a> {
    pokemon: &'a entities::PokemonSettings,
    cp: u32,
    level: u8,
    // atk: u8,
    // def: u8,
    // sta: u8,
    cpm: f64,
    fast_move: &'a entities::CombatMove,
    fast_legacy: Option<bool>,
    charged_move: &'a entities::CombatMove,
    charged_legacy: Option<bool>,
    tpc: Option<u8>,
    dpc: Option<f64>,
}

impl<'a> Moveset<'a>{
    fn from(p: &'a entities::PokemonSettings, combat_moves: &'a HashMap<&'a str, &'a entities::CombatMove>, player_level: &entities::PlayerLevel) -> Vec<Self> {
        let mut res = Vec::new();
        match (Self::convert_moves(&p.quick_moves, LEGACY_QUICK_MOVES.get(&p.pokemon_id), combat_moves), Self::convert_moves(&p.cinematic_moves, LEGACY_CHARGED_MOVES.get(&p.pokemon_id), combat_moves), Self::get_max_level(&p.stats, player_level)) {
            (Some(fast), Some(charged), Some((cp, level))) => {
                for fast_move in &fast {
                    for charged_move in &charged {
                        // for (cp, level, atk, def, sta) in &max {
                            res.push(Moveset {
                                pokemon: p,
                                cp,
                                level,
                                // atk: *atk,
                                // def: *def,
                                // sta: *sta,
                                cpm: player_level.cp_multiplier[(level - 1) as usize],
                                fast_move,
                                fast_legacy: p.quick_moves.as_ref().map(|moves| !moves.contains(&fast_move.unique_id)),
                                charged_move,
                                charged_legacy: p.cinematic_moves.as_ref().map(|moves| !moves.contains(&charged_move.unique_id)),
                                tpc: Self::get_tpc(fast_move, charged_move),
                                dpc: Self::get_dpc(p, fast_move, charged_move/*, p, *level, *atk, player_level*/),
                            });
                        // }
                    }
                }
            },
            _ => {},
        }
        res
    }

    fn convert_moves(m: &'a Option<Vec<String>>, legacy_moves: Option<&'a Vec<&'a str>>, combat_moves: &'a HashMap<&'a str, &'a entities::CombatMove>) -> Option<Vec<&'a entities::CombatMove>> {
        let mut res = Vec::new();
        if let Some(moves) = m.as_ref() {
            for name in moves {
                res.push(combat_moves[&name.as_str()]);
            }
        }
        if let Some(moves) = legacy_moves {
            for name in moves {
                res.push(combat_moves[name]);
            }
        }
        if res.is_empty() {
            None
        }
        else {
            Some(res)
        }
    }

    // return maximum pokemon level for every IV combination to fit CP_CAP
    fn get_max_level(stats: &entities::Stats, player_level: &entities::PlayerLevel) -> Option<(u32, u8)> {
        // let mut res = Vec::new();
        for level in (1_u8..=41_u8).rev() {
            // for sta in (0_u8..=15_u8).rev() {
                // for def in (0_u8..=15_u8).rev() {
                    // for atk in (0_u8..=15_u8).rev() {
                        let cp = Self::get_cp(stats, level, 15, 15, 15, player_level);
                        if CP_CAP.contains(&cp) {
                            // // store only max level for every IV combination
                            // if res.iter().find(|(_, _, s_atk, s_def, s_sta)| s_atk == &atk && s_def == &def && s_sta == &sta).is_none() {
                            //     res.push((cp, level, atk, def, sta));
                            // }
                            return Some((cp, level));
                        }
                    // }
                // }
            // }
        }
        // if res.is_empty() {
        //     None
        // }
        // else {
        //     Some(res)
        // }
        None
    }

    fn get_cp(stats: &entities::Stats, level: u8, atk: u8, def: u8, sta: u8, player_level: &entities::PlayerLevel) -> u32 {
        ((((stats.base_attack + (atk as u16)) as f64) * ((stats.base_defense + (def as u16)) as f64).powf(0.5) * ((stats.base_stamina + (sta as u16)) as f64).powf(0.5) * player_level.cp_multiplier[(level - 1) as usize].powi(2)) / 10_f64).floor() as u32
    }

    // how many turns of fast move are needed to launch a charged move
    fn get_tpc(fast_move: &entities::CombatMove, charged_move: &entities::CombatMove) -> Option<u8> {
        match (charged_move.energy_delta, fast_move.energy_delta) {
            (Some(c), Some(f)) => Some(((((c * -1) as f64)  / (f as f64)).ceil() as u8) * (fast_move.duration_turns.unwrap_or_else(|| 0) + 1)),
            _ => None,
        }
    }

    // get the total amount of damage given by the fast moves neede to lauch  a charged move, and the charged move itself, all moltiplied by base attack stat
    fn get_dpc(pokemon: &entities::PokemonSettings, fast_move: &entities::CombatMove, charged_move: &entities::CombatMove/*, pokemon: &entities::PokemonSettings, level: u8, atk: u8, player_level: &entities::PlayerLevel*/) -> Option<f64> {
        match (charged_move.energy_delta, fast_move.energy_delta) {
            (Some(c), Some(f)) => Some((((c * -1) as f64)  / (f as f64)).ceil() * (fast_move.power.unwrap_or_else(|| 0.0) as f64) * (if pokemon.r#type == fast_move.r#type || pokemon.type2.as_ref() == Some(&fast_move.r#type) { 1.2 } else { 1.0 }) + (charged_move.power.unwrap_or_else(|| 0.0) as f64) * (if pokemon.r#type == charged_move.r#type || pokemon.type2.as_ref() == Some(&charged_move.r#type) { 1.2 } else { 1.0 })),// * Self::get_attack(pokemon, level, atk, player_level)),
            _ => None,
        }
    }

    // fn get_attack(pokemon: &entities::PokemonSettings, level: u8, atk: u8, player_level: &entities::PlayerLevel) -> f64 {
    //     (((pokemon.stats.base_attack + (atk as u16)) as f64) * player_level.cp_multiplier[(level - 1) as usize]).floor() + 1.0
    // }
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    env_logger::init();

    // load game master
    let root: entities::Root = reqwest::get("https://raw.githubusercontent.com/pokemongo-dev-contrib/pokemongo-game-master/master/versions/latest/GAME_MASTER.json")
        .await
        .map_err(|e| error!("Game Master retrieve error: {}", e))?
        .json()
        .await
        .map_err(|e| error!("Game Master decode error: {}", e))?;

    // load CPM
    let player_level = root.item_templates.iter()
        .find(|item| item.player_level.is_some())
        .map(|item| item.player_level.as_ref().unwrap())
        .unwrap();

    // create PVP moves dictionary
    let combat_moves: HashMap<&str, &entities::CombatMove> = root.item_templates.iter()
        .filter(|item| item.combat_move.is_some())
        .map(|item| {
            let combat_move = item.combat_move.as_ref().unwrap();
            (combat_move.unique_id.as_str(), combat_move)
        })
        .collect();

    // create Pokémon-Form dicionary
    let mut pokemon: Vec<&entities::PokemonSettings> = root.item_templates.iter()
        .filter(|item| item.pokemon_settings.is_some())
        .map(|item| item.pokemon_settings.as_ref().unwrap())
        .collect();
    // try to cleanup duplication given by forms
    pokemon.sort_unstable_by(|a, b| a.pokemon_id.cmp(&b.pokemon_id).then_with(|| a.stats.cmp(&b.stats)));
    pokemon.dedup_by(|a, b| a.pokemon_id == b.pokemon_id && a.r#type == b.r#type && a.type2 == b.type2 && a.stats == b.stats);
    // pokemon.iter().for_each(|p| println!("{} {:?} {} {:?} {:?}", p.pokemon_id, p.form, p.r#type, p.type2, p.stats));

    // create Pokémon-Moveset dicionary
    let movesets = pokemon.iter()
        .map(|p| Moveset::from(p, &combat_moves, &player_level));
        // .flatten()
        // .collect();

    info!("Here we go!");

    // movesets.sort_unstable_by(|a, b| a.tpc.cmp(&b.tpc).reverse().then_with(|| (a.dpc.unwrap_or_else(|| 0.0) * (a.pokemon.stats.base_attack as f64)).partial_cmp(&(b.dpc.unwrap_or_else(|| 0.0) * (b.pokemon.stats.base_attack as f64))).unwrap()));
    // movesets.iter().for_each(|mv| println!("{} {:?} {}{} {}{} {:?} {:?}", mv.pokemon.pokemon_id, mv.pokemon.form, mv.fast_move.unique_id, if mv.fast_legacy == Some(true) { "!" } else { "" }, mv.charged_move.unique_id, if mv.charged_legacy == Some(true) { "!" } else { "" }, mv.tpc, mv.dpc.as_ref().map(|dpc| dpc * &(mv.pokemon.stats.base_attack as f64))));

    // very aggressive
    iter(movesets).for_each_concurrent(Some(10), |mut mvs| async move {
        let client = Client::new();// not good
        'outer: loop {
            if mvs.is_empty() {
                break;
            }

            let movesets = mvs.drain(0..1000.min(mvs.len())).map(|mv| json!({
                "pokemon_id": mv.pokemon.pokemon_id.as_str(),
                "pokemon_type1": mv.pokemon.r#type.as_str(),
                "pokemon_type2": mv.pokemon.type2.as_ref().map(|s| s.as_str()),
                "base_atk": mv.pokemon.stats.base_attack,
                "base_def": mv.pokemon.stats.base_defense,
                "base_sta": mv.pokemon.stats.base_stamina,
                "form": mv.pokemon.form.as_ref().map(|s| s.as_str()),
                "cp": mv.cp,
                "level": mv.level,
                // "atk": mv.atk,
                // "def": mv.def,
                // "sta": mv.sta,
                "cpm": mv.cpm,
                "fast_move": mv.fast_move.unique_id.as_str(),
                "fast_type": mv.fast_move.r#type.as_str(),
                "fast_legacy": mv.fast_legacy,
                "charged_move": mv.charged_move.unique_id.as_str(),
                "charged_type": mv.charged_move.r#type.as_str(),
                "charged_legacy": mv.charged_legacy,
                "tpc": mv.tpc,
                "dpc": mv.dpc,
            })).collect::<Vec<Value>>();

            loop {
                if let Ok(res) = client.post(TARGET.as_str())
                    .json(&movesets)
                    .send()
                    .await
                    .map_err(|e| error!("Transmission error: {}", e)) {
                    if res.status().is_success() {
                        info!("Inserted {} combinations: {:?}", movesets.len(), res.text().await);
                        continue 'outer;
                    }
                    else {
                        error!("Creation error: {:?}", res.text().await);
                    }
                }
            }
        }
    }).await;

    Ok(())
}
