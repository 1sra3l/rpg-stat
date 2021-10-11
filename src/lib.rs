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

#[cfg(test)]
mod tests {
    use crate::stats::Basic as Stats;
    use crate::class::Basic as Class;
    use crate::stats::BasicStat as BasicStat;
    // used in tests below
    pub struct Character {
        pub name:String,
        pub stats:Stats<f64>,
        pub class:Class,
    }
    impl Character {
        pub fn empty() -> Self {
            Character {
                name:String::from(""),
                stats:Stats::default(),
                class:Class::default(),
            }
        }
    }
    impl BasicStat<f64> for Character {
        fn hp(&self) -> f64 {
            self.stats.hp
        }
        fn mp(&self) -> f64 {
            self.stats.mp
        }
        fn xp(&self) -> f64 {
            self.stats.xp
        }
        fn hp_max(&self) -> f64 {
            self.stats.hp_max
        }
        fn mp_max(&self) -> f64 {
            self.stats.mp_max
        }
        fn xp_next(&self) -> f64 {
            self.stats.xp_next
        }
        fn damage(&mut self, amount:f64) {
            let mut val = self.stats.hp;
            val -= amount ;
            self.stats.hp = val;
        }
    }

    #[test]
    fn test_basic() {
        let b:Stats<f64> = Stats::default();
        assert_eq!(0.0, b.xp);
    }
    #[test]
    fn test_class() {
        let c:Class = Class::default();
        assert_eq!(Class::Enemy, c);
    }
    #[test]
    fn trait_test() {
        let c:Character = Character::empty();
    }
}
