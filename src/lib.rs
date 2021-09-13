/*!
# RPG Stat library
[![Documentation](https://docs.rs/rpg-stat/badge.svg)](https://docs.rs/rpg-stat)
[![Crates.io](https://img.shields.io/crates/v/rpg-stat.svg)](https://crates.io/crates/rpg-stat)

# Stats
The Stats are broken down into categories `Basic`, `Normal`, and `Advanced`

This allows you to run:
```rs:no_run
use crate rpgstat::stats::Basic as Stats
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
use crate rpgstat::class::Basic as Class
// more stuff
fn my_fun () {
    let b = Class::default();
    // whatever
}
```

# Creatures
 This contains all manner of creatures.  We have `Animal` creatures, `Hero` creatures, and more will come!

*/
pub mod stats;
pub mod class;
pub mod creature;

#[cfg(test)]
mod tests {
    use crate::stats::Basic as Stats;
    use crate::class::Basic as Class;
    #[test]
    fn test_basic() {
        let b:Stats = Stats::default();
        assert_eq!(0.0, b.xp);
    }
    #[test]
    fn test_class() {
        let c:Class = Class::default();
        assert_eq!(Class::Enemy, c);
    }
}
