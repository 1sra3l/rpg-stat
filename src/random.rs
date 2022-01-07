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
#[allow(unused)]
pub fn rand(min:f64, max:f64) -> f64 {
    let mut rng = thread_rng();
    let n:f64 = rng.gen_range(min..max);
    n
}
#[allow(unused)]
pub fn random_0max(value:u32) -> u32 {
    let mut rng = thread_rng();
    let n: u32 = rng.gen_range(0..value);
    n
}
#[allow(unused)]
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
#[allow(unused)]
pub fn random_creature_name() -> String {
    let max = 8;
    let val = random_0max(max);
    //Syllable 1
    let mut name:String = match val {
        0 => String::from("Arc"),
        1 => String::from("Molt"),
        2 => String::from("Dynar"),
        3 => String::from("Megas"),
        4 => String::from("Gibor"),
        5 => String::from("Dend"),
        7 => String::from("Egal"),
        8 => String::from("Bend"),
        _=> String::from("Apt"), // yes i use ubuntu/debian/variants sometimes
    };
    //Syllable 2
    let val = random_0max(max);
    let name_second:String = match val {
        0 => String::from("ion"),
        1 => String::from("acron"),
        2 => String::from("os"),
        3 => String::from("aya"),
        4 => String::from("on"),
        5 => String::from("ine"),
        7 => String::from("aur"),// yes i use arch sometimes
        8 => String::from("ate"),
        _=> String::from("ay"),
    };
    name.push_str(name_second.as_str());
    name
}
