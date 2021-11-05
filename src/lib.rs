/*!
# RPG Stat library
[![Documentation](https://docs.rs/rpg-stat/badge.svg)](https://docs.rs/rpg-stat)
[![Crates.io](https://img.shields.io/crates/v/rpg-stat.svg)](https://crates.io/crates/rpg-stat)

# Stats
The Stats are broken down into categories `Basic`, `Normal`, and `Advanced`

This allows you to run:
```rs:no_run
use rpgstat::stats::Basic as Stats
// more stuff
fn my_fun () {
    let b = Stats::default();
    // whatever
}
```

# Classes

The Classes are broken down into categories `Basic`, `Normal`, and `Advanced`

This allows you to run:
```rs:no_run
use rpgstat::class::Basic as Class
// more stuff
fn my_fun () {
    let b = Class::default();
    // whatever
}
```

# Creatures
 This contains all manner of creatures.  We have `Animal` creatures, `Hero` creatures, and more to come!

*/
pub mod stats;
pub mod class;
pub mod creature;
pub mod legendary;
pub mod body;
