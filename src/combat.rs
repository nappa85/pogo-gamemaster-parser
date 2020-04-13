use std::cmp::max;
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
    Change(usize),
    Fast(i32),
    Charged(i32),
}

#[derive(Debug, PartialEq)]
pub enum CombatResult {
    Draw,
    First,
    Second,
}

#[derive(Default)]
struct Team<'a> {
    movesets: &'a [&'a Moveset<'a>],
    pokemon: usize,
    hp: Vec<i32>,
    wait: i32,
    energy: i32,
    cooldown: i32,
    fainted: bool,
}

impl<'a> Team<'a> {
    fn get_damage(&mut self, p2: &Moveset) -> DamageType {
        if self.cooldown <= 0 {
            if let Some(p) = self.get_best_pokemon_against(p2) {
                if self.pokemon != p {
                    return DamageType::Change(p);
                }
            }
        }
        else {
            self.cooldown -= 1;
        }

        let charged1_damage = _get_damage(&self.movesets[self.pokemon].charged_move1, &self.movesets[self.pokemon], p2);
        let charged2_damage = _get_damage(&self.movesets[self.pokemon].charged_move2, &self.movesets[self.pokemon], p2);
        if charged1_damage > charged2_damage {
            let energy_delta = self.movesets[self.pokemon].charged_move1.energy_delta.unwrap_or_else(|| 0) as i32;
            if self.energy + energy_delta > 0 {
                self.energy += energy_delta;
                return DamageType::Charged(charged1_damage);
            }
        }
        else {
            let energy_delta = p2.charged_move1.energy_delta.unwrap_or_else(|| 0) as i32;
            if self.energy + energy_delta > 0 {
                self.energy += energy_delta;
                return DamageType::Charged(charged2_damage);
            }
        }

        self.energy += self.movesets[self.pokemon].fast_move.energy_delta.unwrap_or_else(|| 0) as i32;
        self.wait = self.movesets[self.pokemon].fast_move.duration_turns.unwrap_or_else(|| 0) as i32;
        DamageType::Fast(_get_damage(&self.movesets[self.pokemon].fast_move, &self.movesets[self.pokemon], p2))
    }

    fn get_best_pokemon_against(&self, p2: &Moveset) -> Option<usize> {
        let mut damages: Vec<(usize, f64)> = self.movesets.iter().enumerate().map(|(index, mv)| {
            if self.hp[index] > 0 {
                // p1
                let fast = _get_damage(&mv.fast_move, mv, p2);
                let charged1_turns = ((mv.charged_move1.energy_delta.unwrap_or_else(|| 0) * -1) / mv.fast_move.energy_delta.unwrap_or_else(|| 0)) as i32;
                let charged1 = (fast * charged1_turns + _get_damage(&mv.charged_move1, mv, p2)) / charged1_turns;
                let charged2_turns = ((mv.charged_move2.energy_delta.unwrap_or_else(|| 0) * -1) / mv.fast_move.energy_delta.unwrap_or_else(|| 0)) as i32;
                let charged2 = (fast * charged2_turns + _get_damage(&mv.charged_move1, mv, p2)) / charged2_turns;
                let p1_damage = max(charged1, charged2);
                // p2
                let fast = _get_damage(&p2.fast_move, p2, mv);
                let charged1_turns = ((p2.charged_move1.energy_delta.unwrap_or_else(|| 0) * -1) / p2.fast_move.energy_delta.unwrap_or_else(|| 0)) as i32;
                let charged1 = (fast * charged1_turns + _get_damage(&p2.charged_move1, p2, mv)) / charged1_turns;
                let charged2_turns = ((p2.charged_move2.energy_delta.unwrap_or_else(|| 0) * -1) / p2.fast_move.energy_delta.unwrap_or_else(|| 0)) as i32;
                let charged2 = (fast * charged2_turns + _get_damage(&p2.charged_move1, p2, mv)) / charged2_turns;
                let p2_damage = max(charged1, charged2);
                Some((index, (p1_damage as f64) / (p2_damage as f64)))
            }
            else {
                None
            }
        })
        .filter(Option::is_some)
        .map(Option::unwrap)
        .collect();

        if damages.is_empty() {
            None
        }
        else {
            damages.sort_unstable_by(|(_, d1), (_, d2)| d1.partial_cmp(d2).unwrap());
            damages.pop().map(|(index, _)| index)
        }
    }
}

pub fn combat<'a>(team1: &'a [&'a Moveset<'a>], team2: &'a [&'a Moveset<'a>]) -> CombatResult {
    debug!(
        "\nteam1 = [{}]\nteam2 = [{}]",
        team1.iter().map(|m| if let Some(form) = m.pokemon.form.as_ref() { form.as_str() } else { m.pokemon.unique_id.as_str() }).collect::<Vec<&str>>().join(", "),
        team2.iter().map(|m| if let Some(form) = m.pokemon.form.as_ref() { form.as_str() } else { m.pokemon.unique_id.as_str() }).collect::<Vec<&str>>().join(", ")
    );
    let mut team1 = Team {
        movesets: team1,
        hp: team1.iter().map(|mv| (((mv.pokemon.stats.base_stamina as f64) + (mv.sta as f64)) * mv.cpm).floor() as i32).collect(),
        ..Default::default()
    };
    let mut team1_damage;
    let mut team2 = Team {
        movesets: team2,
        hp: team2.iter().map(|mv| (((mv.pokemon.stats.base_stamina as f64) + (mv.sta as f64)) * mv.cpm).floor() as i32).collect(),
        ..Default::default()
    };
    let mut team2_damage;
    loop {
        if team1.wait > 0 {
            debug!("team1 waits");
            team1.wait -= 1;
            team1_damage = None;
        }
        else {
            team1_damage = Some(team1.get_damage(&team2.movesets[team2.pokemon]));
        }
        if team2.wait > 0 {
            debug!("team2 waits");
            team2.wait -= 1;
            team2_damage = None;
        }
        else {
            team2_damage = Some(team2.get_damage(&team1.movesets[team1.pokemon]));
        }

        match (team1_damage, team2_damage) {
            (Some(DamageType::Charged(d1)), Some(DamageType::Charged(d2))) => {
                if team1.movesets[team1.pokemon].pokemon.stats.base_attack + (team1.movesets[team1.pokemon].atk as u16) >= team2.movesets[team2.pokemon].pokemon.stats.base_attack + (team2.movesets[team2.pokemon].atk as u16) {
                    debug!("team1 has priority and deals {} damage with a charged move", d1);
                    team2.hp[team2.pokemon] -= d1;
                    if team2.hp[team2.pokemon] > 0 {
                        debug!("team2 survives and deals {} damage with a charged move", d2);
                        team1.hp[team1.pokemon] -= d2;
                    }
                }
                else {
                    debug!("team2 has priority and deals {} damage with a charged move", d2);
                    team1.hp[team1.pokemon] -= d2;
                    if team1.hp[team1.pokemon] > 0 {
                        debug!("team1 survives and deals {} damage with a charged move", d1);
                        team2.hp[team2.pokemon] -= d1;
                    }
                }
            },
            (Some(DamageType::Charged(d1)), Some(DamageType::Fast(d2))) => {
                debug!("team1 deals {} damage with a charged move", d1);
                team2.hp[team2.pokemon] -= d1;
                if team2.hp[team2.pokemon] > 0 {
                    debug!("team2 survives and deals {} damage with a fast move", d2);
                    team1.hp[team1.pokemon] -= d2;
                }
            },
            (Some(DamageType::Fast(d1)), Some(DamageType::Charged(d2))) => {
                debug!("team2 deals {} damage with a charged move", d2);
                team1.hp[team1.pokemon] -= d2;
                if team1.hp[team1.pokemon] > 0 {
                    debug!("team1 survives and deals {} damage with a fast move", d1);
                    team2.hp[team2.pokemon] -= d1;
                }
            },
            (Some(DamageType::Fast(d1)), Some(DamageType::Fast(d2))) => {
                debug!("team1 deals {} damage with a fast move", d1);
                team1.hp[team1.pokemon] -= d2;
                debug!("team2 deals {} damage with a fast move", d2);
                team2.hp[team2.pokemon] -= d1;
            },
            (Some(DamageType::Fast(d1)), Some(DamageType::Change(p2))) => {
                debug!("team1 deals {} damage with a fast move", d1);
                team2.hp[team2.pokemon] -= d1;
                debug!("team2 switch to {}", team2.movesets[p2].pokemon.unique_id);
                team2.pokemon = p2;
                team2.cooldown = 120;
            },
            (Some(DamageType::Fast(d1)), None) => {
                debug!("team1 deals {} damage with a fast move", d1);
                team2.hp[team2.pokemon] -= d1;
            },
            (Some(DamageType::Charged(d1)), Some(DamageType::Change(p2))) => {
                debug!("team1 deals {} damage with a charged move", d1);
                team2.hp[team2.pokemon] -= d1;
                debug!("team2 switch to {}", team2.movesets[p2].pokemon.unique_id);
                team2.pokemon = p2;
                team2.cooldown = 120;
            },
            (Some(DamageType::Charged(d1)), None) => {
                debug!("team1 deals {} damage with a charged move", d1);
                team2.hp[team2.pokemon] -= d1;
            },
            (Some(DamageType::Change(p1)), None) => {
                debug!("team1 switch to {}", team1.movesets[p1].pokemon.unique_id);
                team1.pokemon = p1;
            },
            (Some(DamageType::Change(p1)), Some(DamageType::Fast(d2))) => {
                debug!("team2 deals {} damage with a fast move", d2);
                team1.hp[team1.pokemon] -= d2;
                debug!("team1 switch to {}", team1.movesets[p1].pokemon.unique_id);
                team1.pokemon = p1;
                team2.cooldown = 120;
            },
            (None, Some(DamageType::Fast(d2))) => {
                debug!("team2 deals {} damage with a fast move", d2);
                team1.hp[team1.pokemon] -= d2;
            },
            (Some(DamageType::Change(p1)), Some(DamageType::Charged(d2))) => {
                debug!("team2 deals {} damage with a charged move", d2);
                team1.hp[team1.pokemon] -= d2;
                debug!("team1 switch to {}", team1.movesets[p1].pokemon.unique_id);
                team1.pokemon = p1;
                team1.cooldown = 120;
            },
            (None, Some(DamageType::Charged(d2))) => {
                debug!("team2 deals {} damage with a charged move", d2);
                team1.hp[team1.pokemon] -= d2;
            },
            (None, Some(DamageType::Change(p2))) => {
                debug!("team2 switch to {}", team2.movesets[p2].pokemon.unique_id);
                team2.pokemon = p2;
                team2.cooldown = 120;
            },
            (Some(DamageType::Change(p1)), Some(DamageType::Change(p2))) => {
                debug!("team1 switch to {}", team1.movesets[p1].pokemon.unique_id);
                team1.pokemon = p1;
                team1.cooldown = 120;
                debug!("team2 switch to {}", team2.movesets[p2].pokemon.unique_id);
                team2.pokemon = p2;
                team2.cooldown = 120;
            },
            (None, None) => {},
        }

        if team1.hp[team1.pokemon] <= 0 {
            debug!("team1 pokemon faints");
            if let Some(p) = team1.get_best_pokemon_against(&team2.movesets[team2.pokemon]) {
                debug!("team1 switch to {}", team1.movesets[p].pokemon.unique_id);
                team1.pokemon = p;
                team1.energy = 0;
                team1.wait = 0;
                team1.cooldown = 0;
            }
            else {
                team1.fainted = true;
            }
        }
        if team2.hp[team2.pokemon] <= 0 {
            debug!("team2 pokemon faints");
            // here team2 has the advantage to coose the best pokemon against the new pokemon in case both fainted at the same time
            // in reality would be a random choice, because you can't know which pokemon the other player will choose
            // so it's kind of ok
            if let Some(p) = team2.get_best_pokemon_against(&team1.movesets[team1.pokemon]) {
                debug!("team2 switch to {}", team2.movesets[p].pokemon.unique_id);
                team2.pokemon = p;
                team2.energy = 0;
                team2.wait = 0;
                team2.cooldown = 0;
            }
            else {
                team2.fainted = true;
            }
        }
        if team1.fainted && team2.fainted {
            return CombatResult::Draw;
        }
        else if team1.fainted {
            return CombatResult::Second;
        }
        else if team2.fainted {
            return CombatResult::First;
        }
        debug!("next turn")
    }
}

#[cfg(test)]
mod test {
    use chrono::offset::Local;

    use log::info;

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

    #[test]
    fn timing() {
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
        let sceptile: PokemonSettings = serde_json::from_str(r#"{
            "uniqueId": "SCEPTILE",
            "modelScale": 0.8,
            "type1": "POKEMON_TYPE_GRASS",
            "camera": {
            "diskRadiusM": 0.555,
            "cylinderRadiusM": 1.0,
            "cylinderHeightM": 1.7,
            "shoulderModeScale": 0.5
            },
            "encounter": {
            "baseCaptureRate": 0.05,
            "baseFleeRate": 0.05,
            "collisionRadiusM": 0.25,
            "collisionHeightM": 0.6,
            "collisionHeadRadiusM": 0.25,
            "movementType": "MOVEMENT_JUMP",
            "movementTimerS": 11.0,
            "jumpTimeS": 1.1,
            "attackTimerS": 20.0,
            "bonusCandyCaptureReward": 7,
            "bonusStardustCaptureReward": 400,
            "attackProbability": 0.2,
            "dodgeProbability": 0.15,
            "dodgeDurationS": 1.0,
            "dodgeDistance": 1.0,
            "cameraDistance": 5.0,
            "minPokemonActionFrequencyS": 0.2,
            "maxPokemonActionFrequencyS": 1.6
            },
            "stats": {
            "baseStamina": 172,
            "baseAttack": 223,
            "baseDefense": 169
            },
            "quickMoves": ["FURY_CUTTER_FAST", "BULLET_SEED_FAST"],
            "cinematicMoves": ["LEAF_BLADE", "AERIAL_ACE", "EARTHQUAKE", "DRAGON_CLAW"],
            "animTime": [1.3333, 0.6667, 1.6667, 2.0, 0.0, 2.0, 3.0, 3.0],
            "evolutionPips": 1,
            "pokedexHeightM": 1.7,
            "pokedexWeightKg": 52.2,
            "parentId": "GROVYLE",
            "heightStdDev": 0.2125,
            "weightStdDev": 6.525,
            "familyId": "FAMILY_TREECKO",
            "kmBuddyDistance": 3.0,
            "buddySize": "BUDDY_BIG",
            "modelHeight": 1.7,
            "modelScaleV2": 1.0,
            "buddyOffsetMale": [15.0, 0.0, 27.0],
            "buddyOffsetFemale": [15.0, 0.0, 27.0],
            "buddyScale": 19.0,
            "thirdMove": {
            "stardustToUnlock": 10000,
            "candyToUnlock": 25
            },
            "isTransferable": true,
            "isDeployable": true,
            "buddyGroupNumber": 3
        }"#).unwrap();
        let dialga: PokemonSettings = serde_json::from_str(r#"{
            "uniqueId": "DIALGA",
            "modelScale": 1.26,
            "type1": "POKEMON_TYPE_STEEL",
            "type2": "POKEMON_TYPE_DRAGON",
            "camera": {
              "diskRadiusM": 0.378,
              "cylinderRadiusM": 2.0,
              "cylinderHeightM": 5.0,
              "shoulderModeScale": 0.5
            },
            "encounter": {
              "baseCaptureRate": 0.02,
              "baseFleeRate": 0.04,
              "collisionRadiusM": 0.7,
              "collisionHeightM": 1.5,
              "collisionHeadRadiusM": 0.4,
              "movementType": "MOVEMENT_JUMP",
              "movementTimerS": 10.0,
              "jumpTimeS": 0.9,
              "attackTimerS": 29.0,
              "attackProbability": 0.1,
              "dodgeProbability": 0.15,
              "dodgeDurationS": 1.0,
              "dodgeDistance": 1.0,
              "cameraDistance": 6.0,
              "minPokemonActionFrequencyS": 0.2,
              "maxPokemonActionFrequencyS": 1.6
            },
            "stats": {
              "baseStamina": 205,
              "baseAttack": 275,
              "baseDefense": 211
            },
            "quickMoves": ["DRAGON_BREATH_FAST", "METAL_CLAW_FAST"],
            "cinematicMoves": ["DRACO_METEOR", "IRON_HEAD", "THUNDER"],
            "animTime": [1.9, 0.6667, 1.8, 1.7667, 0.0, 2.4, 0.8667, 0.0],
            "evolutionPips": 1,
            "pokemonClass": "POKEMON_CLASS_LEGENDARY",
            "pokedexHeightM": 5.4,
            "pokedexWeightKg": 683.0,
            "heightStdDev": 0.675,
            "weightStdDev": 85.375,
            "familyId": "FAMILY_DIALGA",
            "candyToEvolve": 25,
            "kmBuddyDistance": 20.0,
            "buddySize": "BUDDY_BIG",
            "modelHeight": 5.6,
            "modelScaleV2": 0.61,
            "buddyOffsetMale": [1.1, 0.0, -14.79],
            "buddyOffsetFemale": [1.1, 0.0, -14.79],
            "buddyScale": 19.0,
            "thirdMove": {
              "stardustToUnlock": 100000,
              "candyToUnlock": 100
            },
            "isTransferable": true,
            "buddyGroupNumber": 3
        }"#).unwrap();
        let articuno: PokemonSettings = serde_json::from_str(r#"{
            "uniqueId": "ARTICUNO",
            "modelScale": 0.66,
            "type1": "POKEMON_TYPE_ICE",
            "type2": "POKEMON_TYPE_FLYING",
            "camera": {
              "diskRadiusM": 0.594,
              "cylinderRadiusM": 1.25,
              "cylinderHeightM": 1.25,
              "cylinderGroundM": 1.0,
              "shoulderModeScale": 0.5
            },
            "encounter": {
              "baseCaptureRate": 0.03,
              "baseFleeRate": 0.1,
              "collisionRadiusM": 0.231,
              "collisionHeightM": 0.66,
              "collisionHeadRadiusM": 0.231,
              "movementType": "MOVEMENT_FLYING",
              "movementTimerS": 3.0,
              "jumpTimeS": 1.0,
              "attackTimerS": 8.0,
              "attackProbability": 0.1,
              "dodgeProbability": 0.15,
              "dodgeDurationS": 1.0,
              "dodgeDistance": 1.0,
              "cameraDistance": 3.7125,
              "minPokemonActionFrequencyS": 0.2,
              "maxPokemonActionFrequencyS": 1.6
            },
            "stats": {
              "baseStamina": 207,
              "baseAttack": 192,
              "baseDefense": 236
            },
            "quickMoves": ["FROST_BREATH_FAST", "ICE_SHARD_FAST"],
            "cinematicMoves": ["ICE_BEAM", "ICY_WIND", "BLIZZARD", "ANCIENT_POWER"],
            "animTime": [2.0667, 1.0, 1.6667, 0.6667, 0.0, 2.4, 1.3333, 1.333333],
            "evolutionPips": 1,
            "pokemonClass": "POKEMON_CLASS_LEGENDARY",
            "pokedexHeightM": 1.7,
            "pokedexWeightKg": 55.4,
            "heightStdDev": 0.2125,
            "weightStdDev": 6.925,
            "familyId": "FAMILY_ARTICUNO",
            "kmBuddyDistance": 20.0,
            "buddySize": "BUDDY_FLYING",
            "modelHeight": 2.6,
            "modelScaleV2": 0.91,
            "buddyOffsetMale": [10.0, -16.9, 28.01],
            "buddyOffsetFemale": [10.0, -16.9, 28.01],
            "buddyScale": 19.0,
            "thirdMove": {
              "stardustToUnlock": 100000,
              "candyToUnlock": 100
            },
            "isTransferable": true,
            "buddyGroupNumber": 7
        }"#).unwrap();
        let swampert: PokemonSettings = serde_json::from_str(r#"{
            "uniqueId": "SWAMPERT",
            "modelScale": 0.78,
            "type1": "POKEMON_TYPE_WATER",
            "type2": "POKEMON_TYPE_GROUND",
            "camera": {
              "diskRadiusM": 0.555,
              "cylinderRadiusM": 0.75,
              "cylinderHeightM": 1.7,
              "shoulderModeScale": 0.5
            },
            "encounter": {
              "baseCaptureRate": 0.05,
              "baseFleeRate": 0.05,
              "collisionRadiusM": 0.4,
              "collisionHeightM": 0.8,
              "collisionHeadRadiusM": 0.25,
              "movementType": "MOVEMENT_JUMP",
              "movementTimerS": 11.0,
              "jumpTimeS": 1.1,
              "attackTimerS": 20.0,
              "bonusCandyCaptureReward": 7,
              "bonusStardustCaptureReward": 400,
              "attackProbability": 0.2,
              "dodgeProbability": 0.15,
              "dodgeDurationS": 1.0,
              "dodgeDistance": 1.0,
              "cameraDistance": 5.0,
              "minPokemonActionFrequencyS": 0.2,
              "maxPokemonActionFrequencyS": 1.6
            },
            "stats": {
              "baseStamina": 225,
              "baseAttack": 208,
              "baseDefense": 175
            },
            "quickMoves": ["MUD_SHOT_FAST", "WATER_GUN_FAST"],
            "cinematicMoves": ["EARTHQUAKE", "SLUDGE_WAVE", "SURF", "MUDDY_WATER"],
            "animTime": [1.3333, 0.6667, 1.6667, 2.0, 0.0, 2.0, 3.0, 3.0],
            "evolutionPips": 1,
            "pokedexHeightM": 1.5,
            "pokedexWeightKg": 81.9,
            "parentId": "MARSHTOMP",
            "heightStdDev": 0.1875,
            "weightStdDev": 10.2375,
            "familyId": "FAMILY_MUDKIP",
            "kmBuddyDistance": 3.0,
            "buddySize": "BUDDY_BIG",
            "modelHeight": 1.8,
            "modelScaleV2": 1.0,
            "buddyOffsetMale": [20.0, 0.0, 0.0],
            "buddyOffsetFemale": [20.0, 0.0, 0.0],
            "buddyScale": 19.0,
            "thirdMove": {
              "stardustToUnlock": 10000,
              "candyToUnlock": 25
            },
            "isTransferable": true,
            "isDeployable": true,
            "buddyGroupNumber": 3
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
        let shadow_sneak: CombatMove = serde_json::from_str(r#"{
            "uniqueId": "SHADOW_SNEAK",
            "type": "POKEMON_TYPE_GHOST",
            "power": 50.0,
            "vfxName": "shadow_sneak",
            "energyDelta": -45
        }"#).unwrap();
        let mud_shot: CombatMove = serde_json::from_str(r#"{
            "uniqueId": "MUD_SHOT_FAST",
            "type": "POKEMON_TYPE_GROUND",
            "power": 3.0,
            "vfxName": "mud_shot_fast",
            "durationTurns": 1,
            "energyDelta": 9
        }"#).unwrap();
        let hydro_cannon: CombatMove = serde_json::from_str(r#"{
            "uniqueId": "HYDRO_CANNON",
            "type": "POKEMON_TYPE_WATER",
            "power": 80.0,
            "vfxName": "hydro_cannon",
            "energyDelta": -40
        }"#).unwrap();
        let earthquake: CombatMove = serde_json::from_str(r#"{
            "uniqueId": "EARTHQUAKE",
            "type": "POKEMON_TYPE_GROUND",
            "power": 120.0,
            "vfxName": "earthquake",
            "energyDelta": -65
        }"#).unwrap();
        let ice_shard: CombatMove = serde_json::from_str(r#"{
            "uniqueId": "ICE_SHARD_FAST",
            "type": "POKEMON_TYPE_ICE",
            "power": 9.0,
            "vfxName": "ice_shard_fast",
            "durationTurns": 2,
            "energyDelta": 10
        }"#).unwrap();
        let hurricane: CombatMove = serde_json::from_str(r#"{
            "uniqueId": "HURRICANE",
            "type": "POKEMON_TYPE_FLYING",
            "power": 110.0,
            "vfxName": "hurricane",
            "energyDelta": -65
        }"#).unwrap();
        let icy_wind: CombatMove = serde_json::from_str(r#"{
            "uniqueId": "ICY_WIND",
            "type": "POKEMON_TYPE_ICE",
            "power": 60.0,
            "vfxName": "icy_wind",
            "energyDelta": -45,
            "buffs": {
            "targetAttackStatStageChange": -1,
            "buffActivationChance": 1.0
            }
        }"#).unwrap();
        let frenzy_plant: CombatMove = serde_json::from_str(r#"{
            "uniqueId": "FRENZY_PLANT",
            "type": "POKEMON_TYPE_GRASS",
            "power": 100.0,
            "vfxName": "frenzy_plant",
            "energyDelta": -45
        }"#).unwrap();
        let fury_cutter: CombatMove = serde_json::from_str(r#"{
            "uniqueId": "FURY_CUTTER_FAST",
            "type": "POKEMON_TYPE_BUG",
            "power": 2.0,
            "vfxName": "fury_cutter_fast",
            "energyDelta": 4
        }"#).unwrap();
        let draco_meteor: CombatMove = serde_json::from_str(r#"{
            "uniqueId": "DRACO_METEOR",
            "type": "POKEMON_TYPE_DRAGON",
            "power": 150.0,
            "vfxName": "draco_meteor",
            "energyDelta": -65,
            "buffs": {
            "attackerAttackStatStageChange": -2,
            "buffActivationChance": 1.0
            }
        }"#).unwrap();
        let iron_head: CombatMove = serde_json::from_str(r#"{
            "uniqueId": "IRON_HEAD",
            "type": "POKEMON_TYPE_STEEL",
            "power": 70.0,
            "vfxName": "iron_head",
            "energyDelta": -50
        }"#).unwrap();
        let m0 = Moveset {
            pokemon: &giratina,
            cp: 2491,
            level: 26,
            atk: 10,
            def: 15,
            sta: 14,
            cpm: cpm[25],
            fast_move: &dragon_breath,
            fast_legacy: None,
            charged_move1: &dragon_claw,
            charged_legacy1: None,
            charged_move2: &shadow_sneak,
            charged_legacy2: None,
        };
        let m1 = Moveset {
            pokemon: &swampert,
            cp: 2472,
            level: 29,
            atk: 13,
            def: 14,
            sta: 14,
            cpm: cpm[28],
            fast_move: &mud_shot,
            fast_legacy: None,
            charged_move1: &hydro_cannon,
            charged_legacy1: Some(true),
            charged_move2: &earthquake,
            charged_legacy2: None,
        };
        let m2 = Moveset {
            pokemon: &articuno,
            cp: 2495,
            level: 27,
            atk: 10,
            def: 10,
            sta: 10,
            cpm: cpm[26],
            fast_move: &ice_shard,
            fast_legacy: None,
            charged_move1: &hurricane,
            charged_legacy1: Some(true),
            charged_move2: &icy_wind,
            charged_legacy2: None,
        };
        let m3 = Moveset {
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
        let m4 = Moveset {
            pokemon: &sceptile,
            cp: 2499,
            level: 28,
            atk: 0,
            def: 15,
            sta: 15,
            cpm: cpm[27],
            fast_move: &fury_cutter,
            fast_legacy: None,
            charged_move1: &frenzy_plant,
            charged_legacy1: Some(true),
            charged_move2: &earthquake,
            charged_legacy2: None,
        };
        let m5 = Moveset {
            pokemon: &dialga,
            cp: 2499,
            level: 28,
            atk: 0,
            def: 15,
            sta: 15,
            cpm: cpm[27],
            fast_move: &dragon_breath,
            fast_legacy: None,
            charged_move1: &draco_meteor,
            charged_legacy1: None,
            charged_move2: &iron_head,
            charged_legacy2: None,
        };
        let start = Local::now();
        assert_eq!(combat(&[&m0, &m1, &m2], &[&m3, &m4, &m5]), CombatResult::First);
        let end = Local::now();
        info!("took {} nanoseconds", end.timestamp_nanos() - start.timestamp_nanos());
    }
}
