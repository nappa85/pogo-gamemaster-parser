use std::ops::RangeInclusive;
use std::collections::HashMap;

use itertools::Itertools;

use once_cell::sync::Lazy;

use pogo_gamemaster_entities::{PokemonSettings, CombatMove, Stats};

use crate::PLAYER_LEVEL;

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
    res.insert(String::from("DRAGONITE"), vec!["DRAGON_PULSE", "DRACO_METEOR", "RETURN"]);
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
    res.insert(String::from("RHYPERIOR"), vec!["ROCK_WRECKER"]);
    res
});

// intermediate struct
#[derive(Debug, Clone)]
pub struct Moveset<'a> {
    pub pokemon: &'a PokemonSettings,
    // pub cp: u32,
    // pub hp: u32,
    pub level: u8,
    pub atk: u8,
    pub def: u8,
    pub sta: u8,
    // pub cpm: f64,
    pub fast_move: &'a CombatMove,
    pub fast_legacy: Option<bool>,
    pub charged_move1: &'a CombatMove,
    pub charged_legacy1: Option<bool>,
    pub charged_move2: &'a CombatMove,
    pub charged_legacy2: Option<bool>,
}

impl<'a> Moveset<'a> {
    pub fn from(p: &'a PokemonSettings, combat_moves: &'a HashMap<&'a str, &'a CombatMove>, max_cp: Option<RangeInclusive<u32>>) -> Vec<Self> {
        let mut res = Vec::new();
        match (Self::convert_moves(&p.quick_moves, LEGACY_QUICK_MOVES.get(&p.unique_id), combat_moves), Self::convert_moves(&p.cinematic_moves, LEGACY_CHARGED_MOVES.get(&p.unique_id), combat_moves)) {
            (Some(fast), Some(charged)) => {
                if let Some((_cp, level, atk, def, sta)) = Self::get_max_level(&p.stats, p.pokemon_class.is_some(), max_cp) {
                    for charged_moves in charged.into_iter().combinations(2) {
                        for fast_move in &fast {
                            res.push(Moveset {
                                pokemon: p,
                                // cp: cp,
                                // hp: (((p.stats.base_stamina as f64) + (sta as f64)) * player_level.cp_multiplier[(level - 1) as usize]).floor() as u32,
                                level: level,
                                atk: atk,
                                def: def,
                                sta: sta,
                                // cpm: player_level.cp_multiplier[(level - 1) as usize],
                                fast_move,
                                fast_legacy: p.quick_moves.as_ref().map(|moves| !moves.contains(&fast_move.unique_id)),
                                charged_move1: charged_moves[0],
                                charged_legacy1: p.cinematic_moves.as_ref().map(|moves| !moves.contains(&charged_moves[0].unique_id)),
                                charged_move2: charged_moves[1],
                                charged_legacy2: p.cinematic_moves.as_ref().map(|moves| !moves.contains(&charged_moves[1].unique_id)),
                            });
                        }
                    }
                }
            },
            _ => {},
        }
        res
    }

    fn convert_moves(m: &'a Option<Vec<String>>, legacy_moves: Option<&'a Vec<&'a str>>, combat_moves: &'a HashMap<&'a str, &'a CombatMove>) -> Option<Vec<&'a CombatMove>> {
        let mut res = Vec::new();
        if let Some(moves) = m.as_ref() {
            for name in moves {
                let temp = combat_moves[&name.as_str()];
                if temp.power.is_some() {
                    res.push(temp);
                }
            }
        }
        if let Some(moves) = legacy_moves {
            for name in moves {
                let temp = combat_moves[name];
                if temp.power.is_some() {
                    res.push(temp);
                }
            }
        }
        if res.is_empty() {
            None
        }
        else {
            Some(res)
        }
    }

    // return maximum pokemon level for every IV combination to fit various CP caps
    fn get_max_level(stats: &Stats, level15: bool, max_cp: Option<RangeInclusive<u32>>) -> Option<(u32, u8, u8, u8, u8)> {
        let mut res = Vec::new();
        for level in ((if level15 { 15_u8 } else { 1_u8 })..=41_u8).rev() {
            for sta in (0_u8..=15_u8).rev() {
                for def in (0_u8..=15_u8).rev() {
                    for atk in (0_u8..=15_u8).rev() {
                        let cp = Self::get_cp(stats, level, atk, def, sta);
                        if let Some(ref mcp) = max_cp {
                            if !mcp.contains(&cp) {
                                continue;
                            }
                        }
                        res.push((cp, level, atk, def, sta));
                    }
                }
            }
        }
        // sort by CP ASC, then by level ASC, finally take last element
        res.sort_unstable_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));

        res.pop()
    }

    fn get_cp(stats: &Stats, level: u8, atk: u8, def: u8, sta: u8) -> u32 {
        let player_level = PLAYER_LEVEL.get().unwrap();
        ((((stats.base_attack + (atk as u16)) as f64) * ((stats.base_defense + (def as u16)) as f64).powf(0.5) * ((stats.base_stamina + (sta as u16)) as f64).powf(0.5) * player_level.cp_multiplier[(level - 1) as usize].powi(2)) / 10_f64).floor() as u32
    }

    pub fn get_cpm(&self) -> f64 {
        let player_level = PLAYER_LEVEL.get().unwrap();
        player_level.cp_multiplier[(self.level - 1) as usize]
    }

    pub fn get_hp(&self) -> u32 {
        let player_level = PLAYER_LEVEL.get().unwrap();
        (((self.pokemon.stats.base_stamina as f64) + (self.sta as f64)) * player_level.cp_multiplier[(self.level - 1) as usize]).floor() as u32
    }
}
