use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::entities::{CombatMove/*, PokemonSettings*/};
use crate::moveset::Moveset;

// EFFECTIVENESS[atk_type][def_type]
static EFFECTIVENESS: Lazy<HashMap<String, HashMap<String, f64>>> = Lazy::new(|| {
    let mut res = HashMap::new();
    let temp = res.entry(String::from("POKEMON_TYPE_BUG")).or_insert_with(HashMap::new);
    temp.insert(String::from("POKEMON_TYPE_BUG"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_DARK"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_DRAGON"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_ELECTRIC"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_FAIRY"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_FIGHTING"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_FIRE"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_FLYING"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_GHOST"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_GRASS"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_GROUND"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_ICE"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_NORMAL"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_POISON"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_PSYCHIC"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_ROCK"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_STEEL"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_WATER"), 1.0);
    let temp = res.entry(String::from("POKEMON_TYPE_DARK")).or_insert_with(HashMap::new);
    temp.insert(String::from("POKEMON_TYPE_BUG"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_DARK"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_DRAGON"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_ELECTRIC"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_FAIRY"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_FIGHTING"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_FIRE"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_FLYING"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_GHOST"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_GRASS"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_GROUND"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_ICE"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_NORMAL"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_POISON"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_PSYCHIC"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_ROCK"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_STEEL"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_WATER"), 1.0);
    let temp = res.entry(String::from("POKEMON_TYPE_DRAGON")).or_insert_with(HashMap::new);
    temp.insert(String::from("POKEMON_TYPE_BUG"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_DARK"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_DRAGON"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_ELECTRIC"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_FAIRY"), 0.51);
    temp.insert(String::from("POKEMON_TYPE_FIGHTING"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_FIRE"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_FLYING"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_GHOST"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_GRASS"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_GROUND"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_ICE"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_NORMAL"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_POISON"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_PSYCHIC"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_ROCK"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_STEEL"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_WATER"), 1.0);
    let temp = res.entry(String::from("POKEMON_TYPE_ELECTRIC")).or_insert_with(HashMap::new);
    temp.insert(String::from("POKEMON_TYPE_BUG"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_DARK"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_DRAGON"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_ELECTRIC"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_FAIRY"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_FIGHTING"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_FIRE"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_FLYING"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_GHOST"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_GRASS"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_GROUND"), 0.51);
    temp.insert(String::from("POKEMON_TYPE_ICE"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_NORMAL"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_POISON"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_PSYCHIC"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_ROCK"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_STEEL"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_WATER"), 1.0);
    let temp = res.entry(String::from("POKEMON_TYPE_FAIRY")).or_insert_with(HashMap::new);
    temp.insert(String::from("POKEMON_TYPE_BUG"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_DARK"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_DRAGON"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_ELECTRIC"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_FAIRY"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_FIGHTING"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_FIRE"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_FLYING"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_GHOST"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_GRASS"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_GROUND"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_ICE"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_NORMAL"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_POISON"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_PSYCHIC"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_ROCK"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_STEEL"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_WATER"), 1.0);
    let temp = res.entry(String::from("POKEMON_TYPE_FIGHTING")).or_insert_with(HashMap::new);
    temp.insert(String::from("POKEMON_TYPE_BUG"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_DARK"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_DRAGON"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_ELECTRIC"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_FAIRY"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_FIGHTING"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_FIRE"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_FLYING"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_GHOST"), 0.51);
    temp.insert(String::from("POKEMON_TYPE_GRASS"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_GROUND"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_ICE"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_NORMAL"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_POISON"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_PSYCHIC"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_ROCK"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_STEEL"), 0.4);
    temp.insert(String::from("POKEMON_TYPE_WATER"), 1.0);
    let temp = res.entry(String::from("POKEMON_TYPE_FIRE")).or_insert_with(HashMap::new);
    temp.insert(String::from("POKEMON_TYPE_BUG"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_DARK"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_DRAGON"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_ELECTRIC"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_FAIRY"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_FIGHTING"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_FIRE"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_FLYING"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_GHOST"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_GRASS"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_GROUND"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_ICE"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_NORMAL"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_POISON"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_PSYCHIC"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_ROCK"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_STEEL"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_WATER"), 0.714);
    let temp = res.entry(String::from("POKEMON_TYPE_FLYING")).or_insert_with(HashMap::new);
    temp.insert(String::from("POKEMON_TYPE_BUG"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_DARK"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_DRAGON"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_ELECTRIC"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_FAIRY"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_FIGHTING"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_FIRE"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_FLYING"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_GHOST"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_GRASS"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_GROUND"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_ICE"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_NORMAL"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_POISON"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_PSYCHIC"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_ROCK"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_STEEL"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_WATER"), 1.0);
    let temp = res.entry(String::from("POKEMON_TYPE_GHOST")).or_insert_with(HashMap::new);
    temp.insert(String::from("POKEMON_TYPE_BUG"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_DARK"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_DRAGON"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_ELECTRIC"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_FAIRY"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_FIGHTING"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_FIRE"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_FLYING"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_GHOST"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_GRASS"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_GROUND"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_ICE"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_NORMAL"), 0.51);
    temp.insert(String::from("POKEMON_TYPE_POISON"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_PSYCHIC"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_ROCK"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_STEEL"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_WATER"), 1.0);
    let temp = res.entry(String::from("POKEMON_TYPE_GRASS")).or_insert_with(HashMap::new);
    temp.insert(String::from("POKEMON_TYPE_BUG"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_DARK"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_DRAGON"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_ELECTRIC"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_FAIRY"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_FIGHTING"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_FIRE"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_FLYING"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_GHOST"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_GRASS"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_GROUND"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_ICE"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_NORMAL"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_POISON"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_PSYCHIC"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_ROCK"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_STEEL"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_WATER"), 1.4);
    let temp = res.entry(String::from("POKEMON_TYPE_GROUND")).or_insert_with(HashMap::new);
    temp.insert(String::from("POKEMON_TYPE_BUG"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_DARK"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_DRAGON"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_ELECTRIC"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_FAIRY"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_FIGHTING"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_FIRE"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_FLYING"), 0.51);
    temp.insert(String::from("POKEMON_TYPE_GHOST"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_GRASS"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_GROUND"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_ICE"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_NORMAL"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_POISON"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_PSYCHIC"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_ROCK"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_STEEL"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_WATER"), 1.0);
    let temp = res.entry(String::from("POKEMON_TYPE_ICE")).or_insert_with(HashMap::new);
    temp.insert(String::from("POKEMON_TYPE_BUG"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_DARK"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_DRAGON"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_ELECTRIC"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_FAIRY"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_FIGHTING"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_FIRE"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_FLYING"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_GHOST"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_GRASS"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_GROUND"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_ICE"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_NORMAL"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_POISON"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_PSYCHIC"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_ROCK"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_STEEL"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_WATER"), 0.714);
    let temp = res.entry(String::from("POKEMON_TYPE_NORMAL")).or_insert_with(HashMap::new);
    temp.insert(String::from("POKEMON_TYPE_BUG"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_DARK"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_DRAGON"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_ELECTRIC"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_FAIRY"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_FIGHTING"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_FIRE"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_FLYING"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_GHOST"), 0.51);
    temp.insert(String::from("POKEMON_TYPE_GRASS"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_GROUND"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_ICE"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_NORMAL"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_POISON"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_PSYCHIC"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_ROCK"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_STEEL"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_WATER"), 1.0);
    let temp = res.entry(String::from("POKEMON_TYPE_POISON")).or_insert_with(HashMap::new);
    temp.insert(String::from("POKEMON_TYPE_BUG"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_DARK"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_DRAGON"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_ELECTRIC"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_FAIRY"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_FIGHTING"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_FIRE"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_FLYING"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_GHOST"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_GRASS"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_GROUND"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_ICE"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_NORMAL"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_POISON"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_PSYCHIC"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_ROCK"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_STEEL"), 0.51);
    temp.insert(String::from("POKEMON_TYPE_WATER"), 1.0);
    let temp = res.entry(String::from("POKEMON_TYPE_PSYCHIC")).or_insert_with(HashMap::new);
    temp.insert(String::from("POKEMON_TYPE_BUG"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_DARK"), 0.51);
    temp.insert(String::from("POKEMON_TYPE_DRAGON"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_ELECTRIC"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_FAIRY"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_FIGHTING"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_FIRE"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_FLYING"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_GHOST"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_GRASS"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_GROUND"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_ICE"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_NORMAL"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_POISON"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_PSYCHIC"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_ROCK"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_STEEL"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_WATER"), 1.0);
    let temp = res.entry(String::from("POKEMON_TYPE_ROCK")).or_insert_with(HashMap::new);
    temp.insert(String::from("POKEMON_TYPE_BUG"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_DARK"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_DRAGON"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_ELECTRIC"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_FAIRY"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_FIGHTING"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_FIRE"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_FLYING"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_GHOST"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_GRASS"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_GROUND"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_ICE"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_NORMAL"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_POISON"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_PSYCHIC"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_ROCK"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_STEEL"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_WATER"), 1.0);
    let temp = res.entry(String::from("POKEMON_TYPE_STEEL")).or_insert_with(HashMap::new);
    temp.insert(String::from("POKEMON_TYPE_BUG"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_DARK"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_DRAGON"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_ELECTRIC"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_FAIRY"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_FIGHTING"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_FIRE"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_FLYING"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_GHOST"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_GRASS"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_GROUND"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_ICE"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_NORMAL"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_POISON"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_PSYCHIC"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_ROCK"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_STEEL"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_WATER"), 0.714);
    let temp = res.entry(String::from("POKEMON_TYPE_WATER")).or_insert_with(HashMap::new);
    temp.insert(String::from("POKEMON_TYPE_BUG"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_DARK"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_DRAGON"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_ELECTRIC"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_FAIRY"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_FIGHTING"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_FIRE"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_FLYING"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_GHOST"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_GRASS"), 0.714);
    temp.insert(String::from("POKEMON_TYPE_GROUND"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_ICE"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_NORMAL"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_POISON"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_PSYCHIC"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_ROCK"), 1.4);
    temp.insert(String::from("POKEMON_TYPE_STEEL"), 1.0);
    temp.insert(String::from("POKEMON_TYPE_WATER"), 0.714);
    res
});

fn _get_damage(m: &CombatMove, atk: &Moveset, def: &Moveset) -> i32 {
    let mut multipliers = 1.0;
    //stab
    if m.r#type == atk.pokemon.type1 || Some(&m.r#type) == atk.pokemon.type2.as_ref() {
        multipliers *= 1.2;
    }
    multipliers *= EFFECTIVENESS[&m.r#type][&def.pokemon.type1];
    if let Some(type2) = atk.pokemon.type2.as_ref() {
        multipliers *= EFFECTIVENESS[&m.r#type][type2];
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

pub enum CombatResult {
    Draw,
    First,
    Second,
}

pub fn combat<'a>(team1: &'a [&'a Moveset<'a>], team2: &'a [&'a Moveset<'a>]) -> CombatResult {
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
            team1_wait -= 1;
            team1_damage = None;
        }
        else {
            team1_damage = Some(get_damage(&team1[team1_pokemon], &mut team1_energy, &mut team1_wait, &team2[team2_pokemon]));
        }
        if team2_wait > 0 {
            team2_wait -= 1;
            team2_damage = None;
        }
        else {
            team2_damage = Some(get_damage(&team2[team2_pokemon], &mut team2_energy, &mut team2_wait, &team1[team1_pokemon]));
        }

        match (team1_damage, team2_damage) {
            (Some(DamageType::Charged(d1)), Some(DamageType::Charged(d2))) => {
                if team1[team1_pokemon].pokemon.stats.base_attack + (team1[team1_pokemon].atk as u16) >= team2[team2_pokemon].pokemon.stats.base_attack + (team2[team2_pokemon].atk as u16) {
                    team2_cp -= d1;
                    if team2_cp > 0 {
                        team1_cp -= d2;
                    }
                }
                else {
                    team1_cp -= d2;
                    if team1_cp > 0 {
                        team2_cp -= d1;
                    }
                }
            },
            (Some(DamageType::Charged(d1)), Some(DamageType::Fast(d2))) => {
                team2_cp -= d1;
                if team2_cp > 0 {
                    team1_cp -= d2;
                }
            },
            (Some(DamageType::Fast(d1)), Some(DamageType::Charged(d2))) => {
                team1_cp -= d2;
                if team1_cp > 0 {
                    team2_cp -= d1;
                }
            },
            (Some(DamageType::Fast(d1)), Some(DamageType::Fast(d2))) => {
                team1_cp -= d2;
                team2_cp -= d1;
            },
            (Some(DamageType::Fast(d1)), None) => {
                team2_cp -= d1;
            },
            (Some(DamageType::Charged(d1)), None) => {
                team2_cp -= d1;
            },
            (None, Some(DamageType::Fast(d2))) => {
                team1_cp -= d2;
            },
            (None, Some(DamageType::Charged(d2))) => {
                team1_cp -= d2;
            },
            (None, None) => {},
        }

        if team1_cp <= 0 {
            team1_pokemon += 1;
            if team1_pokemon < team1.len() {
                team1_cp = team2[team1_pokemon].cp as i32;
                team1_energy = 0;
                team1_wait = 0;
            }
        }
        if team2_cp <= 0 {
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
    }
}
