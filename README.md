# PoGo-GameMaster-Parser

## Prject history

This project was born as a Rust program that parses the GameMaster, pushing the resulting combinations (aka every moveset and IV combination) into a database.<br />
To consume the database data, it's needed also a frontend, with a minimal backend to retrieve the data.<br />
Despite having the best possible performances, I had 3 different elements to maintain:
* Rust Parser
* Site Backend
* Site Frontend

This is a lot of work for a project that simple, not made for a big audience.<br />
So I tried to throw away everything except the frontend (you can still find the project in the `html` branch), and made everything work from client's browser.<br />
Performances were really bad, as in every modern web application, but worst.<br />
Maybe, one day, I'll redo everyting with WASM for good performances, but not today.

So I returned to the Rust version, adding the second charged move to the movesets and creating the concept of team, trying to computer every possible team match.<br/>
Turns out that, even with limited Pokémon GO meta, it would have been like YEARS of computation and a result so big it wouldn't fit my computer's RAM.

The only possible evolution was to reduce the meta, so I ended up accepting input files that described the meta we are taking in consideration, and from there the step to specify also you IVs in the file was really short, this way the simulation is "your pokémon against the perfect teams", AKA the worst case scenario.

## Features

* This simulator handles changes, so only the opener is a fixed slot in a team. What does it means? That given 3 Pokémon A, B and C, the simulator only takes in account a single combination between ABC and ACB.
* The simulator also handles buffs from charged moves, but only the ones with 1.0 probability (worst case scenario, remember?). The buffs are mantaned despite the changes, but it isn't clear if the game does it too or resets the buffs on change.
* In the case of both Pokémon launching a charged move in the same turn, the simulator checks for attack value of both Pokémon and gives piority to he highest one, like the game. In case of equal values, it gives priority to team2 intead of randomly choosing like the game (worst case scenario FTW).
* In the case of both Pokémon dying in the same turn, the simulator first takes a new Pokémon (if existing) for team1, and after that takes the best Pokémon agaist team1's Pokémon from the remaining team2's Pokémons, this gives an advantage to team2 (worst case scenario, as always).

## Performances

It uses extensively multithreading to get the best performances out of your computer.<br/>
Keep in mind that a match takes an average of 1 millisecond to be computed, so a match between two metas of 30 movesets each will end up as:
```
(30!/(2!27!))^2 = 12180^2 = 148352400 milliseconds = 41.209 hours
```
That count is the total single threaded time, if your computer has more that one core, you can divide up.<br/>
For example, for a computer with 8 cores:
```
41.209/8 = 5.151125 hours
```
As always,this is a worst case scenario, it can take less, or more, it depens on a number of factors.

## Usage

The project is made with Rust (you can install it with [rustup](https://rustup.rs/)).

To run the tests just run:
```
RUST_LOG=debug cargo test
```
To know the meaning of RUST_LOG env var check the [env_logger crate](https://docs.rs/env_logger/).

To run a simulation, just run:
```
cargo run --release -- -l <league> -f <your pokemon file> -s <meta pokemon file>
```
For example:
```
cargo run --release -- -l master -f ./my_iv.json -s ./meta.json
```

## File format

Input files have both the same JSON schema:
```
{
    pokemon: String,
    form: Option<String>,
    fast: Option<String>,
    charged: Option<Vec<String>>,
    level: Option<u8>,
    atk: Option<u8>,
    def: Option<u8>,
    sta: Option<u8>,
}
```
Pokémon and moves are in the [Game Master](https://raw.githubusercontent.com/pokemongo-dev-contrib/pokemongo-game-master/master/versions/latest/GAME_MASTER.json) format, so, for example, Bite fast move is "BITE_FAST".

Where an Option field is missing, the filter isn't applied, or, in the case of level, atk, def and sta, the best possible value is kept.
This is useful when making the meta file, for example:
```json
[
{"pokemon":"ARTICUNO","fast":"ICE_SHARD_FAST","charged":["ICY_WIND", "HURRICANE"]},
{"pokemon":"BLAZIKEN","fast":"COUNTER_FAST","charged":["BLAST_BURN", "BLAZE_KICK"]},
{"pokemon":"CONKELDURR","fast":"COUNTER_FAST","charged":["DYNAMIC_PUNCH", "STONE_EDGE"]},
{"pokemon":"DARKRAI","fast":"SNARL_FAST","charged":["DARK_PULSE", "FOCUS_BLAST"]},
{"pokemon":"DIALGA","fast":"DRAGON_BREATH_FAST","charged":["IRON_HEAD", "THUNDER"]},
{"pokemon":"DRAGONITE","fast":"DRAGON_BREATH_FAST","charged":["DRAGON_CLAW", "DRACO_METEOR"]},
{"pokemon":"GARCHOMP","fast":"MUD_SHOT_FAST","charged":["OUTRAGE", "EARTHQUAKE"]},
{"pokemon":"GIRATINA","fast":"SHADOW_CLAW_FAST","charged":["DRAGON_CLAW", "SHADOW_SNEAK"]},
{"pokemon":"GIRATINA","form":"GIRATINA_ORIGIN","fast":"SHADOW_CLAW_FAST","charged":["OMINOUS_WIND", "SHADOW_BALL"]},
{"pokemon":"GLACEON","fast":"ICE_SHARD_FAST","charged":["ICY_WIND", "AVALANCHE"]},
{"pokemon":"GROUDON","fast":"MUD_SHOT_FAST","charged":["EARTHQUAKE", "FIRE_BLAST"]},
{"pokemon":"GYARADOS","fast":"DRAGON_BREATH_FAST","charged":["CRUNCH", "OUTRAGE"]},
{"pokemon":"HAXORUS","fast":"COUNTER_FAST","charged":["DRAGON_CLAW", "NIGHT_SLASH"]},
{"pokemon":"HEATRAN","fast":"FIRE_SPIN_FAST","charged":["IRON_HEAD", "FLAMETHROWER"]},
{"pokemon":"HERACROSS","fast":"COUNTER_FAST","charged":["CLOSE_COMBAT", "MEGAHORN"]},
{"pokemon":"KYOGRE","fast":"WATERFALL_FAST","charged":["SURF", "BLIZZARD"]},
{"pokemon":"LANDORUS","fast":"MUD_SHOT_FAST","charged":["ROCK_SLIDE", "EARTH_POWER"]},
{"pokemon":"LUGIA","fast":"DRAGON_TAIL_FAST","charged":["SKY_ATTACK", "HYDRO_PUMP"]},
{"pokemon":"MACHAMP","fast":"COUNTER_FAST","charged":["CROSS_CHOP", "ROCK_SLIDE"]},
{"pokemon":"MELMETAL","fast":"THUNDER_SHOCK_FAST","charged":["SUPER_POWER", "ROCK_SLIDE"]},
{"pokemon":"METAGROSS","fast":"BULLET_PUNCH_FAST","charged":["METEOR_MASH", "EARTHQUAKE"]},
{"pokemon":"MEWTWO","fast":"PSYCHO_CUT_FAST","charged":["PSYSTRIKE", "FOCUS_BLAST", "ICE_BEAM"]},
{"pokemon":"MEWTWO","fast":"PSYCHO_CUT_FAST","charged":["FOCUS_BLAST", "SHADOW_BALL", "ICE_BEAM"]},
{"pokemon":"RHYPERIOR","fast":"MUD_SLAP_FAST","charged":["ROCK_WRECKER", "SURF"]},
{"pokemon":"SNORLAX","fast":"LICK_FAST","charged":["BODY_SLAM", "EARTHQUAKE"]},
{"pokemon":"SWAMPERT","fast":"MUD_SHOT_FAST","charged":["HYDRO_CANNON", "EARTHQUAKE"]},
{"pokemon":"TOGEKISS","fast":"CHARM_FAST","charged":["ANCIENT_POWER", "FLAMETHROWER"]},
{"pokemon":"TORTERRA","fast":"RAZOR_LEAF_FAST","charged":["FRENZY_PLANT", "SAND_TOMB"]},
{"pokemon":"TYRANITAR","fast":"SMACK_DOWN_FAST","charged":["STONE_EDGE", "CRUNCH"]}
]
```
As you can see, level, atk, def and sta aren't specified, in master league this means that all pokemon will be level 41 with 100% IV.

Instead, when making your Pokémon file:
```json
[
{"pokemon":"MEWTWO","fast":"PSYCHO_CUT_FAST","charged":["FOCUS_BLAST", "SHADOW_BALL", "ICE_BEAM"],"level":40,"atk":15,"def":13,"sta":15},
{"pokemon":"DRAGONITE","fast":"DRAGON_BREATH_FAST","charged":["DRAGON_CLAW", "DRACO_METEOR"],"level":40,"atk":15,"def":15,"sta":15},
{"pokemon":"GIRATINA","fast":"SHADOW_CLAW_FAST","charged":["OMINOUS_WIND", "SHADOW_BALL"],"level":40,"atk":15,"def":14,"sta":10},
{"pokemon":"TYRANITAR","fast":"SMACK_DOWN_FAST","charged":["STONE_EDGE", "CRUNCH"],"level":40,"atk":15,"def":14,"sta":15},
{"pokemon":"DIALGA","fast":"DRAGON_BREATH_FAST","charged":["IRON_HEAD", "THUNDER"],"level":40,"atk":15,"def":14,"sta":15},
{"pokemon":"MEWTWO","fast":"PSYCHO_CUT_FAST","charged":["PSYSTRIKE", "FLAMETHROWER"],"level":41,"atk":15,"def":15,"sta":14},
{"pokemon":"TOGEKISS","fast":"CHARM_FAST","charged":["ANCIENT_POWER", "AERIAL_ACE"],"level":40,"atk":15,"def":14,"sta":14},
{"pokemon":"MELMETAL","fast":"THUNDER_SHOCK_FAST","charged":["SUPER_POWER", "ROCK_SLIDE"],"level":40,"atk":15,"def":13,"sta":15},
{"pokemon":"SCIZOR","fast":"BULLET_PUNCH_FAST","charged":["X_SCISSOR", "NIGHT_SLASH"],"level":40,"atk":15,"def":15,"sta":15},
{"pokemon":"GIRATINA","fast":"SHADOW_CLAW_FAST","charged":["DRAGON_CLAW", "SHADOW_SNEAK"],"level":40,"atk":15,"def":15,"sta":15},
{"pokemon":"DARKRAI","fast":"SNARL_FAST","charged":["DARK_PULSE", "FOCUS_BLAST"],"level":40,"atk":15,"def":11,"sta":15},
{"pokemon":"GARCHOMP","fast":"MUD_SHOT_FAST","charged":["OUTRAGE", "EARTHQUAKE"],"level":41,"atk":15,"def":14,"sta":15},
{"pokemon":"GARDEVOIR","fast":"CHARM_FAST","charged":["PSYCHIC", "SYNCHRONOISE"],"level":40,"atk":15,"def":14,"sta":15}
]
```
As you can seen, every pokemon is level 40 (because we are in master league), except for the ones you alrady made best friend with (keep in mind that you can only have a single active best friend at time, so this simulation is a bit invalid).
