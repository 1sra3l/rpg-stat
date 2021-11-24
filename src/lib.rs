/*!
# RPG Stat library

[![Documentation](https://docs.rs/rpg-stat/badge.svg)](https://docs.rs/rpg-stat)
[![Crates.io](https://img.shields.io/crates/v/rpg-stat.svg)](https://crates.io/crates/rpg-stat)

Cargo.toml

`rpgstat="4.0"`

This is fairly exhaustive and links to most things you can use.
The library is **still a WIP** as the battle system is rudimentary in the current form.  [TOML](https://crates.io/crates/toml) format with [serde](https://crates.io/crates/serde) is supported.

# **breaking changes**
A choice was made to remove data from this library and move it to a [repo]() as it became too much information for the scope of this crate, very quickly.

This breaks **all** previous usage using `rpgstat::creature::*;`
This breaks **all** previous usage using `rpgstat::legendary::*;`

But adds quite a bit, by removing so much!

Using the `--feature fltkform` you can now build a GUI without coding!  It does require [FLTK](https://github.com/fltk-rs/) which has many incredible features built in.

## Only SPECIFIC stats are supported for FLTK

This is due to limitations of abstraction


# Stats

The Stats are broken down into categories `Basic`, `Normal`, and `Advanced`

`Basic` contains only the most needed for a generic game
Your file needs:
`use rpgstat::stats::Basic as Stats`
 * id
 * xp
 * xp_next
 * level
 * gp
 * hp
 * mp
 * hp_max
 * mp_max
 * speed

`Normal` includes a few more for the generic RPG battle system as well as everything in `Basic`
Your file needs:
`use rpgstat::stats::Normal as Stats`
 * atk
 * def
 * m_atk
 * m_def

`Advanced` contains the finer details seen in tabletop RPG stats as well as everything in `Normal` and `Basic`
Your file needs:
`use rpgstat::stats::Advanced as Stats`
 * agility
 * strength
 * dexterity
 * constitution
 * intelligence
 * charisma
 * wisdom
 * age

You can easily **ANY** of build these to populate however you like:
```
// choose Normal or Basic if you'd rather...
use rpgstat::stats::Advanced as Stats;
let stats:Stats<f64> = Stats::empty::<f64>();
```

## Serde + TOML/INI

Yes you can use serde with any of the assets/characters/ files provided.  You can use them in your custom structs.

## Custom toml/ini with serde

```
use serde::{Deserialize, Serialize};
use rpgstat::attributes::{Effectiveness, Value};
use rpgstat::class::Basic as Class;
use rpgstat::stats::Basic as Stats;

// example program
const INI_FILE:&str = r#"name="test"
class="Hero"
effectiveness="None"
image="/path/to/file"
[stats]
id = 1
hp = 10
mp = 10
xp = 10
level = 1
hp_max = 10
mp_max = 10
xp_next = 10
gp = 10
speed = 10
atk = 10
def = 10
m_atk = 10
m_def = 10
agi = 10
str = 10
int = 10
dex = 10
con = 10
char = 10
wis = 10
age = 10"#;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct OccasionalEnemy {
    pub name:String,
    pub image:String,
    pub stats:Stats<f64>,
    pub effectiveness:Effectiveness,
    pub class:Class,
}
let decoded: OccasionalEnemy = toml::from_str(INI_FILE).unwrap();
assert_eq!(decoded.stats.hp, 10.0);
assert_eq!(decoded.effectiveness, Effectiveness::None);
// Value trait used here:
assert_eq!(0.0, Effectiveness::None.value(decoded.stats.hp));
assert_eq!(decoded.name, String::from("test"));
assert_eq!(decoded.class.to_string(), String::from("Hero"));
```
## Builder
Since the 1.X version `rpg-stat` has come with a `Builder` trait.
The builder trait is being implemented for all the enumerations like the `rpgstat::class::*` as well as `rpgstat::creature::*`

This allows you to do:
```
// feel free to use `Normal` or `Advanced` instead of `Basic`
use rpgstat::stats::Basic as Stats;
use rpgstat::class::Basic as Class;

// this is the thing we need!
use rpgstat::stats::Builder;

// get Hero stats for our program
fn hero_stats () -> Stats<f64> {
    // make the hero enum
    let hero:Class = Class::Hero;
    // this number only matters if you want
    let id:f64 = 0.0;
    // this effects the stats returned
    let level:f64 = 1.0;
    // use the basic `Builder`
    let hero_stats:Stats<f64> = hero.build_basic(id, level);
    // that was easy!
    hero_stats
}

// TODO make them meet...
```

## Build your own!
If you are not into making stats from things I made, you can implement your own builder:
```toml
num = "0.2"
rpg-stat = "4.0"
toml = "0.5"
```

# Classes

The Classes are broken down into categories `Basic`, `Normal`, and `Advanced`

The `Basic` class is either `Hero` or `Enemy`
Your file needs:
`use rpgstat::class::Basic as Class`

The `Normal` class includes a range of character classes for a battle game.
Your file needs:
`use rpgstat::class::Normal as Class`

`Advanced` includes more characters for a game with interactive roles, not simply a game of battle.
Your file needs:
`use rpgstat::class::Advanced as Class`

The stat `Builder` is implemented for all the classes and can be used easily:

```
use rpgstat::stats::Normal as Stats;
use rpgstat::class::Normal as Class;
// *Use this*
use rpgstat::stats::Builder;

// get Hero stats for our program
fn hero_stats () -> Stats<f64> {
    // make the hero enum
    let hero:Class = Class::Alchemist;
    // this number only matters if you want
    let id:f64 = 0.0;
    // this effects the stats returned
    let level:f64 = 1.0;
    // use the basic `Builder`
    let hero_stats:Stats<f64> = hero.build_normal(id, level);
    // that was easy!
    hero_stats
}

```

# Creatures



# Types

This includes various enums related to the type of character you have

`use rpgstat::types::Basic as Type`

 * `Basic` is the basic type `Good` or `Bad`
 * `Normal` has elemental types
 * `Advanced` has elemental types

## Compare

The Compare trait is implemented for `Normal`
according to this chart:
 
```
use rpgstat::types::Normal as Type;
// to check effectiveness
use rpgstat::types::Compare;
// need effectiveness too!
use rpgstat::attributes::Effectiveness;

let rock = Type::Rock;
let wind = Type::Wind;
assert_eq!(rock.effectiveness(wind), Effectiveness::None);
assert_eq!(wind.effectiveness(rock), Effectiveness::Double);
```

# Special

These are names of `Special` moves.

```
use rpgstat::special::Normal as Special;
let grind:Special = Special::Grind;
```

# Effect
This composes the various Effects in-game related to a character's Stats


# Attributes

These are definitions of abstract terms into code

## Rate
Rate of occurance
```
use rpgstat::attributes::Rate;
let yes:Rate = Rate::Always;
assert_eq!(yes.worked(), true);
let no:Rate = Rate::None;
assert_eq!(no.worked(), false);
let hmmm:Rate = Rate::Some;
// who knows....
```

## Effectiveness

This effectiveness can be stored in a struct and you could implement a wrapper for `value(T)`:
```
use rpgstat::attributes::{Effectiveness, Value};

pub struct Item {
    pub name:String,
    pub amount:i32,
    pub effectiveness:Effectiveness,
}
impl Item {
    // much easier to use now!
    pub fn value(&self) -> i32 {
        self.effectiveness.value(self.amount)
    }
}
```

```
use rpgstat::attributes::{Effectiveness, Value};
let hp:i32 = 50;
// later on we use an item and check the effectiveness of it
assert_eq!(Effectiveness::Half.value(hp), 25);

```

## Stage

```
use rpgstat::attributes::Stage;

```
This includes the `Stage` of life.  This is similar to things like "evolution" in creature raising games, but based on reality.  In real life no creature evolves randomly in front of someone, however they do get older and change their "form".  There are eight forms:
 * Baby
 * Toddler
 * Kid
 * Teen
 * Young
 * Grown
 * Older
 * Old

# Body
This is to collect all the information about armor, stats, status, etc, based on each body part.  This will be some serious numeric control over the simulation.


# RPG Stat command line tool
 WIP
 Ideally, this will use `clap` and support some very specific stat traits only.

 AFAIK the interface will end up looking like:
 `rpgstat class normal Archer stat normal hp`
 
 That said none of the work has been started yet, and I am open to input.


*/
pub mod stats;
pub mod class;
pub mod creature;
//pub mod legendary;
pub mod body;
pub mod types;
pub mod random;
pub mod equation;
pub mod npc;
pub mod special;
pub mod attributes;
pub mod effect;
pub mod item;
