#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "fltkform", derive(FltkForm))]
/*  # Advanced

*/
pub enum Advanced {
    /// Painite - Borate, Calcium, Zirconium, Boron, Aluminium  mineral
    Painite,
    /// Hibonite - Calcium, Aluminium, Magnesium, Cerium, Titanium, 
    Hibonite,
    /// RedBeryl - Beryllium, Aluminium, Silicon
    RedBeryl,
    /// Jeremejevite - Aluminium, Boron
    Jeremejevite,
    /// Chambersite - Manganese, Boron
    Chambersite,
    /// Musgravite - 
    Musgravite,
    /// Grandidirite - 
    Grandidirite,
    /// Poudretteite - 
    Poudretteite,
    /// Serendibite - 
    Serendibite,
    /// Zektzerite - 
    Zektzerite,
    /// Nothing
    None,
}
impl Default for Advanced {
    fn default() -> Self {
        Self::None
    }
}
impl fmt::Display for Advanced {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v:String;
        match *self {
            Advanced::Painite => v = String::from("Painite"),
            Advanced::Hibonite => v = String::from("Hibonite"),
            Advanced::RedBeryl => v = String::from("RedBeryl"),
            Advanced::Jeremejevite => v = String::from("Jeremejevite"),
            Advanced::Chambersite => v = String::from("Chambersite"),
            Advanced::Musgravite => v = String::from("Musgravite"),
            Advanced::Grandidirite => v = String::from("Grandidirite"),
            Advanced::Poudretteite => v = String::from("Poudretteite"),
            Advanced::Serendibite => v = String::from("Serendibite"),
            Advanced::Zektzerite => v = String::from("Zektzerite"),
            _=> v = String::from("None"),
        }
        write!(f, "{}", v.as_str())
    }
}
impl Random for Advanced {
    type Type = Advanced;
    fn random_type(&self) -> Self::Type {
        let max = 10;
        let val = self.random_rate(max);
        match val {
            0 => Advanced::Painite,
            1 => Advanced::Hibonite,
            2 => Advanced::RedBeryl,
            3 => Advanced::Jeremejevite,
            4 => Advanced::Chambersite,
            5 => Advanced::Musgravite,
            6 => Advanced::Grandidirite,
            7 => Advanced::Poudretteite,
            8 => Advanced::Serendibite,
            9 => Advanced::Zektzerite,
            _=> Advanced::None,
        }
    }
    
}
