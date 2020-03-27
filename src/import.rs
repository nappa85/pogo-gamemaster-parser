use std::collections::HashMap;
use std::path::Path;

use serde::{Deserialize, Serialize};

use futures_util::stream::StreamExt;

use tokio::{fs::{File, read_dir, create_dir, remove_file}, io::{AsyncReadExt, AsyncWriteExt}};

use once_cell::sync::Lazy;

use log::error;

use crate::entities::{CombatMove/*, PokemonSettings*/};

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

#[derive(Deserialize, Serialize)]
#[allow(dead_code)]
struct OwnedMoveset {
    // pokemon: PokemonSettings,
    unique_id: String,
    type1: String,
    type2: Option<String>,
    base_attack: u16,
    base_defense: u16,
    // 
    cp: u32,
    level: u8,
    atk: u8,
    def: u8,
    sta: u8,
    cpm: f64,
    fast_move: CombatMove,
    fast_legacy: Option<bool>,
    charged_move1: CombatMove,
    charged_legacy1: Option<bool>,
    charged_move2: CombatMove,
    charged_legacy2: Option<bool>,
}

fn _get_damage(m: &CombatMove, atk: &OwnedMoveset, def: &OwnedMoveset) -> i32 {
    let mut multipliers = 1.0;
    //stab
    if m.r#type == atk.type1 || Some(&m.r#type) == atk.type2.as_ref() {
        multipliers *= 1.2;
    }
    multipliers *= EFFECTIVENESS[&m.r#type][&def.type1];
    if let Some(type2) = atk.type2.as_ref() {
        multipliers *= EFFECTIVENESS[&m.r#type][type2];
    }
    ((0.5 * (m.power.unwrap_or_else(|| 0.0_f32) as f64) *
        ((((atk.base_attack as f64) + (atk.atk as f64)) * atk.cpm) / (((def.base_defense as f64) + (def.def as f64)) * def.cpm)) *
        multipliers).floor() as i32) + 1
}

fn get_damage(p1: &OwnedMoveset, energy: &mut i32, wait: &mut i32, p2: &OwnedMoveset) -> i32 {
    let charged1_damage = _get_damage(&p1.charged_move1, p1, p2);
    let charged2_damage = _get_damage(&p1.charged_move2, p1, p2);
    if charged1_damage > charged2_damage {
        let energy_delta = p1.charged_move1.energy_delta.unwrap_or_else(|| 0) as i32;
        if *energy + energy_delta > 0 {
            *energy += energy_delta;
            return charged1_damage;
        }
    }
    else {
        let energy_delta = p2.charged_move1.energy_delta.unwrap_or_else(|| 0) as i32;
        if *energy + energy_delta > 0 {
            *energy += energy_delta;
            return charged2_damage;
        }
    }

    *energy += p1.fast_move.energy_delta.unwrap_or_else(|| 0) as i32;
    *wait = p1.fast_move.duration_turns.unwrap_or_else(|| 0) as i32;
    _get_damage(&p1.fast_move, p1, p2)
}

fn combat<'a>(team1: &'a [OwnedMoveset], team2: &'a [OwnedMoveset]) -> Option<&'a [OwnedMoveset]> {
    let mut team1_pokemon = 0;
    let mut team1_cp = team1[0].cp as i32;
    let mut team1_wait = 0;
    let mut team1_energy = 0;
    let mut team2_pokemon = 0;
    let mut team2_cp = team2[0].cp as i32;
    let mut team2_wait = 0;
    let mut team2_energy = 0;
    loop {
        if team1_wait > 0 {
            team1_wait -= 1;
        }
        else {
            team2_cp -= get_damage(&team1[team1_pokemon], &mut team1_energy, &mut team1_wait, &team2[team2_pokemon]);
        }
        if team2_wait > 0 {
            team2_wait -= 1;
        }
        else {
            team1_cp -= get_damage(&team2[team2_pokemon], &mut team2_energy, &mut team2_wait, &team1[team1_pokemon]);
        }

        if team1_cp <= 0 {
            team1_pokemon += 1;
            if team1_pokemon < 3 {
                team1_cp = team2[team1_pokemon].cp as i32;
                team1_energy = 0;
                team1_wait = 0;
            }
        }
        if team2_cp <= 0 {
            team2_pokemon += 1;
            if team2_pokemon < 3 {
                team2_cp = team2[team2_pokemon].cp as i32;
                team2_energy = 0;
                team2_wait = 0;
            }
        }
        if team1_pokemon > 2 && team2_pokemon > 2 {
            return None;
        }
        else if team1_pokemon > 2 {
            return Some(team2);
        }
        else if team2_pokemon > 2 {
            return Some(team1);
        }
    }
}

async fn digest(source_path: &Path, target: &Path) -> Result<(), ()> {
    let mut file = File::open(source_path).await.map_err(|e| error!("Can't open file {}: {}", source_path.display(), e))?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).await.map_err(|e| error!("Can't read file {}: {}", source_path.display(), e))?;
    let teams: Vec<Vec<OwnedMoveset>> = bincode::deserialize(&contents).map_err(|e| error!("Can't decode file  {}: {}", source_path.display(), e))?;
    if let Some(team) = combat(&teams[0], &teams[1]) {
        let mut filename = target.to_path_buf();
        filename.push(source_path.file_stem().unwrap());
        let mut target_file = File::create(&filename).await.map_err(|e| error!("Cant create file {}: {}", filename.display(), e))?;
        let contents = bincode::serialize(team).map_err(|e| error!("Can't serialize team: {}", e))?;
        target_file.write_all(&contents).await.map_err(|e| error!("Can't write file {}: {}", filename.display(), e))?;
    }
    remove_file(source_path).await.map_err(|e| error!("Can't delete file {}: {}", source_path.display(), e))?;
    Ok(())
}

pub async fn exec(source: &Path) -> Result<(), ()> {
    let target = Path::new("./results/");
    if !target.exists() {
        create_dir(&target).await.map_err(|e| error!("Can't create results folder: {}", e))?;
    }

    read_dir(source).await
        .map_err(|e| error!("Can't open matches folder: {}", e))?
        .for_each_concurrent(Some(10), |entry| async move {
            if let Ok(dir_entry) = entry {
                digest(&dir_entry.path(), target).await.ok();
            }
        })
        .await;

    Ok(())
}
