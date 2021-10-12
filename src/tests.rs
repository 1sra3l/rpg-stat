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
