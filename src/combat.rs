use std::collections::HashMap;

use once_cell::sync::Lazy;

use log::debug;

use crate::entities::{CombatMove/*, PokemonSettings*/};
use crate::moveset::Moveset;

// EFFECTIVENESS[atk_type][def_type]
static EFFECTIVENESS: Lazy<HashMap<String, HashMap<String, f64>>> = Lazy::new(|| {
    let mut res = HashMap::new();
    let temp = res.entry(String::from("POKEMON_TYPE_BUG")).or_insert_with(HashMap::new);
    temp.insert(String::from("POKEMON_TYPE_DARK"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_FAIRY"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_FIGHTING"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_FIRE"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_FLYING"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_GHOST"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_GRASS"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_POISON"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_PSYCHIC"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_STEEL"), 0.714);
    let temp = res.entry(String::from("POKEMON_TYPE_DARK")).or_insert_with(HashMap::new);
    temp.insert(String::from("POKEMON_TYPE_DARK"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_FAIRY"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_FIGHTING"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_GHOST"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_PSYCHIC"), 1.4);
    let temp = res.entry(String::from("POKEMON_TYPE_DRAGON")).or_insert_with(HashMap::new);
    temp.insert(String::from("POKEMON_TYPE_DRAGON"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_FAIRY"), 0.51);
    temp.insert(String::from("POKEMON_TYPE_STEEL"), 0.714);
    let temp = res.entry(String::from("POKEMON_TYPE_ELECTRIC")).or_insert_with(HashMap::new);
    temp.insert(String::from("POKEMON_TYPE_DRAGON"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_ELECTRIC"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_FLYING"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_GRASS"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_GROUND"), 0.51);
    let temp = res.entry(String::from("POKEMON_TYPE_FAIRY")).or_insert_with(HashMap::new);
    temp.insert(String::from("POKEMON_TYPE_DARK"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_DRAGON"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_FIGHTING"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_FIRE"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_POISON"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_STEEL"), 0.714);
    let temp = res.entry(String::from("POKEMON_TYPE_FIGHTING")).or_insert_with(HashMap::new);
    temp.insert(String::from("POKEMON_TYPE_BUG"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_DARK"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_FAIRY"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_FLYING"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_GHOST"), 0.51);
    temp.insert(String::from("POKEMON_TYPE_ICE"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_NORMAL"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_POISON"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_PSYCHIC"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_ROCK"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_STEEL"), 0.4);
    let temp = res.entry(String::from("POKEMON_TYPE_FIRE")).or_insert_with(HashMap::new);
    temp.insert(String::from("POKEMON_TYPE_BUG"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_DRAGON"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_FIRE"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_GRASS"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_ICE"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_ROCK"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_STEEL"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_WATER"), 0.714);
    let temp = res.entry(String::from("POKEMON_TYPE_FLYING")).or_insert_with(HashMap::new);
    temp.insert(String::from("POKEMON_TYPE_BUG"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_ELECTRIC"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_FIGHTING"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_GRASS"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_ROCK"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_STEEL"), 0.714);
    let temp = res.entry(String::from("POKEMON_TYPE_GHOST")).or_insert_with(HashMap::new);
    temp.insert(String::from("POKEMON_TYPE_DARK"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_GHOST"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_NORMAL"), 0.51);
    temp.insert(String::from("POKEMON_TYPE_PSYCHIC"), 1.4);
    let temp = res.entry(String::from("POKEMON_TYPE_GRASS")).or_insert_with(HashMap::new);
    temp.insert(String::from("POKEMON_TYPE_BUG"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_DRAGON"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_FIRE"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_FLYING"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_GRASS"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_GROUND"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_POISON"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_ROCK"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_STEEL"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_WATER"), 1.4);
    let temp = res.entry(String::from("POKEMON_TYPE_GROUND")).or_insert_with(HashMap::new);
    temp.insert(String::from("POKEMON_TYPE_BUG"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_ELECTRIC"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_FIRE"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_FLYING"), 0.51);
    temp.insert(String::from("POKEMON_TYPE_GRASS"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_POISON"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_ROCK"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_STEEL"), 1.4);
    let temp = res.entry(String::from("POKEMON_TYPE_ICE")).or_insert_with(HashMap::new);
    temp.insert(String::from("POKEMON_TYPE_DRAGON"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_FIRE"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_FLYING"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_GRASS"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_GROUND"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_ICE"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_STEEL"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_WATER"), 0.714);
    let temp = res.entry(String::from("POKEMON_TYPE_NORMAL")).or_insert_with(HashMap::new);
    temp.insert(String::from("POKEMON_TYPE_GHOST"), 0.51);
    temp.insert(String::from("POKEMON_TYPE_ROCK"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_STEEL"), 0.714);
    let temp = res.entry(String::from("POKEMON_TYPE_POISON")).or_insert_with(HashMap::new);
    temp.insert(String::from("POKEMON_TYPE_FAIRY"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_GHOST"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_GRASS"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_GROUND"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_POISON"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_ROCK"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_STEEL"), 0.51);
    let temp = res.entry(String::from("POKEMON_TYPE_PSYCHIC")).or_insert_with(HashMap::new);
    temp.insert(String::from("POKEMON_TYPE_DARK"), 0.51);
    temp.insert(String::from("POKEMON_TYPE_FIGHTING"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_POISON"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_PSYCHIC"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_STEEL"), 0.714);
    let temp = res.entry(String::from("POKEMON_TYPE_ROCK")).or_insert_with(HashMap::new);
    temp.insert(String::from("POKEMON_TYPE_BUG"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_FIGHTING"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_FIRE"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_FLYING"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_GROUND"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_ICE"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_STEEL"), 0.714);
    let temp = res.entry(String::from("POKEMON_TYPE_STEEL")).or_insert_with(HashMap::new);
    temp.insert(String::from("POKEMON_TYPE_ELECTRIC"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_FAIRY"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_FIRE"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_ICE"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_ROCK"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_STEEL"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_WATER"), 0.714);
    let temp = res.entry(String::from("POKEMON_TYPE_WATER")).or_insert_with(HashMap::new);
    temp.insert(String::from("POKEMON_TYPE_DRAGON"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_FIRE"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_GRASS"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_GROUND"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_ROCK"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_WATER"), 0.714);
    res
});

fn _get_damage(m: &CombatMove, atk: &Moveset, def: &Moveset) -> i32 {
    let mut multipliers = 1.0;
    //stab
    if m.r#type == atk.pokemon.type1 || Some(&m.r#type) == atk.pokemon.type2.as_ref() {
        multipliers *= 1.2;
    }
    multipliers *= EFFECTIVENESS[&m.r#type].get(&def.pokemon.type1).cloned().unwrap_or_else(|| 1.0);
    if let Some(type2) = atk.pokemon.type2.as_ref() {
        multipliers *= EFFECTIVENESS[&m.r#type].get(type2).cloned().unwrap_or_else(|| 1.0);
    }
    ((0.5 * (m.power.unwrap_or_else(|| 0.0_f32) as f64) *
        ((((atk.pokemon.stats.base_attack as f64) + (atk.atk as f64)) * atk.cpm) / (((def.pokemon.stats.base_defense as f64) + (def.def as f64)) * def.cpm)) *
        multipliers).floor() as i32) + 1
}

enum DamageType {
    Fast(i32),
    Charged(i32),
}

fn get_damage(p1: &Moveset, energy: &mut i32, wait: &mut i32, p2: &Moveset) -> DamageType {
    let charged1_damage = _get_damage(&p1.charged_move1, p1, p2);
    let charged2_damage = _get_damage(&p1.charged_move2, p1, p2);
    if charged1_damage > charged2_damage {
        let energy_delta = p1.charged_move1.energy_delta.unwrap_or_else(|| 0) as i32;
        if *energy + energy_delta > 0 {
            *energy += energy_delta;
            return DamageType::Charged(charged1_damage);
        }
    }
    else {
        let energy_delta = p2.charged_move1.energy_delta.unwrap_or_else(|| 0) as i32;
        if *energy + energy_delta > 0 {
            *energy += energy_delta;
            return DamageType::Charged(charged2_damage);
        }
    }

    *energy += p1.fast_move.energy_delta.unwrap_or_else(|| 0) as i32;
    *wait = p1.fast_move.duration_turns.unwrap_or_else(|| 0) as i32;
    DamageType::Fast(_get_damage(&p1.fast_move, p1, p2))
}

#[derive(Debug, PartialEq)]
pub enum CombatResult {
    Draw,
    First,
    Second,
}

pub fn combat<'a>(team1: &'a [&'a Moveset<'a>], team2: &'a [&'a Moveset<'a>]) -> CombatResult {
    debug!(
        "team1 = [{}]\nteam2 = [{}]",
        team1.iter().map(|m| if let Some(form) = m.pokemon.form.as_ref() { form.as_str() } else { m.pokemon.unique_id.as_str() }).collect::<Vec<&str>>().join(", "),
        team2.iter().map(|m| if let Some(form) = m.pokemon.form.as_ref() { form.as_str() } else { m.pokemon.unique_id.as_str() }).collect::<Vec<&str>>().join(", ")
    );
    let mut team1_pokemon = 0;
    let mut team1_cp = team1[0].cp as i32;
    let mut team1_wait = 0;
    let mut team1_energy = 0;
    let mut team1_damage;
    let mut team2_pokemon = 0;
    let mut team2_cp = team2[0].cp as i32;
    let mut team2_wait = 0;
    let mut team2_energy = 0;
    let mut team2_damage;
    loop {
        if team1_wait > 0 {
            debug!("team1 waits");
            team1_wait -= 1;
            team1_damage = None;
        }
        else {
            team1_damage = Some(get_damage(&team1[team1_pokemon], &mut team1_energy, &mut team1_wait, &team2[team2_pokemon]));
        }
        if team2_wait > 0 {
            debug!("team2 waits");
            team2_wait -= 1;
            team2_damage = None;
        }
        else {
            team2_damage = Some(get_damage(&team2[team2_pokemon], &mut team2_energy, &mut team2_wait, &team1[team1_pokemon]));
        }

        match (team1_damage, team2_damage) {
            (Some(DamageType::Charged(d1)), Some(DamageType::Charged(d2))) => {
                if team1[team1_pokemon].pokemon.stats.base_attack + (team1[team1_pokemon].atk as u16) >= team2[team2_pokemon].pokemon.stats.base_attack + (team2[team2_pokemon].atk as u16) {
                    debug!("team1 has priority and deals {} damage with a charged move", d1);
                    team2_cp -= d1;
                    if team2_cp > 0 {
                        debug!("team2 survives and deals {} damage with a charged move", d2);
                        team1_cp -= d2;
                    }
                }
                else {
                    debug!("team2 has priority and deals {} damage with a charged move", d2);
                    team1_cp -= d2;
                    if team1_cp > 0 {
                        debug!("team1 survives and deals {} damage with a charged move", d1);
                        team2_cp -= d1;
                    }
                }
            },
            (Some(DamageType::Charged(d1)), Some(DamageType::Fast(d2))) => {
                debug!("team1 deals {} damage with a charged move", d1);
                team2_cp -= d1;
                if team2_cp > 0 {
                    debug!("team2 survives and deals {} damage with a fast move", d2);
                    team1_cp -= d2;
                }
            },
            (Some(DamageType::Fast(d1)), Some(DamageType::Charged(d2))) => {
                debug!("team2 deals {} damage with a charged move", d2);
                team1_cp -= d2;
                if team1_cp > 0 {
                    debug!("team1 survives and deals {} damage with a fast move", d1);
                    team2_cp -= d1;
                }
            },
            (Some(DamageType::Fast(d1)), Some(DamageType::Fast(d2))) => {
                debug!("team1 deals {} damage with a fast move", d1);
                team1_cp -= d2;
                debug!("team2 deals {} damage with a fast move", d2);
                team2_cp -= d1;
            },
            (Some(DamageType::Fast(d1)), None) => {
                debug!("team1 deals {} damage with a fast move", d1);
                team2_cp -= d1;
            },
            (Some(DamageType::Charged(d1)), None) => {
                debug!("team1 deals {} damage with a charged move", d1);
                team2_cp -= d1;
            },
            (None, Some(DamageType::Fast(d2))) => {
                debug!("team2 deals {} damage with a fast move", d2);
                team1_cp -= d2;
            },
            (None, Some(DamageType::Charged(d2))) => {
                debug!("team2 deals {} damage with a charged move", d2);
                team1_cp -= d2;
            },
            (None, None) => {},
        }

        if team1_cp <= 0 {
            debug!("team1 pokemon faints");
            team1_pokemon += 1;
            if team1_pokemon < team1.len() {
                team1_cp = team2[team1_pokemon].cp as i32;
                team1_energy = 0;
                team1_wait = 0;
            }
        }
        if team2_cp <= 0 {
            debug!("team2 pokemon faints");
            team2_pokemon += 1;
            if team2_pokemon < team2.len() {
                team2_cp = team2[team2_pokemon].cp as i32;
                team2_energy = 0;
                team2_wait = 0;
            }
        }
        if team1_pokemon >= team1.len() && team2_pokemon >= team2.len() {
            return CombatResult::Draw;
        }
        else if team1_pokemon >= team1.len() {
            return CombatResult::Second;
        }
        else if team2_pokemon >= team2.len() {
            return CombatResult::First;
        }
        debug!("next turn")
    }
}

#[cfg(test)]
mod test {
    use super::{combat, CombatResult};
    use crate::entities::{PokemonSettings, CombatMove};
    use crate::moveset::Moveset;

    #[test]
    fn draw() {
        env_logger::try_init().ok();

        let cpm = [0.094, 0.16639787, 0.21573247, 0.25572005, 0.29024988, 0.3210876, 0.34921268, 0.3752356, 0.39956728, 0.4225, 0.44310755, 0.4627984, 0.48168495, 0.49985844, 0.51739395, 0.5343543, 0.5507927, 0.5667545, 0.5822789, 0.5974, 0.6121573, 0.6265671, 0.64065295, 0.65443563, 0.667934, 0.6811649, 0.69414365, 0.7068842, 0.7193991, 0.7317, 0.7377695, 0.74378943, 0.74976104, 0.7556855, 0.76156384, 0.76739717, 0.7731865, 0.77893275, 0.784637, 0.7903, 0.7953, 0.8003, 0.8053, 0.8103, 0.8153];
        let giratina: PokemonSettings = serde_json::from_str(r#"{
            "uniqueId": "GIRATINA",
            "modelScale": 1.26,
            "type1": "POKEMON_TYPE_GHOST",
            "type2": "POKEMON_TYPE_DRAGON",
            "camera": {
              "diskRadiusM": 0.378,
              "cylinderRadiusM": 1.5,
              "cylinderHeightM": 4.0,
              "shoulderModeScale": 0.5
            },
            "encounter": {
              "baseCaptureRate": 0.02,
              "baseFleeRate": 0.04,
              "collisionRadiusM": 1.0,
              "collisionHeightM": 0.252,
              "collisionHeadRadiusM": 0.5,
              "movementType": "MOVEMENT_JUMP",
              "movementTimerS": 10.0,
              "jumpTimeS": 0.9,
              "attackTimerS": 29.0,
              "attackProbability": 0.2,
              "dodgeProbability": 0.05,
              "dodgeDurationS": 1.0,
              "dodgeDistance": 1.0,
              "cameraDistance": 6.0,
              "minPokemonActionFrequencyS": 0.2,
              "maxPokemonActionFrequencyS": 1.6
            },
            "stats": {
              "baseStamina": 284,
              "baseAttack": 187,
              "baseDefense": 225
            },
            "quickMoves": ["DRAGON_BREATH_FAST", "SHADOW_CLAW_FAST"],
            "cinematicMoves": ["DRAGON_CLAW", "ANCIENT_POWER", "SHADOW_SNEAK"],
            "animTime": [1.9, 0.6667, 1.8, 1.7667, 0.0, 2.4, 0.8667, 0.0],
            "evolutionPips": 1,
            "pokemonClass": "POKEMON_CLASS_LEGENDARY",
            "pokedexHeightM": 4.5,
            "pokedexWeightKg": 750.0,
            "heightStdDev": 0.5625,
            "weightStdDev": 93.75,
            "familyId": "FAMILY_GIRATINA",
            "candyToEvolve": 25,
            "kmBuddyDistance": 20.0,
            "buddySize": "BUDDY_BIG",
            "modelHeight": 5.38,
            "modelScaleV2": 0.63,
            "form": "GIRATINA_ALTERED",
            "buddyOffsetMale": [25.5, 0.0, 1.25],
            "buddyOffsetFemale": [25.5, 0.0, 1.25],
            "buddyScale": 19.0,
            "buddyPortraitOffset": [0.33, 0.21, 0.84],
            "thirdMove": {
              "stardustToUnlock": 100000,
              "candyToUnlock": 100
            },
            "isTransferable": true,
            "buddyGroupNumber": 3
        }"#).unwrap();
        let dragon_breath: CombatMove = serde_json::from_str(r#"{
            "uniqueId": "DRAGON_BREATH_FAST",
            "type": "POKEMON_TYPE_DRAGON",
            "power": 4.0,
            "vfxName": "dragon_breath_fast",
            "energyDelta": 3
        }"#).unwrap();
        let dragon_claw: CombatMove = serde_json::from_str(r#"{
            "uniqueId": "DRAGON_CLAW",
            "type": "POKEMON_TYPE_DRAGON",
            "power": 50.0,
            "vfxName": "dragon_claw",
            "energyDelta": -35
        }"#).unwrap();
        let ancient_power: CombatMove = serde_json::from_str(r#"{
            "uniqueId": "ANCIENT_POWER",
            "type": "POKEMON_TYPE_ROCK",
            "power": 45.0,
            "vfxName": "ancient_power",
            "energyDelta": -45,
            "buffs": {
                "attackerAttackStatStageChange": 2,
                "attackerDefenseStatStageChange": 2,
                "buffActivationChance": 0.1
            }
        }"#).unwrap();
        let m0 = Moveset {
            pokemon: &giratina,
            cp: 2495,
            level: 27,
            atk: 10,
            def: 10,
            sta: 10,
            cpm: cpm[26],
            fast_move: &dragon_breath,
            fast_legacy: None,
            charged_move1: &dragon_claw,
            charged_legacy1: None,
            charged_move2: &ancient_power,
            charged_legacy2: None,
        };
        assert_eq!(combat(&[&m0], &[&m0]), CombatResult::Draw);
    }

    #[test]
    fn giratina_vs_togekiss() {
        env_logger::try_init().ok();

        let cpm = [0.094, 0.16639787, 0.21573247, 0.25572005, 0.29024988, 0.3210876, 0.34921268, 0.3752356, 0.39956728, 0.4225, 0.44310755, 0.4627984, 0.48168495, 0.49985844, 0.51739395, 0.5343543, 0.5507927, 0.5667545, 0.5822789, 0.5974, 0.6121573, 0.6265671, 0.64065295, 0.65443563, 0.667934, 0.6811649, 0.69414365, 0.7068842, 0.7193991, 0.7317, 0.7377695, 0.74378943, 0.74976104, 0.7556855, 0.76156384, 0.76739717, 0.7731865, 0.77893275, 0.784637, 0.7903, 0.7953, 0.8003, 0.8053, 0.8103, 0.8153];
        let giratina: PokemonSettings = serde_json::from_str(r#"{
            "uniqueId": "GIRATINA",
            "modelScale": 1.26,
            "type1": "POKEMON_TYPE_GHOST",
            "type2": "POKEMON_TYPE_DRAGON",
            "camera": {
              "diskRadiusM": 0.378,
              "cylinderRadiusM": 1.5,
              "cylinderHeightM": 4.0,
              "shoulderModeScale": 0.5
            },
            "encounter": {
              "baseCaptureRate": 0.02,
              "baseFleeRate": 0.04,
              "collisionRadiusM": 1.0,
              "collisionHeightM": 0.252,
              "collisionHeadRadiusM": 0.5,
              "movementType": "MOVEMENT_JUMP",
              "movementTimerS": 10.0,
              "jumpTimeS": 0.9,
              "attackTimerS": 29.0,
              "attackProbability": 0.2,
              "dodgeProbability": 0.05,
              "dodgeDurationS": 1.0,
              "dodgeDistance": 1.0,
              "cameraDistance": 6.0,
              "minPokemonActionFrequencyS": 0.2,
              "maxPokemonActionFrequencyS": 1.6
            },
            "stats": {
              "baseStamina": 284,
              "baseAttack": 187,
              "baseDefense": 225
            },
            "quickMoves": ["DRAGON_BREATH_FAST", "SHADOW_CLAW_FAST"],
            "cinematicMoves": ["DRAGON_CLAW", "ANCIENT_POWER", "SHADOW_SNEAK"],
            "animTime": [1.9, 0.6667, 1.8, 1.7667, 0.0, 2.4, 0.8667, 0.0],
            "evolutionPips": 1,
            "pokemonClass": "POKEMON_CLASS_LEGENDARY",
            "pokedexHeightM": 4.5,
            "pokedexWeightKg": 750.0,
            "heightStdDev": 0.5625,
            "weightStdDev": 93.75,
            "familyId": "FAMILY_GIRATINA",
            "candyToEvolve": 25,
            "kmBuddyDistance": 20.0,
            "buddySize": "BUDDY_BIG",
            "modelHeight": 5.38,
            "modelScaleV2": 0.63,
            "form": "GIRATINA_ALTERED",
            "buddyOffsetMale": [25.5, 0.0, 1.25],
            "buddyOffsetFemale": [25.5, 0.0, 1.25],
            "buddyScale": 19.0,
            "buddyPortraitOffset": [0.33, 0.21, 0.84],
            "thirdMove": {
              "stardustToUnlock": 100000,
              "candyToUnlock": 100
            },
            "isTransferable": true,
            "buddyGroupNumber": 3
        }"#).unwrap();
        let togekiss: PokemonSettings = serde_json::from_str(r#"{
            "uniqueId": "TOGEKISS",
            "modelScale": 1.26,
            "type1": "POKEMON_TYPE_FAIRY",
            "type2": "POKEMON_TYPE_FLYING",
            "camera": {
              "diskRadiusM": 0.378,
              "cylinderRadiusM": 0.5,
              "cylinderHeightM": 0.9,
              "cylinderGroundM": 0.5,
              "shoulderModeScale": 0.5
            },
            "encounter": {
              "baseCaptureRate": 0.01,
              "baseFleeRate": 0.05,
              "collisionRadiusM": 0.35,
              "collisionHeightM": 0.252,
              "collisionHeadRadiusM": 0.4,
              "movementType": "MOVEMENT_FLYING",
              "movementTimerS": 10.0,
              "jumpTimeS": 0.9,
              "attackTimerS": 29.0,
              "bonusCandyCaptureReward": 7,
              "bonusStardustCaptureReward": 400,
              "attackProbability": 0.2,
              "dodgeProbability": 0.3,
              "dodgeDurationS": 1.0,
              "dodgeDistance": 1.0,
              "cameraDistance": 5.0,
              "minPokemonActionFrequencyS": 0.2,
              "maxPokemonActionFrequencyS": 1.6
            },
            "stats": {
              "baseStamina": 198,
              "baseAttack": 225,
              "baseDefense": 217
            },
            "quickMoves": ["AIR_SLASH_FAST", "HIDDEN_POWER_FAST", "CHARM_FAST"],
            "cinematicMoves": ["ANCIENT_POWER", "DAZZLING_GLEAM", "AERIAL_ACE", "FLAMETHROWER"],
            "animTime": [1.9, 0.6667, 1.8, 1.7667, 0.0, 2.4, 0.8667, 0.0],
            "evolutionPips": 1,
            "pokedexHeightM": 1.5,
            "pokedexWeightKg": 38.0,
            "parentId": "TOGETIC",
            "heightStdDev": 0.1875,
            "weightStdDev": 4.75,
            "familyId": "FAMILY_TOGEPI",
            "candyToEvolve": 25,
            "kmBuddyDistance": 3.0,
            "modelHeight": 1.25,
            "modelScaleV2": 1.0,
            "buddyOffsetMale": [-0.69, 0.0, -66.52],
            "buddyOffsetFemale": [-0.69, 0.0, -66.52],
            "buddyScale": 19.0,
            "thirdMove": {
              "stardustToUnlock": 50000,
              "candyToUnlock": 50
            },
            "isTransferable": true,
            "isDeployable": true,
            "buddyGroupNumber": 2
        }"#).unwrap();
        let dragon_breath: CombatMove = serde_json::from_str(r#"{
            "uniqueId": "DRAGON_BREATH_FAST",
            "type": "POKEMON_TYPE_DRAGON",
            "power": 4.0,
            "vfxName": "dragon_breath_fast",
            "energyDelta": 3
        }"#).unwrap();
        let charm: CombatMove = serde_json::from_str(r#"{
            "uniqueId": "CHARM_FAST",
            "type": "POKEMON_TYPE_FAIRY",
            "power": 16.0,
            "vfxName": "charm_fast",
            "durationTurns": 2,
            "energyDelta": 6
        }"#).unwrap();
        let dragon_claw: CombatMove = serde_json::from_str(r#"{
            "uniqueId": "DRAGON_CLAW",
            "type": "POKEMON_TYPE_DRAGON",
            "power": 50.0,
            "vfxName": "dragon_claw",
            "energyDelta": -35
        }"#).unwrap();
        let ancient_power: CombatMove = serde_json::from_str(r#"{
            "uniqueId": "ANCIENT_POWER",
            "type": "POKEMON_TYPE_ROCK",
            "power": 45.0,
            "vfxName": "ancient_power",
            "energyDelta": -45,
            "buffs": {
                "attackerAttackStatStageChange": 2,
                "attackerDefenseStatStageChange": 2,
                "buffActivationChance": 0.1
            }
        }"#).unwrap();
        let aerial_ace: CombatMove = serde_json::from_str(r#"{
            "uniqueId": "AERIAL_ACE",
            "type": "POKEMON_TYPE_FLYING",
            "power": 55.0,
            "vfxName": "aerial_ace",
            "energyDelta": -45
        }"#).unwrap();
        let m0 = Moveset {
            pokemon: &giratina,
            cp: 2495,
            level: 27,
            atk: 10,
            def: 10,
            sta: 10,
            cpm: cpm[26],
            fast_move: &dragon_breath,
            fast_legacy: None,
            charged_move1: &dragon_claw,
            charged_legacy1: None,
            charged_move2: &ancient_power,
            charged_legacy2: None,
        };
        let m1 = Moveset {
            pokemon: &togekiss,
            cp: 2499,
            level: 28,
            atk: 0,
            def: 15,
            sta: 15,
            cpm: cpm[27],
            fast_move: &charm,
            fast_legacy: None,
            charged_move1: &aerial_ace,
            charged_legacy1: None,
            charged_move2: &ancient_power,
            charged_legacy2: None,
        };
        assert_eq!(combat(&[&m0], &[&m1]), CombatResult::Second);
    }
}
