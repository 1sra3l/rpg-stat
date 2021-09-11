/*!
# RPG Stat library
[![Documentation](https://docs.rs/rpg-stat/badge.svg)](https://docs.rs/rpg-stat)
[![Crates.io](https://img.shields.io/crates/v/rpg-stat.svg)](https://crates.io/crates/rpg-stat)

*/
pub mod stats;
pub mod class;

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
