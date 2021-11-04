#[cfg(test)]
mod tests {
    use crate::stats::Basic as Stats;
    use crate::class::Basic as Class;
    use crate::stats::BasicStat as BasicStat;
    use crate::stats::BasicPremade as BasicPremade;
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
        fn level(&self) -> f64 {
            self.stats.level
        }
        fn speed(&self) -> f64 {
            self.stats.speed
        }
        fn gp(&self) -> f64 {
            self.stats.gp
        }
        fn damage(&mut self, amount:f64) {
            let mut val = self.stats.hp;
            val -= amount ;
            self.stats.hp = val;
        }
        fn heal(&mut self, amount:f64) {
            let mut val = self.stats.hp;
            val += amount ;
            self.stats.hp = val;
        }
    }
    pub struct Hero {
        pub name:String,
        pub stats:Stats<f64>,
        pub class:Class,
    }
    impl Hero {
        pub fn empty() -> Self {
            Hero {
                name:String::from(""),
                stats:Stats::default(),
                class:Class::default(),
            }
        }
    }
    impl BasicPremade<f64> for Hero {
        fn stat(&self) -> Stats<f64> {
            self.stats
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
    fn test_basic_class_builder() {
        let b:Stats<f64> = Stats::from_class(0.0, Class::Hero);
        assert_eq!(b.xp_next, 10.0)
    }
    #[test]
    fn trait_test_0() {
        let _c:Character = Character::empty();
    }
    #[test]
    fn trait_test_1() {
        let _c:Hero = Hero::empty();
    }
} 
