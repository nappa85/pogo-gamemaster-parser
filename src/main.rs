#![deny(warnings)]
#![deny(missing_docs)]

//! # pogo-gamemaster-parser
//!
//! Pokémon GO GameMaster parser

use std::collections::HashMap;

use rayon::{slice::ParallelSliceMut, iter::{IntoParallelRefIterator, ParallelIterator}};

use once_cell::sync::Lazy;

use log::error;

mod entities;

const CP_CAP: u64 = 1500;
const CP_RANGE: u64 = 150;

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
    max_level: (u64, u16, u16, u16, u16),
    fast_move: &'a entities::CombatMove,
    charged_move: &'a entities::CombatMove,
}

impl<'a> Moveset<'a>{
    fn from(p: &'a entities::PokemonSettings, combat_moves: &'a HashMap<&'a str, &'a entities::CombatMove>, player_level: &entities::PlayerLevel) -> Vec<Self> {
        let mut res = Vec::new();
        match (Self::convert_moves(&p.quick_moves, LEGACY_QUICK_MOVES.get(&p.pokemon_id), combat_moves), Self::convert_moves(&p.cinematic_moves, LEGACY_CHARGED_MOVES.get(&p.pokemon_id), combat_moves), Self::get_max_level(&p.stats, player_level)) {
            (Some(fast), Some(charged), Some(max)) => {
                for fast_move in &fast {
                    for charged_move in &charged {
                        // for max_level in &max {
                            res.push(Moveset {
                                pokemon: p,
                                max_level: max,//*max_level,
                                fast_move,
                                charged_move,
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

    // return maximum pokemon level to fit CP_CAP
    fn get_max_level(stats: &entities::Stats, player_level: &entities::PlayerLevel) -> Option<(u64, u16, u16, u16, u16)> {
        let mut res = Vec::new();
        for level in (1_u16..=41_u16).rev() {
            for sta in (0_u16..=15_u16).rev() {
                for def in (0_u16..=15_u16).rev() {
                    for atk in (0_u16..=15_u16).rev() {
                        let cp = Self::get_cp(stats, level, atk, def, sta, player_level);
                        if cp < CP_CAP && cp > CP_CAP - CP_RANGE {
                            res.push((cp, level, atk, def, sta));
                        }
                    }
                }
            }
        }
        if res.is_empty() {
            None
        }
        else {
            res.par_sort_unstable_by(|a, b| {
                a.0.cmp(&b.0)
                    .then_with(|| {
                        a.1.cmp(&b.1)
                            .then_with(|| {
                                a.2.cmp(&b.2)
                            })
                    })
            });
            res.pop()
        }
    }

    pub fn get_cp(stats: &entities::Stats, level: u16, atk: u16, def: u16, sta: u16, player_level: &entities::PlayerLevel) -> u64 {
        ((((stats.base_attack + atk) as f64) * ((stats.base_defense + def) as f64).powf(0.5) * ((stats.base_stamina + sta) as f64).powf(0.5) * player_level.cp_multiplier[(level - 1) as usize].powi(2)) / 10_f64).floor() as u64
    }

    // how many turns of fast move are needed to launch a charged move
    pub fn get_tpc(&self) -> Option<f32> {
        match (self.charged_move.energy_delta, self.fast_move.energy_delta) {
            (Some(c), Some(f)) => Some((((c * -1) as f32)  / (f as f32)).ceil() * ((self.fast_move.duration_turns.unwrap_or_else(|| 0) as f32) + 1.0)),
            _ => None,
        }
    }

    // get the total amount of damage given by the fast moves neede to lauch  a charged move, and the charged move itself, all moltiplied by base attack stat
    pub fn get_dpc(&self, player_level: &entities::PlayerLevel) -> Option<f64> {
        match (self.charged_move.energy_delta, self.fast_move.energy_delta) {
            (Some(c), Some(f)) => Some(((((c * -1) as f64)  / (f as f64)).ceil() * (self.fast_move.power.unwrap_or_else(|| 0.0) as f64) + (self.charged_move.power.unwrap_or_else(|| 0.0) as f64)) * self.get_attack(player_level)),
            _ => None,
        }
    }

    fn get_attack(&self, player_level: &entities::PlayerLevel) -> f64 {
        (((self.pokemon.stats.base_attack + self.max_level.2) as f64) * player_level.cp_multiplier[(self.max_level.1 - 1) as usize]).floor() + 1.0
    }

    // gets the median damage value per turn
    pub fn get_median(&self, player_level: &entities::PlayerLevel) -> Option<f64> {
        match (self.get_dpc(player_level), self.get_tpc()) {
            (Some(damage), Some(time)) => Some(damage / (time as f64)),
            _ => None
        }
    }
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
    // create Pokémon dicionary
    let mut movesets: Vec<Moveset> = root.item_templates.par_iter()
        .filter(|item| item.pokemon_settings.is_some())
        .map(|item| Moveset::from(item.pokemon_settings.as_ref().unwrap(), &combat_moves, &player_level))
        .flatten()
        .collect();
    // Sort by charge move quickness and then by damage
    // movesets.par_sort_unstable_by(|a, b| {
    //     a.get_tpc()
    //         .partial_cmp(&b.get_tpc())
    //         .unwrap()
    //         .reverse()
    //         .then_with(|| {
    //             a.get_dpc(player_level)
    //                 .partial_cmp(&b.get_dpc(player_level))
    //                 .unwrap()
    //         })
    // });
    // // Sort by damage and then by charge move quickness
    // movesets.par_sort_unstable_by(|a, b| {
    //     a.get_dpc(player_level)
    //         .partial_cmp(&b.get_dpc(player_level))
    //         .unwrap()
    //         .then_with(|| {
    //             a.get_tpc()
    //                 .partial_cmp(&b.get_tpc())
    //                 .unwrap()
    //                 .reverse()
    //         })
    // });
    // Sort by (damage / charge move quickness) and then by quickness
    movesets.par_sort_unstable_by(|a, b| {
        a.get_median(player_level)
            .partial_cmp(&b.get_median(player_level))
            .unwrap()
            .then_with(|| {
                a.get_tpc()
                    .partial_cmp(&b.get_tpc())
                    .unwrap()
            })
    });

    for ms in movesets {
        println!("{} ({:?}) {} {} lvl {} {} {} {} {} ({:?} DPC / {:?} TPC = {:?})", ms.pokemon.pokemon_id, ms.pokemon.form, ms.max_level.0, ms.max_level.1, ms.max_level.2, ms.max_level.3, ms.max_level.4, ms.fast_move.unique_id, ms.charged_move.unique_id, ms.get_dpc(player_level), ms.get_tpc(), ms.get_median(player_level));
    }
    Ok(())
}
