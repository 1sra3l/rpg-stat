use rand::{thread_rng, Rng};
pub trait Random {
    type Type;
    fn random_type(&self) -> Self::Type;
    fn random_rate(&self,value:u32) -> u32 {
        let mut rng = thread_rng();
        let n: u32 = rng.gen_range(0..value);
        n
    }
    fn random(&self, min:f64, max:f64) -> f64 {
        let mut rng = thread_rng();
        let n: f64 = rng.gen_range(min..max);
        n
    }
    /// 
    fn half(&self) -> bool {
        if self.random_rate(1) == 1 {
            return true
        }
        false
    }
    /// 
    fn usually(&self) -> bool {
        if self.random_rate(9) > 1 {
            return true
        }
        false
    }
    /// 
    fn often(&self) -> bool {
        if self.random_rate(3) > 0 {
            return true
        }
        false
    }
    /// 
    fn hardly(&self) -> bool {
        if self.random_rate(3) == 0 {
            return true
        }
        false
    }
    /// 
    fn barely(&self) -> bool {
        if self.random_rate(9) == 0 {
            return true
        }
        false
    }
}
pub fn random_0max(value:u32) -> u32 {
    let mut rng = thread_rng();
    let n: u32 = rng.gen_range(0..value);
    n
}
pub fn random_character_name() ->String {
    let max = 8;
    let val = random_0max(max);
    let mut name:String = match val {
        0 => String::from("Ax"),
        1 => String::from("Morph"),
        2 => String::from("Dri"),
        3 => String::from("Cor"),
        4 => String::from("Mig"),
        5 => String::from("Den"),
        7 => String::from("Por"),
        8 => String::from("Bel"),
        _=> String::from("Rust"),
    };
    let val = random_0max(max);
    let name_second:String = match val {
        0 => String::from("atlin"),
        1 => String::from("eleous"),
        2 => String::from("manthus"),
        3 => String::from("timbrle"),
        4 => String::from("darlis"),
        5 => String::from("gerar"),
        7 => String::from("trid"),
        8 => String::from("toph"),
        _=> String::from("ferris"),
    };
    name.push_str(name_second.as_str());
    name
}
pub fn random_creature_name() -> String {
    let max = 8;
    let val = random_0max(max);
    let mut name:String = match val {
        0 => String::from("Muga"),
        1 => String::from("Pory"),
        2 => String::from("Dyno"),
        3 => String::from("Mela"),
        4 => String::from("Ashu"),
        5 => String::from("Deni"),
        7 => String::from("Ega"),
        8 => String::from("Bela"),
        _=> String::from("Raina"),
    };
    let val = random_0max(max);
    let name_second:String = match val {
        0 => String::from("nel"),
        1 => String::from("leous"),
        2 => String::from("dine"),
        3 => String::from("tole"),
        4 => String::from("phous"),
        5 => String::from("ger"),
        7 => String::from("tom"),
        8 => String::from("toph"),
        _=> String::from("fel"),
    };
    name.push_str(name_second.as_str());
    name
}
