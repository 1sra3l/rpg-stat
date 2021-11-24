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
        (n * 0.5).round() / 0.5
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
    //Syllable 1
    let mut name:String = match val {
        0 => String::from("Mu"),
        1 => String::from("Po"),
        2 => String::from("Dy"),
        3 => String::from("Me"),
        4 => String::from("As"),
        5 => String::from("De"),
        7 => String::from("Eg"),
        8 => String::from("Be"),
        _=> String::from("Rai"),
    };
    //Syllable 2
    let val = random_0max(max);
    let name_second:String = match val {
        0 => String::from("ga"),
        1 => String::from("ry"),
        2 => String::from("no"),
        3 => String::from("la"),
        4 => String::from("hu"),
        5 => String::from("ni"),
        7 => String::from("ba"),
        8 => String::from("la"),
        _=> String::from("na"),
    };
    name.push_str(name_second.as_str());
    //Syllable 3
    let val = random_0max(max);
    let name_second:String = match val {
        0 => String::from("ni"),
        1 => String::from("le"),
        2 => String::from("di"),
        3 => String::from("to"),
        4 => String::from("phu"),
        5 => String::from("go"),
        7 => String::from("tto"),
        8 => String::from(""),
        _=> String::from(""),
    };
    name.push_str(name_second.as_str());
    //Syllable 4
    let val = random_0max(max);
    let name_second:String = match val {
        0 => String::from("l"),
        1 => String::from("s"),
        2 => String::from("ne"),
        3 => String::from("le"),
        4 => String::from(""),
        5 => String::from("ar"),
        7 => String::from("om"),
        8 => String::from("ph"),
        _=> String::from("te"),
    };
    name.push_str(name_second.as_str());
    name
}
