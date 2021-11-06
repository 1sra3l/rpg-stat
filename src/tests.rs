#[cfg(test)]
mod tests {
    use crate::stats::Builder;
    use crate::stats::Basic as Stats;
    use crate::stats::Normal as StatsNormal;
    use crate::class::Basic as Class;
    use crate::stats::BasicPremade as BasicPremade;
    use crate::stats::NormalPremade as NormalPremade;
    use crate::legendary::Legendary;
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
    impl BasicPremade<f64> for Character {
        fn stat(&self) -> Stats<f64> {
            self.stats
        }
        fn set_hp(&mut self, amount:f64) {
            self.stats.hp = amount
        }
        fn set_mp(&mut self, amount:f64) {
            self.stats.mp = amount
        }
        fn set_xp(&mut self, amount:f64) {
            self.stats.xp = amount
        }
        fn set_hp_max(&mut self, amount:f64) {
            self.stats.hp = amount
        }
        fn set_mp_max(&mut self, amount:f64) {
            self.stats.mp = amount
        }
        fn set_xp_next(&mut self, amount:f64) {
            self.stats.xp_next = amount
        }
        fn set_gp(&mut self, amount:f64) {
            self.stats.gp = amount
        }
    }
    pub struct Player {
        pub name:String,
        pub stats:StatsNormal<f64>,
        pub class:Class,
    }
    impl Player {
        pub fn empty() -> Self {
            Player {
                name:String::from(""),
                stats:StatsNormal::default(),
                class:Class::default(),
            }
        }
    }
    impl NormalPremade<f64> for Player {
        fn stat(&self) -> StatsNormal<f64> {
            self.stats
        }
        fn set_hp(&mut self, amount:f64) {
            self.stats.hp = amount
        }
        fn set_mp(&mut self, amount:f64) {
            self.stats.mp = amount
        }
        fn set_xp(&mut self, amount:f64) {
            self.stats.xp = amount
        }
        fn set_hp_max(&mut self, amount:f64) {
            self.stats.hp = amount
        }
        fn set_mp_max(&mut self, amount:f64) {
            self.stats.mp = amount
        }
        fn set_xp_next(&mut self, amount:f64) {
            self.stats.xp_next = amount
        }
        fn set_gp(&mut self, amount:f64) {
            self.stats.gp = amount
        }
        fn set_atk(&mut self, amount:f64) {
            self.stats.atk = amount
        }
        fn set_m_atk(&mut self, amount:f64) {
            self.stats.m_atk = amount
        }
         fn set_def(&mut self, amount:f64) {
            self.stats.def = amount
        }
        fn set_m_def(&mut self, amount:f64) {
            self.stats.m_def = amount
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
        //TODO breaking change
        let b:Stats<f64> = Stats::from_class(0.0, Class::Hero);
        assert_eq!(b.xp_next, 10.0)
    }
    #[test]
    fn trait_test_0() {
        let _c:Character = Character::empty();
    }
    #[test]
    fn fight_test_0() {
        let player:Character = Character::empty();
        let mut enemy:Character = Character::empty();
        // enemy has 5 hp
        //TODO breaking change
        enemy.stats = Stats::from_class(1.0, enemy.class);
        assert_eq!(enemy.hp_max(), 5.0);
        // now it has one less
        enemy.damage(1.0);
        assert_eq!(enemy.hp(), 4.0);
        enemy.heal(5.0);
        assert_eq!(enemy.hp(), 5.0);
    }
    #[test]
    fn fight_test_1() {
        let mut player:Player = Player::empty();
        let mut enemy:Character = Character::empty();
        // enemy has 5 hp
        //TODO breaking change
        enemy.stats = Stats::from_class(1.0, enemy.class);
        assert_eq!(enemy.hp_max(), 5.0);
        player.set_atk(1.0);
        // now it has one less
        enemy.damage(player.atk());
        assert_eq!(enemy.hp(), 4.0);
        enemy.heal(5.0);
        assert_eq!(enemy.hp(), 5.0);
    }
    #[test]
    fn legendary_builder() {
        let sc:Legendary = Legendary::SantaClaus;
        let stats:Stats<f64> = sc.build_basic(0.0,1.0);
        assert_eq!(stats.hp,10.0);
    }
} 
