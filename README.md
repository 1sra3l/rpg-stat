# RPG Stat library

[![Documentation](https://docs.rs/rpg-stat/badge.svg)](https://docs.rs/rpg-stat)
[![Crates.io](https://img.shields.io/crates/v/rpg-stat.svg)](https://crates.io/crates/rpg-stat)

Cargo.toml

`rpgstat="2.0"`

This is fairly exhaustive and links to most things you can use.
The library is **stil a WIP** as the battle system is rudimentary in the current form.  [TOML](https://crates.io/crates/toml) format with [serde](https://crates.io/crates/serde) is supported.


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

## Serde + TOML/INI

Yes you can use serde with any of the assets/characters/ files provided.  You can use them in your custom structs.
You can implement Premade Stats to have `mystruct.hp()` instead of something like below `mystruct.stats.hp`


```rs
use std::fs::File;
use std::io::Read;
use toml::*;
use serde::{Deserialize, Serialize};
use rpgstat::legendary::Legendary;
use rpgstat::stats::Basic as Stats;
use rpgstat::stats::Builder;

#[derive(Serialize, Deserialize)]
pub struct Character {
    pub name:String,
    pub stats:Stats<f64>,
}
let filename = "assets/characters/EasterBilby.ini";
    match File::open(filename) {
    Ok(mut file) => {
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        let decoded: Stats<f64> = toml::from_str(content.as_str()).unwrap();
        assert_eq!(decoded.hp, 10.0);
        let decoded2: Character = toml::from_str(content.as_str()).unwrap();
        assert_eq!(decoded2.stats.hp, 10.0);
        let sc:Legendary = Legendary::SantaClaus;
        let stats:Stats<f64> = sc.build_basic(0.0,1.0);
        let toml = toml::to_string(&stats).unwrap();

    },
    Err(e) => println!("Error:{} opening File:{}", e, filename),
}
```

## Custom toml/ini with serde

```rs
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
```rs
// feel free to use `Normal` or `Advanced` instead of `Basic`
use rpgstat::stats::Basic as Stats;
use rpgstat::class::Basic as Class;
use rpgstat::creature::Animal;
// this is the thing we need!
use rpgstat::stats::Builder;

// get bear stats for our program
fn bear_stats () -> Stats<f64> {
    // make the bear enum
    let bear:Animal = Animal::Bear;
    // this number only matters if you want
    let id:f64 = 0.0;
    // this effects the stats returned
    let level:f64 = 1.0;
    // use the basic `Builder`
    let bear_stats:Stats<f64> = bear.build_basic(id, level);
    // that was easy!
    bear_stats
}

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

```rs
use rpgstat::stats::Basic as BasicStats;
use rpgstat::stats::Normal as NormalStats;
use rpgstat::stats::Advanced as AdvancedStats;
use rpgstat::stats::Builder;
use std::ops::{Add, AddAssign,  Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign};

// gee maybe I should make stats for these awesome libre characters
pub enum MyAwesomeThing {
    Tux,
    Pepper,
    Gnu,
    Kiki,
    Sara,
    Konqi,
    Suzanne,
    Xue,
    Wilber,
    Pidgin,
}

impl<T:Copy 
    + Default
    + AddAssign
    + Add<Output = T>
    + Div<Output = T>
    + DivAssign
    + Mul<Output = T>
    + MulAssign
    + Neg<Output = T>
    + Rem<Output = T>
    + RemAssign
    + Sub<Output = T>
    + SubAssign
    + std::cmp::PartialOrd
    + num::NumCast> Builder<T> for MyAwesomeThing {
    /// Build a `Basic` stat
    fn build_basic(&self, id:T, level:T) -> BasicStats<T>{
        match *self {
            // make basic
            _=>
            BasicStats {
                id: Default::default(),
                xp: Default::default(),
                xp_next: Default::default(),
                level: Default::default(),
                gp: Default::default(),
                hp: Default::default(),
                mp: Default::default(),
                hp_max: Default::default(),
                mp_max: Default::default(),
                speed: Default::default(),
            },
        }
    }
    fn build_normal(&self, id:T, level:T) -> NormalStats<T>{
        match *self {
            _=>
            // make normal
            NormalStats {
                id: Default::default(),
                xp: Default::default(),
                xp_next: Default::default(),
                level: Default::default(),
                gp: Default::default(),
                hp: Default::default(),
                mp: Default::default(),
                hp_max: Default::default(),
                mp_max: Default::default(),
                speed: Default::default(),
                atk:Default::default(),
                def:Default::default(),
                m_atk:Default::default(),
                m_def:Default::default(),
            },
        }
    }
    fn build_advanced(&self, id:T, level:T) -> AdvancedStats<T>{
        match *self {
            // make advanced
            // TODO make Tux destroy the other characters stats
            // well maybe not Pepper since she gives out free paint brushes...
            _=>
            AdvancedStats {
                id: Default::default(),
                xp: Default::default(),
                xp_next: Default::default(),
                level: Default::default(),
                gp: Default::default(),
                hp: Default::default(),
                mp: Default::default(),
                hp_max: Default::default(),
                mp_max: Default::default(),
                speed: Default::default(),
                atk:Default::default(),
                def:Default::default(),
                m_atk:Default::default(),
                m_def:Default::default(),
                agility:Default::default(),
                strength:Default::default(),
                dexterity:Default::default(),
                constitution:Default::default(),
                intelligence:Default::default(),
                charisma:Default::default(),
                wisdom:Default::default(),
                age:Default::default(),
            },
        }
    }
}
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

```rs
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
 This contains all manner of creatures.  We have `Animal` creatures, `Person` creatures, and even `Monster` creatures

I have implemented `Builder` for `Animal`
 
# Legendary
This contains the basics to use any creature from [Wikipedia's Legendary Creatures](https://en.wikipedia.org/wiki/Lists_of_legendary_creatures) and create `Basic`, `Normal` or `Advanced` stats.
```rs
use rpgstat::legendary::Legendary;
// we want basic stats
use rpgstat::stats::Basic as Stats;
// this is the thing we need!
use rpgstat::stats::Builder;
let sc:Legendary = Legendary::SantaClaus;
// remember build_*(id,level); if you are copy/paste XD
let stats:Stats<f64> = sc.build_basic(0.0,1.0);
assert_eq!(stats.hp, 10.0);
```
Eventually I would like to curate unique stats for each creature and have a full `long_description()` available for every creature.  This may change, however.

As it stands you can still battle the `ToothFairy` against the `EasterBilby`, they will just have the exact same stats to start.  Feel free to modify them.

The goal will be to read all the information from the individual files *eventually*.

# Types

This includes various enums related to the type of character you have

`use rpgstat::types::Basic as Type`

 * `Basic` is the basic type `Good` or `Bad`
 * `Normal` has elemental types
 * `Advanced` has elemental types

## Compare

The Compare trait is implemented for `Normal`
according to this chart:
 
```rs
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

```rs
use rpgstat::special::Normal as Special;
let grind:Special = Special::Grind;
```

You can get the prebuilt `mp_cost()`:
```rs
use rpgstat::special::ManaCost;
use rpgstat::special::Normal as Special;
let grind:Special = Special::Grind;
assert_eq!(grind.mp_cost(0), 7);
```

# Effect
This composes the various Effects in-game related to a character's Stats


# Attributes

These are definitions of abstract terms into code

## Rate
Rate of occurance
```rs
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
```rs
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

```rs
use rpgstat::attributes::{Effectiveness, Value};
let hp:i32 = 50;
// later on we use an item and check the effectiveness of it
assert_eq!(Effectiveness::Half.value(hp), 25);

```

## Stage

```rs
use rpgstat::attributes::Stage;

```
This includes the `Stage<T>` of life.  This is similar to things like "evolution" in creature raising games, but based on reality.  In real life no creature evolves randomly in front of someone, however they do get older and change their "form".  There are eight forms:
 * Baby
 * Toddler
 * Kid
 * Teen
 * Young
 * Grown
 * Older
 * Old


```rs
use rpgstat::attributes::Stage;
let stage:Stage<i32> = Stage::current(15);
// Yup 15 is teen
assert_eq!(stage, Stage::Teen(15));

```

# Body
This is to collect all the information about armor, stats, status, etc, based on each body part.  This will be some serious numeric control over the simulation.


# RPG Stat command line tool
 WIP
 Ideally, this will use `clap` and support some very specific stat traits only.

 AFAIK the interface will end up looking like:
 `rpgstat class normal Archer stat normal hp`
 
 That said none of the work has been started yet, and I am open to input.
