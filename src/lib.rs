/*!
# RPG Stat library

[![Documentation](https://docs.rs/rpg-stat/badge.svg)](https://docs.rs/rpg-stat)
[![Crates.io](https://img.shields.io/crates/v/rpg-stat.svg)](https://crates.io/crates/rpg-stat)

# Stats
The Stats are broken down into categories `Basic`, `Normal`, and `Advanced`

`Basic` contains only the most needed for a generic game
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
 * atk
 * def
 * m_atk
 * m_def

`Advanced` contains the finer details seen in tabletop RPG stats as well as everything in `Normal` and `Basic`
 * agility
 * strength
 * dexterity
 * constitution
 * intelligence
 * charisma
 * wisdom
 * age


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

// get bear stats for our program
fn bear_stats () -> Stats<f64> {
    // make the bear enum
    let bear:Animal = Animal::Bear;
    // this number only matters if you want
    let id:f64 = 0;
    // this effects the stats returned
    let level:f64 = 1;
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

```
use crate::stats::Basic as BasicStats;
use crate::stats::Normal as NormalStats;
use crate::stats::Advanced as AdvancedStats;
use crate::stats::Builder;

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
            }
        }
    }
    fn build_normal(&self, id:T, level:T) -> NormalStats<T>{
        match *self {
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
            }
        }
    }
    fn build_advanced(&self, id:T, level:T) -> AdvancedStats<T>{
        match *self {
            // make advanced
            // TODO make Tux destroy the other characters stats
            // well maybe not Pepper since she gives out free paint brushes...
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
            }
        }
    }

```


# Classes

The Classes are broken down into categories `Basic`, `Normal`, and `Advanced`

The `Basic` class is either `Hero` or `Enemy`

The `Normal` class includes a range of character classes for a battle game.

`Advanced` includes more characters for a game with interactive roles, not simply a game of battle.

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
    let hero_stats:Stats<f64> = hero.build_basic(id, level);
    // that was easy!
    hero_stats
}

```

# Creatures
 This contains all manner of creatures.  We have `Animal` creatures, `Person` creatures, and even `Monster` creatures

I would like to implement `Builder` for all the enums

So far `Animal` is complete.
 
# Legendary
This contains the basics to use any creature from [Wikipedia's Legendary Creatures](https://en.wikipedia.org/wiki/Lists_of_legendary_creatures) and create `Basic`, `Normal` or `Advanced` stats.
```
let sc:Legendary = Legendary::SantaClaus;
let stats:Stats<f64> = sc.build_basic(0.0,1.0);
assert_eq!(stats.hp, 10.0);
```
Eventually I would like to curate unique stats for each creature and have a full `long_description()` available for every creature.

As it stands you can still battle the `ToothFairy` against the `EasterBilby`, they will just have the exact same stats to start.  Feel free to modify them.

The goal will be to move all the information into `legendary.ini` to be read in, but there are well over 1000 creatures.

# Types

General abstractions for Element, Special, and Effect.

## Element

These are Elements similar to what you'd find in any game with type differences.  The function `opposite_from_type()` can be used to find weakness/strength

## Special

These are names of `Special` moves.  There is also an `mp_cost()` calculator for the special.

```
let grind:Special = Special::Grind;
assert_eq!(grind.mp_cost(),7.0);

```

## Effect

The goal with effect will be to operate on stats, and likely implement a `Builder` to make it easy.

## Stage
This includes the `Stage<T>` of life.  This is similar to things like "evolution" in creature raising games, but based on reality.  In real life no creature evolves randomly in front of someone, however they do get older and change their "form".  There are eight forms:
 * Baby
 * Toddler
 * Kid
 * Teen
 * Young
 * Grown
 * Older
 * Old
I intend to make a simpler version of this, and may move this to a separate file

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
pub mod legendary;
pub mod body;
