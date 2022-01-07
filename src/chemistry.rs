/*!
# 

## Periodic

```
use rpg_stat::chemistry::Periodic;
// 
let periodic = Periodic::None;
assert_eq!(periodic, Periodic::default());
```

*/
use std::fmt;
use crate::random::Random;
use serde::{Deserialize, Serialize};
#[cfg(feature = "fltkform")]
use fltk::{prelude::*, *};
#[cfg(feature = "fltkform")]
use fltk_form_derive::*;
#[cfg(feature = "fltkform")]
use fltk_form::FltkForm;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "fltkform", derive(FltkForm))]
/*# Polarity

*/
pub enum Polarity {
    /// Polar bonds generally occur when the difference in electronegativity between the two atoms is roughly between 0.5 and 2.0
    Polar,
    /// Ionic bonds generally occur when the difference in electronegativity between the two atoms is greater than 2.0
    Ionic,
    /// Nonpolar bonds generally occur when the difference in electronegativity between the two atoms is less than 0.5
    None,
}
impl Default for Polarity {
    fn default() -> Self {
        Self::None
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "fltkform", derive(FltkForm))]
/*# Bonds

*/ 
pub enum Bonds {
    Covalent,
    Ionic,
    Metallic,
    CoordinateCovalent,
    None,
}
impl Default for Bonds {
    fn default() -> Self {
        Self::None
    }
}
/*# Molecule

molecules held together by covalent bonds
*/
pub struct Molecule {
    
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "fltkform", derive(FltkForm))]
/*# Compound

A [chemical compound](https://en.wikipedia.org/wiki/Chemical_compound) is a chemical substance composed of many identical molecules (or molecular entities) composed of atoms from more than one element held together by chemical bonds. A molecule consisting of atoms of only one element is therefore not a compound.
*/
pub struct Compound {
    // 
    pub atoms:Vec<Periodic>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "fltkform", derive(FltkForm))]
/*# Periodic

*/
pub enum Periodic {
    /// 
    Actinium,
    /// 
    Aluminum,
    /// 
    Americium,
    /// 
    Antimony,
    /// 
    Argon,
    /// 
    Arsenic,
    /// 
    Astatine,
    /// 
    Barium,
    /// 
    Berkelium,
    /// 
    Beryllium,
    /// 
    Bismuth,
    /// 
    Bohrium,
    /// 
    Boron,
    /// 
    Bromine,
    /// 
    Cadmium,
    /// 
    Calcium,
    /// 
    Californium,
    /// 
    Carbon,
    /// 
    Cerium,
    /// 
    Cesium,
    /// 
    Chlorine,
    /// 
    Chromium,
    /// 
    Cobalt,
    /// 
    Copernicium,
    /// 
    Copper,
    /// 
    Curium,
    /// 
    Darmstadtium,
    /// 
    Dubnium,
    /// 
    Dysprosium,
    /// 
    Einsteinium,
    /// 
    Erbium,
    /// 
    Europium,
    /// 
    Fermium,
    /// 
    Flerovium,
    /// 
    Fluorine,
    /// 
    Francium,
    /// 
    Gadolinium,
    /// 
    Gallium,
    /// 
    Germanium,
    /// 
    Gold,
    /// 
    Hafnium,
    /// 
    Hassium,
    /// 
    Helium,
    /// 
    Holmium,
    /// 
    Hydrogen,
    /// 
    Indium,
    /// 
    Iodine,
    /// 
    Iridium,
    /// 
    Iron,
    /// 
    Krypton,
    /// 
    Lanthanum,
    /// 
    Lawrencium,
    /// 
    Lead,
    /// 
    Lithium,
    /// 
    Livermorium,
    /// 
    Lutetium,
    /// 
    Magnesium,
    /// 
    Moscovium,
    /// 
    Manganese,
    /// 
    Meitnerium,
    /// 
    Mendelevium,
    /// 
    Mercury,
    /// 
    Molybdenum,
    /// 
    Neilsborium,
    /// 
    Neodymium,
    /// 
    Neon,
    /// 
    Neptunium,
    /// 
    Nihonium,
    /// 
    Nickel,
    /// 
    Niobium,
    /// 
    Nitrogen,
    /// 
    Nobelium,
    /// 
    Oganesson,
    /// 
    Osmium,
    /// 
    Oxygen,
    /// 
    Palladium,
    /// 
    Phosphorus,
    /// 
    Platinum,
    /// 
    Plutonium,
    /// 
    Polonium,
    /// 
    Praseodymium,
    /// 
    Potassium,
    /// 
    Promethium,
    /// 
    Protactinium,
    /// 
    Radium,
    /// 
    Radon,
    /// 
    Rhenium,
    /// 
    Rhodium,
    /// 
    Roentgenium,
    /// 
    Rubidium,
    /// 
    Ruthenium,
    /// 
    Rutherfordium,
    /// 
    Samarium,
    /// 
    Scandium,
    /// 
    Seaborgium,
    /// 
    Selenium,
    /// 
    Silicon,
    /// 
    Silver,
    /// 
    Sodium,
    /// 
    Strontium,
    /// 
    Sulfur,
    /// 
    Tantalum,
    /// 
    Technetium,
    /// 
    Tellurium,
    /// 
    Tennessine,
    /// 
    Terbium,
    /// 
    Thallium,
    /// 
    Thorium,
    /// 
    Thulium,
    /// 
    Tin,
    /// 
    Titanium,
    /// 
    Tungsten,
    /// 
    Uranium,
    /// 
    Vanadium,
    /// 
    Xenon,
    /// 
    Ytterbium,
    /// 
    Yttrium,
    /// 
    Zinc,
    /// 
    Zirconium,
    /// Nothing
    None,
}
impl Periodic {
    /// A list of all items (except Periodic::None)
    pub fn periodics() -> Vec<Periodic> {
        vec![
            Periodic::Actinium,
            Periodic::Aluminum,
            Periodic::Americium,
            Periodic::Antimony,
            Periodic::Argon,
            Periodic::Arsenic,
            Periodic::Astatine,
            Periodic::Barium,
            Periodic::Berkelium,
            Periodic::Beryllium,
            Periodic::Bismuth,
            Periodic::Bohrium,
            Periodic::Boron,
            Periodic::Bromine,
            Periodic::Cadmium,
            Periodic::Calcium,
            Periodic::Californium,
            Periodic::Carbon,
            Periodic::Cerium,
            Periodic::Cesium,
            Periodic::Chlorine,
            Periodic::Chromium,
            Periodic::Cobalt,
            Periodic::Copernicium,
            Periodic::Copper,
            Periodic::Curium,
            Periodic::Darmstadtium,
            Periodic::Dubnium,
            Periodic::Dysprosium,
            Periodic::Einsteinium,
            Periodic::Erbium,
            Periodic::Europium,
            Periodic::Fermium,
            Periodic::Flerovium,
            Periodic::Fluorine,
            Periodic::Francium,
            Periodic::Gadolinium,
            Periodic::Gallium,
            Periodic::Germanium,
            Periodic::Gold,
            Periodic::Hafnium,
            Periodic::Hassium,
            Periodic::Helium,
            Periodic::Holmium,
            Periodic::Hydrogen,
            Periodic::Indium,
            Periodic::Iodine,
            Periodic::Iridium,
            Periodic::Iron,
            Periodic::Krypton,
            Periodic::Lanthanum,
            Periodic::Lawrencium,
            Periodic::Lead,
            Periodic::Lithium,
            Periodic::Livermorium,
            Periodic::Lutetium,
            Periodic::Magnesium,
            Periodic::Moscovium,
            Periodic::Manganese,
            Periodic::Meitnerium,
            Periodic::Mendelevium,
            Periodic::Mercury,
            Periodic::Molybdenum,
            Periodic::Neilsborium,
            Periodic::Neodymium,
            Periodic::Neon,
            Periodic::Neptunium,
            Periodic::Nihonium,
            Periodic::Nickel,
            Periodic::Niobium,
            Periodic::Nitrogen,
            Periodic::Nobelium,
            Periodic::Oganesson,
            Periodic::Osmium,
            Periodic::Oxygen,
            Periodic::Palladium,
            Periodic::Phosphorus,
            Periodic::Platinum,
            Periodic::Plutonium,
            Periodic::Polonium,
            Periodic::Praseodymium,
            Periodic::Potassium,
            Periodic::Promethium,
            Periodic::Protactinium,
            Periodic::Radium,
            Periodic::Radon,
            Periodic::Rhenium,
            Periodic::Rhodium,
            Periodic::Roentgenium,
            Periodic::Rubidium,
            Periodic::Ruthenium,
            Periodic::Rutherfordium,
            Periodic::Samarium,
            Periodic::Scandium,
            Periodic::Seaborgium,
            Periodic::Selenium,
            Periodic::Silicon,
            Periodic::Silver,
            Periodic::Sodium,
            Periodic::Strontium,
            Periodic::Sulfur,
            Periodic::Tantalum,
            Periodic::Technetium,
            Periodic::Tellurium,
            Periodic::Tennessine,
            Periodic::Terbium,
            Periodic::Thallium,
            Periodic::Thorium,
            Periodic::Thulium,
            Periodic::Tin,
            Periodic::Titanium,
            Periodic::Tungsten,
            Periodic::Uranium,
            Periodic::Vanadium,
            Periodic::Xenon,
            Periodic::Ytterbium,
            Periodic::Yttrium,
            Periodic::Zinc,
            Periodic::Zirconium,
            ]
    }
    /// Get the atomic number of some periodic element
    pub fn get_atomic_number(periodic:Periodic) -> f64 {
        periodic.atomic_number()
    }
    /// The atomic number of the periodic element
    pub fn atomic_number(&self) -> f64 {
        match *self {
           Periodic::Actinium =>  89.0,
           Periodic::Aluminum =>  13.0,
           Periodic::Americium =>  95.0,
           Periodic::Antimony =>  51.0,
           Periodic::Argon =>  18.0,
           Periodic::Arsenic =>  33.0,
           Periodic::Astatine =>  85.0,
           Periodic::Barium =>  56.0,
           Periodic::Berkelium =>  97.0,
           Periodic::Beryllium =>  4.0,
           Periodic::Bismuth =>  83.0,
           Periodic::Bohrium =>  107.0,
           Periodic::Boron =>  5.0,
           Periodic::Bromine =>  35.0,
           Periodic::Cadmium =>  48.0,
           Periodic::Calcium =>  20.0,
           Periodic::Californium =>  98.0,
           Periodic::Carbon =>  6.0,
           Periodic::Cerium =>  58.0,
           Periodic::Cesium =>  55.0,
           Periodic::Chlorine =>  17.0,
           Periodic::Chromium =>  24.0,
           Periodic::Cobalt =>  27.0,
           Periodic::Copernicium =>  112.0,
           Periodic::Copper =>  29.0,
           Periodic::Curium =>  96.0,
           Periodic::Darmstadtium =>  110.0,
           Periodic::Dubnium =>  105.0,
           Periodic::Dysprosium =>  66.0,
           Periodic::Einsteinium =>  99.0,
           Periodic::Erbium =>  68.0,
           Periodic::Europium =>  63.0,
           Periodic::Fermium =>  100.0,
           Periodic::Flerovium =>  114.0,
           Periodic::Fluorine =>  9.0,
           Periodic::Francium =>  87.0,
           Periodic::Gadolinium =>  64.0,
           Periodic::Gallium =>  31.0,
           Periodic::Germanium =>  32.0,
           Periodic::Gold =>  79.0,
           Periodic::Hafnium =>  72.0,
           Periodic::Hassium =>  108.0,
           Periodic::Helium =>  2.0,
           Periodic::Holmium =>  67.0,
           Periodic::Hydrogen =>  1.0,
           Periodic::Indium =>  49.0,
           Periodic::Iodine =>  53.0,
           Periodic::Iridium =>  77.0,
           Periodic::Iron =>  26.0,
           Periodic::Krypton =>  36.0,
           Periodic::Lanthanum =>  57.0,
           Periodic::Lawrencium =>  103.0,
           Periodic::Lead =>  82.0,
           Periodic::Lithium =>  3.0,
           Periodic::Livermorium =>  116.0,
           Periodic::Lutetium =>  71.0,
           Periodic::Magnesium =>  12.0,
           Periodic::Moscovium =>  115.0,
           Periodic::Manganese =>  25.0,
           Periodic::Meitnerium =>  109.0,
           Periodic::Mendelevium =>  101.0,
           Periodic::Mercury =>  80.0,
           Periodic::Molybdenum =>  42.0,
           Periodic::Neilsborium =>  107.0,
           Periodic::Neodymium =>  60.0,
           Periodic::Neon =>  10.0,
           Periodic::Neptunium =>  93.0,
           Periodic::Nihonium =>  113.0,
           Periodic::Nickel =>  28.0,
           Periodic::Niobium =>  41.0,
           Periodic::Nitrogen =>  7.0,
           Periodic::Nobelium =>  102.0,
           Periodic::Oganesson =>  118.0,
           Periodic::Osmium =>  76.0,
           Periodic::Oxygen =>  8.0,
           Periodic::Palladium =>  46.0,
           Periodic::Phosphorus =>  15.0,
           Periodic::Platinum =>  78.0,
           Periodic::Plutonium =>  94.0,
           Periodic::Polonium =>  84.0,
           Periodic::Praseodymium =>  59.0,
           Periodic::Potassium =>  19.0,
           Periodic::Promethium =>  61.0,
           Periodic::Protactinium =>  91.0,
           Periodic::Radium =>  88.0,
           Periodic::Radon =>  86.0,
           Periodic::Rhenium =>  75.0,
           Periodic::Rhodium =>  45.0,
           Periodic::Roentgenium =>  111.0,
           Periodic::Rubidium =>  37.0,
           Periodic::Ruthenium =>  44.0,
           Periodic::Rutherfordium =>  104.0,
           Periodic::Samarium =>  62.0,
           Periodic::Scandium =>  21.0,
           Periodic::Seaborgium =>  106.0,
           Periodic::Selenium =>  34.0,
           Periodic::Silicon =>  14.0,
           Periodic::Silver =>  47.0,
           Periodic::Sodium =>  11.0,
           Periodic::Strontium =>  38.0,
           Periodic::Sulfur =>  16.0,
           Periodic::Tantalum =>  73.0,
           Periodic::Technetium =>  43.0,
           Periodic::Tellurium =>  52.0,
           Periodic::Tennessine =>  117.0,
           Periodic::Terbium =>  65.0,
           Periodic::Thallium =>  81.0,
           Periodic::Thorium =>  90.0,
           Periodic::Thulium =>  69.0,
           Periodic::Tin =>  50.0,
           Periodic::Titanium =>  22.0,
           Periodic::Tungsten =>  74.0,
           Periodic::Uranium =>  92.0,
           Periodic::Vanadium =>  23.0,
           Periodic::Xenon =>  54.0,
           Periodic::Ytterbium =>  70.0,
           Periodic::Yttrium =>  39.0,
           Periodic::Zinc =>  30.0,
           Periodic::Zirconium =>  40.0,
           _=> 0.0,
        }
    }
    pub fn abreviation(&self) -> &str {
        match *self {
           Periodic::Actinium =>  "Ac",
           Periodic::Aluminum =>  "Al",
           Periodic::Americium =>  "Am",
           Periodic::Antimony =>  "Sb",
           Periodic::Argon =>  "Ar",
           Periodic::Arsenic =>  "As",
           Periodic::Astatine =>  "At",
           Periodic::Barium =>  "Ba",
           Periodic::Berkelium =>  "Bk",
           Periodic::Beryllium =>  "Be",
           Periodic::Bismuth =>  "Bi",
           Periodic::Bohrium =>  "Bh",
           Periodic::Boron =>  "B",
           Periodic::Bromine =>  "Br",
           Periodic::Cadmium =>  "Cd",
           Periodic::Calcium =>  "Ca",
           Periodic::Californium =>  "Cf",
           Periodic::Carbon =>  "C",
           Periodic::Cerium =>  "Ce",
           Periodic::Cesium =>  "Cs",
           Periodic::Chlorine =>  "Cl",
           Periodic::Chromium =>  "Cr",
           Periodic::Cobalt =>  "Co",
           Periodic::Copernicium =>  "Cn",
           Periodic::Copper =>  "Cu",
           Periodic::Curium =>  "Cm",
           Periodic::Darmstadtium =>  "Ds",
           Periodic::Dubnium =>  "Db",
           Periodic::Dysprosium =>  "Dy",
           Periodic::Einsteinium =>  "Es",
           Periodic::Erbium =>  "Er",
           Periodic::Europium =>  "Eu",
           Periodic::Fermium =>  "Fm",
           Periodic::Flerovium =>  "Fl",
           Periodic::Fluorine =>  "F",
           Periodic::Francium =>  "Fr",
           Periodic::Gadolinium =>  "Gd",
           Periodic::Gallium =>  "Ga",
           Periodic::Germanium =>  "Ge",
           Periodic::Gold =>  "Au",
           Periodic::Hafnium =>  "Hf",
           Periodic::Hassium =>  "Hs",
           Periodic::Helium =>  "He",
           Periodic::Holmium =>  "Ho",
           Periodic::Hydrogen =>  "H",
           Periodic::Indium =>  "In",
           Periodic::Iodine =>  "I",
           Periodic::Iridium =>  "Ir",
           Periodic::Iron =>  "Fe",
           Periodic::Krypton =>  "Kr",
           Periodic::Lanthanum =>  "La",
           Periodic::Lawrencium =>  "Lr",
           Periodic::Lead =>  "Pb",
           Periodic::Lithium =>  "Li",
           Periodic::Livermorium =>  "Lv",
           Periodic::Lutetium =>  "Lu",
           Periodic::Magnesium =>  "Mg",
           Periodic::Moscovium =>  "Mc",
           Periodic::Manganese =>  "Mn",
           Periodic::Meitnerium =>  "Mt",
           Periodic::Mendelevium =>  "Md",
           Periodic::Mercury =>  "Hg",
           Periodic::Molybdenum =>  "Mo",
           Periodic::Neilsborium =>  "Ns",
           Periodic::Neodymium =>  "Nd",
           Periodic::Neon =>  "Ne",
           Periodic::Neptunium =>  "Np",
           Periodic::Nihonium =>  "Nh",
           Periodic::Nickel =>  "Ni",
           Periodic::Niobium =>  "Nb",
           Periodic::Nitrogen =>  "N",
           Periodic::Nobelium =>  "No",
           Periodic::Oganesson =>  "Og",
           Periodic::Osmium =>  "Os",
           Periodic::Oxygen =>  "O",
           Periodic::Palladium =>  "Pd",
           Periodic::Phosphorus =>  "P",
           Periodic::Platinum =>  "Pt",
           Periodic::Plutonium =>  "Pu",
           Periodic::Polonium =>  "Po",
           Periodic::Praseodymium =>  "Pr",
           Periodic::Potassium =>  "K",
           Periodic::Promethium =>  "Pm",
           Periodic::Protactinium =>  "Pa",
           Periodic::Radium =>  "Ra",
           Periodic::Radon =>  "Rn",
           Periodic::Rhenium =>  "Re",
           Periodic::Rhodium =>  "Rh",
           Periodic::Roentgenium =>  "Rg",
           Periodic::Rubidium =>  "Rb",
           Periodic::Ruthenium =>  "Ru",
           Periodic::Rutherfordium =>  "Rf",
           Periodic::Samarium =>  "Sm",
           Periodic::Scandium =>  "Sc",
           Periodic::Seaborgium =>  "Sg",
           Periodic::Selenium =>  "Se",
           Periodic::Silicon =>  "Si",
           Periodic::Silver =>  "Ag",
           Periodic::Sodium =>  "Na",
           Periodic::Strontium =>  "Sr",
           Periodic::Sulfur =>  "S",
           Periodic::Tantalum =>  "Ta",
           Periodic::Technetium =>  "Tc",
           Periodic::Tellurium =>  "Te",
           Periodic::Tennessine =>  "Ts",
           Periodic::Terbium =>  "Tb",
           Periodic::Thallium =>  "Tl",
           Periodic::Thorium =>  "Th",
           Periodic::Thulium =>  "Tm",
           Periodic::Tin =>  "Sn",
           Periodic::Titanium =>  "Ti",
           Periodic::Tungsten =>  "W",
           Periodic::Uranium =>  "U",
           Periodic::Vanadium =>  "V",
           Periodic::Xenon =>  "Xe",
           Periodic::Ytterbium =>  "Yb",
           Periodic::Yttrium =>  "Y",
           Periodic::Zinc =>  "Zn",
           Periodic::Zirconium =>  "Zr",
           _=> "",
        }
    }
}
impl Default for Periodic {
    fn default() -> Self {
        Self::None
    }
}
impl fmt::Display for Periodic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v:String;
        match *self {
            Periodic::Actinium => v = String::from("Actinium"),
            Periodic::Aluminum => v = String::from("Aluminum"),
            Periodic::Americium => v = String::from("Americium"),
            Periodic::Antimony => v = String::from("Antimony"),
            Periodic::Argon => v = String::from("Argon"),
            Periodic::Arsenic => v = String::from("Arsenic"),
            Periodic::Astatine => v = String::from("Astatine"),
            Periodic::Barium => v = String::from("Barium"),
            Periodic::Berkelium => v = String::from("Berkelium"),
            Periodic::Beryllium => v = String::from("Beryllium"),
            Periodic::Bismuth => v = String::from("Bismuth"),
            Periodic::Bohrium => v = String::from("Bohrium"),
            Periodic::Boron => v = String::from("Boron"),
            Periodic::Bromine => v = String::from("Bromine"),
            Periodic::Cadmium => v = String::from("Cadmium"),
            Periodic::Calcium => v = String::from("Calcium"),
            Periodic::Californium => v = String::from("Californium"),
            Periodic::Carbon => v = String::from("Carbon"),
            Periodic::Cerium => v = String::from("Cerium"),
            Periodic::Cesium => v = String::from("Cesium"),
            Periodic::Chlorine => v = String::from("Chlorine"),
            Periodic::Chromium => v = String::from("Chromium"),
            Periodic::Cobalt => v = String::from("Cobalt"),
            Periodic::Copernicium => v = String::from("Copernicium"),
            Periodic::Copper => v = String::from("Copper"),
            Periodic::Curium => v = String::from("Curium"),
            Periodic::Darmstadtium => v = String::from("Darmstadtium"),
            Periodic::Dubnium => v = String::from("Dubnium"),
            Periodic::Dysprosium => v = String::from("Dysprosium"),
            Periodic::Einsteinium => v = String::from("Einsteinium"),
            Periodic::Erbium => v = String::from("Erbium"),
            Periodic::Europium => v = String::from("Europium"),
            Periodic::Fermium => v = String::from("Fermium"),
            Periodic::Flerovium => v = String::from("Flerovium"),
            Periodic::Fluorine => v = String::from("Fluorine"),
            Periodic::Francium => v = String::from("Francium"),
            Periodic::Gadolinium => v = String::from("Gadolinium"),
            Periodic::Gallium => v = String::from("Gallium"),
            Periodic::Germanium => v = String::from("Germanium"),
            Periodic::Gold => v = String::from("Gold"),
            Periodic::Hafnium => v = String::from("Hafnium"),
            Periodic::Hassium => v = String::from("Hassium"),
            Periodic::Helium => v = String::from("Helium"),
            Periodic::Holmium => v = String::from("Holmium"),
            Periodic::Hydrogen => v = String::from("Hydrogen"),
            Periodic::Indium => v = String::from("Indium"),
            Periodic::Iodine => v = String::from("Iodine"),
            Periodic::Iridium => v = String::from("Iridium"),
            Periodic::Iron => v = String::from("Iron"),
            Periodic::Krypton => v = String::from("Krypton"),
            Periodic::Lanthanum => v = String::from("Lanthanum"),
            Periodic::Lawrencium => v = String::from("Lawrencium"),
            Periodic::Lead => v = String::from("Lead"),
            Periodic::Lithium => v = String::from("Lithium"),
            Periodic::Livermorium => v = String::from("Livermorium"),
            Periodic::Lutetium => v = String::from("Lutetium"),
            Periodic::Magnesium => v = String::from("Magnesium"),
            Periodic::Moscovium => v = String::from("Moscovium"),
            Periodic::Manganese => v = String::from("Manganese"),
            Periodic::Meitnerium => v = String::from("Meitnerium"),
            Periodic::Mendelevium => v = String::from("Mendelevium"),
            Periodic::Mercury => v = String::from("Mercury"),
            Periodic::Molybdenum => v = String::from("Molybdenum"),
            Periodic::Neilsborium => v = String::from("Neilsborium"),
            Periodic::Neodymium => v = String::from("Neodymium"),
            Periodic::Neon => v = String::from("Neon"),
            Periodic::Neptunium => v = String::from("Neptunium"),
            Periodic::Nihonium => v = String::from("Nihonium"),
            Periodic::Nickel => v = String::from("Nickel"),
            Periodic::Niobium => v = String::from("Niobium"),
            Periodic::Nitrogen => v = String::from("Nitrogen"),
            Periodic::Nobelium => v = String::from("Nobelium"),
            Periodic::Oganesson => v = String::from("Oganesson"),
            Periodic::Osmium => v = String::from("Osmium"),
            Periodic::Oxygen => v = String::from("Oxygen"),
            Periodic::Palladium => v = String::from("Palladium"),
            Periodic::Phosphorus => v = String::from("Phosphorus"),
            Periodic::Platinum => v = String::from("Platinum"),
            Periodic::Plutonium => v = String::from("Plutonium"),
            Periodic::Polonium => v = String::from("Polonium"),
            Periodic::Praseodymium => v = String::from("Praseodymium"),
            Periodic::Potassium => v = String::from("Potassium"),
            Periodic::Promethium => v = String::from("Promethium"),
            Periodic::Protactinium => v = String::from("Protactinium"),
            Periodic::Radium => v = String::from("Radium"),
            Periodic::Radon => v = String::from("Radon"),
            Periodic::Rhenium => v = String::from("Rhenium"),
            Periodic::Rhodium => v = String::from("Rhodium"),
            Periodic::Roentgenium => v = String::from("Roentgenium"),
            Periodic::Rubidium => v = String::from("Rubidium"),
            Periodic::Ruthenium => v = String::from("Ruthenium"),
            Periodic::Rutherfordium => v = String::from("Rutherfordium"),
            Periodic::Samarium => v = String::from("Samarium"),
            Periodic::Scandium => v = String::from("Scandium"),
            Periodic::Seaborgium => v = String::from("Seaborgium"),
            Periodic::Selenium => v = String::from("Selenium"),
            Periodic::Silicon => v = String::from("Silicon"),
            Periodic::Silver => v = String::from("Silver"),
            Periodic::Sodium => v = String::from("Sodium"),
            Periodic::Strontium => v = String::from("Strontium"),
            Periodic::Sulfur => v = String::from("Sulfur"),
            Periodic::Tantalum => v = String::from("Tantalum"),
            Periodic::Technetium => v = String::from("Technetium"),
            Periodic::Tellurium => v = String::from("Tellurium"),
            Periodic::Tennessine => v = String::from("Tennessine"),
            Periodic::Terbium => v = String::from("Terbium"),
            Periodic::Thallium => v = String::from("Thallium"),
            Periodic::Thorium => v = String::from("Thorium"),
            Periodic::Thulium => v = String::from("Thulium"),
            Periodic::Tin => v = String::from("Tin"),
            Periodic::Titanium => v = String::from("Titanium"),
            Periodic::Tungsten => v = String::from("Tungsten"),
            Periodic::Uranium => v = String::from("Uranium"),
            Periodic::Vanadium => v = String::from("Vanadium"),
            Periodic::Xenon => v = String::from("Xenon"),
            Periodic::Ytterbium => v = String::from("Ytterbium"),
            Periodic::Yttrium => v = String::from("Yttrium"),
            Periodic::Zinc => v = String::from("Zinc"),
            Periodic::Zirconium => v = String::from("Zirconium"),
            _=> v = String::from("None"),
        }
        write!(f, "{}", v.as_str())
    }
}
impl Random for Periodic {
    type Type = Periodic;
    fn random_type(&self) -> Self::Type {
        let max = 119;
        let val = self.random_rate(max);
        match val {
            0 => Periodic::Actinium,
            1 => Periodic::Aluminum,
            2 => Periodic::Americium,
            3 => Periodic::Antimony,
            4 => Periodic::Argon,
            5 => Periodic::Arsenic,
            6 => Periodic::Astatine,
            7 => Periodic::Barium,
            8 => Periodic::Berkelium,
            9 => Periodic::Beryllium,
            10 => Periodic::Bismuth,
            11 => Periodic::Bohrium,
            12 => Periodic::Boron,
            13 => Periodic::Bromine,
            14 => Periodic::Cadmium,
            15 => Periodic::Calcium,
            16 => Periodic::Californium,
            17 => Periodic::Carbon,
            18 => Periodic::Cerium,
            19 => Periodic::Cesium,
            20 => Periodic::Chlorine,
            21 => Periodic::Chromium,
            22 => Periodic::Cobalt,
            23 => Periodic::Copernicium,
            24 => Periodic::Copper,
            25 => Periodic::Curium,
            26 => Periodic::Darmstadtium,
            27 => Periodic::Dubnium,
            28 => Periodic::Dysprosium,
            29 => Periodic::Einsteinium,
            30 => Periodic::Erbium,
            31 => Periodic::Europium,
            32 => Periodic::Fermium,
            33 => Periodic::Flerovium,
            34 => Periodic::Fluorine,
            35 => Periodic::Francium,
            36 => Periodic::Gadolinium,
            37 => Periodic::Gallium,
            38 => Periodic::Germanium,
            39 => Periodic::Gold,
            40 => Periodic::Hafnium,
            41 => Periodic::Hassium,
            42 => Periodic::Helium,
            43 => Periodic::Holmium,
            44 => Periodic::Hydrogen,
            45 => Periodic::Indium,
            46 => Periodic::Iodine,
            47 => Periodic::Iridium,
            48 => Periodic::Iron,
            49 => Periodic::Krypton,
            50 => Periodic::Lanthanum,
            51 => Periodic::Lawrencium,
            52 => Periodic::Lead,
            53 => Periodic::Lithium,
            54 => Periodic::Livermorium,
            55 => Periodic::Lutetium,
            56 => Periodic::Magnesium,
            57 => Periodic::Moscovium,
            58 => Periodic::Manganese,
            59 => Periodic::Meitnerium,
            60 => Periodic::Mendelevium,
            61 => Periodic::Mercury,
            62 => Periodic::Molybdenum,
            63 => Periodic::Neilsborium,
            64 => Periodic::Neodymium,
            65 => Periodic::Neon,
            66 => Periodic::Neptunium,
            67 => Periodic::Nihonium,
            68 => Periodic::Nickel,
            69 => Periodic::Niobium,
            70 => Periodic::Nitrogen,
            71 => Periodic::Nobelium,
            72 => Periodic::Oganesson,
            73 => Periodic::Osmium,
            74 => Periodic::Oxygen,
            75 => Periodic::Palladium,
            76 => Periodic::Phosphorus,
            77 => Periodic::Platinum,
            78 => Periodic::Plutonium,
            79 => Periodic::Polonium,
            80 => Periodic::Praseodymium,
            81 => Periodic::Potassium,
            82 => Periodic::Promethium,
            83 => Periodic::Protactinium,
            84 => Periodic::Radium,
            85 => Periodic::Radon,
            86 => Periodic::Rhenium,
            87 => Periodic::Rhodium,
            88 => Periodic::Roentgenium,
            89 => Periodic::Rubidium,
            90 => Periodic::Ruthenium,
            91 => Periodic::Rutherfordium,
            92 => Periodic::Samarium,
            93 => Periodic::Scandium,
            94 => Periodic::Seaborgium,
            95 => Periodic::Selenium,
            96 => Periodic::Silicon,
            97 => Periodic::Silver,
            98 => Periodic::Sodium,
            99 => Periodic::Strontium,
            100 => Periodic::Sulfur,
            101 => Periodic::Tantalum,
            102 => Periodic::Technetium,
            103 => Periodic::Tellurium,
            104 => Periodic::Tennessine,
            105 => Periodic::Terbium,
            106 => Periodic::Thallium,
            107 => Periodic::Thorium,
            108 => Periodic::Thulium,
            109 => Periodic::Tin,
            110 => Periodic::Titanium,
            111 => Periodic::Tungsten,
            112 => Periodic::Uranium,
            113 => Periodic::Vanadium,
            114 => Periodic::Xenon,
            115 => Periodic::Ytterbium,
            116 => Periodic::Yttrium,
            117 => Periodic::Zinc,
            118 => Periodic::Zirconium,
            _=> Periodic::None,
        }
    }
    
}
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "fltkform", derive(FltkForm))]
/*
# PeriodicTable

This is a container for elements from the periodic table

## What is a periodic table anyway??

[The periodic table](https://en.wikipedia.org/wiki/Periodic_table), also known as the periodic table of (the) chemical elements, is a tabular display of the chemical elements. It is widely used in chemistry, physics, and other sciences, and is generally seen as an icon of chemistry. It is a graphic formulation of the periodic law, which states that the properties of the chemical elements exhibit a periodic dependence on their atomic numbers.

The table is divided into four roughly rectangular areas called blocks. The rows of the table are called periods, and the columns are called groups. Elements from the same column group of the periodic table show similar chemical characteristics. Trends run through the periodic table, with nonmetallic character (keeping their own electrons) increasing from left to right across a period, and from down to up across a group, and metallic character (surrendering electrons to other atoms) increasing in the opposite direction. The underlying reason for these trends is electron configurations of atoms.

The first periodic table to become generally accepted was that of the Russian chemist Dmitri Mendeleev in 1869: he formulated the periodic law as a dependence of chemical properties on atomic mass. Because not all elements were then known, there were gaps in his periodic table, and Mendeleev successfully used the periodic law to predict properties of some of the missing elements. The periodic law was recognized as a fundamental discovery in the late 19th century, and it was explained with the discovery of the atomic number and pioneering work in quantum mechanics of the early 20th century that illuminated the internal structure of the atom. With Glenn T. Seaborg's 1945 discovery that the actinides were in fact f-block rather than d-block elements, a recognisably modern form of the table was reached. The periodic table and law are now a central and indispensable part of modern chemistry.

The periodic table continues to evolve with the progress of science. In nature, only elements up to atomic number 94 exist; to go further, it was necessary to synthesise new elements in the laboratory. Today, all the first 118 elements are known, completing the first seven rows of the table, but chemical characterisation is still needed for the heaviest elements to confirm that their properties match their positions. It is not yet known how far the table will stretch beyond these seven rows and whether the patterns of the known part of the table will continue into this unknown region. Some scientific discussion also continues regarding whether some elements are correctly positioned in today's table. Many alternative representations of the periodic law exist, and there is some discussion as to whether or not there is an optimal form of the periodic table.
*/
pub struct PeriodicTable {
    /// Number of actinium periodic
    pub actinium:u32,
    /// Number of aluminum periodic
    pub aluminum:u32,
    /// Number of americium periodic
    pub americium:u32,
    /// Number of antimony periodic
    pub antimony:u32,
    /// Number of argon periodic
    pub argon:u32,
    /// Number of arsenic periodic
    pub arsenic:u32,
    /// Number of astatine periodic
    pub astatine:u32,
    /// Number of barium periodic
    pub barium:u32,
    /// Number of berkelium periodic
    pub berkelium:u32,
    /// Number of beryllium periodic
    pub beryllium:u32,
    /// Number of bismuth periodic
    pub bismuth:u32,
    /// Number of bohrium periodic
    pub bohrium:u32,
    /// Number of boron periodic
    pub boron:u32,
    /// Number of bromine periodic
    pub bromine:u32,
    /// Number of cadmium periodic
    pub cadmium:u32,
    /// Number of calcium periodic
    pub calcium:u32,
    /// Number of californium periodic
    pub californium:u32,
    /// Number of carbon periodic
    pub carbon:u32,
    /// Number of cerium periodic
    pub cerium:u32,
    /// Number of cesium periodic
    pub cesium:u32,
    /// Number of chlorine periodic
    pub chlorine:u32,
    /// Number of chromium periodic
    pub chromium:u32,
    /// Number of cobalt periodic
    pub cobalt:u32,
    /// Number of copernicium periodic
    pub copernicium:u32,
    /// Number of copper periodic
    pub copper:u32,
    /// Number of curium periodic
    pub curium:u32,
    /// Number of darmstadtium periodic
    pub darmstadtium:u32,
    /// Number of dubnium periodic
    pub dubnium:u32,
    /// Number of dysprosium periodic
    pub dysprosium:u32,
    /// Number of einsteinium periodic
    pub einsteinium:u32,
    /// Number of erbium periodic
    pub erbium:u32,
    /// Number of europium periodic
    pub europium:u32,
    /// Number of fermium periodic
    pub fermium:u32,
    /// Number of flerovium periodic
    pub flerovium:u32,
    /// Number of fluorine periodic
    pub fluorine:u32,
    /// Number of francium periodic
    pub francium:u32,
    /// Number of gadolinium periodic
    pub gadolinium:u32,
    /// Number of gallium periodic
    pub gallium:u32,
    /// Number of germanium periodic
    pub germanium:u32,
    /// Number of gold periodic
    pub gold:u32,
    /// Number of hafnium periodic
    pub hafnium:u32,
    /// Number of hassium periodic
    pub hassium:u32,
    /// Number of helium periodic
    pub helium:u32,
    /// Number of holmium periodic
    pub holmium:u32,
    /// Number of hydrogen periodic
    pub hydrogen:u32,
    /// Number of indium periodic
    pub indium:u32,
    /// Number of iodine periodic
    pub iodine:u32,
    /// Number of iridium periodic
    pub iridium:u32,
    /// Number of iron periodic
    pub iron:u32,
    /// Number of krypton periodic
    pub krypton:u32,
    /// Number of lanthanum periodic
    pub lanthanum:u32,
    /// Number of lawrencium periodic
    pub lawrencium:u32,
    /// Number of lead periodic
    pub lead:u32,
    /// Number of lithium periodic
    pub lithium:u32,
    /// Number of livermorium periodic
    pub livermorium:u32,
    /// Number of lutetium periodic
    pub lutetium:u32,
    /// Number of magnesium periodic
    pub magnesium:u32,
    /// Number of moscovium periodic
    pub moscovium:u32,
    /// Number of manganese periodic
    pub manganese:u32,
    /// Number of meitnerium periodic
    pub meitnerium:u32,
    /// Number of mendelevium periodic
    pub mendelevium:u32,
    /// Number of mercury periodic
    pub mercury:u32,
    /// Number of molybdenum periodic
    pub molybdenum:u32,
    /// Number of neilsborium periodic
    pub neilsborium:u32,
    /// Number of neodymium periodic
    pub neodymium:u32,
    /// Number of neon periodic
    pub neon:u32,
    /// Number of neptunium periodic
    pub neptunium:u32,
    /// Number of nihonium periodic
    pub nihonium:u32,
    /// Number of nickel periodic
    pub nickel:u32,
    /// Number of niobium periodic
    pub niobium:u32,
    /// Number of nitrogen periodic
    pub nitrogen:u32,
    /// Number of nobelium periodic
    pub nobelium:u32,
    /// Number of oganesson periodic
    pub oganesson:u32,
    /// Number of osmium periodic
    pub osmium:u32,
    /// Number of oxygen periodic
    pub oxygen:u32,
    /// Number of palladium periodic
    pub palladium:u32,
    /// Number of phosphorus periodic
    pub phosphorus:u32,
    /// Number of platinum periodic
    pub platinum:u32,
    /// Number of plutonium periodic
    pub plutonium:u32,
    /// Number of polonium periodic
    pub polonium:u32,
    /// Number of praseodymium periodic
    pub praseodymium:u32,
    /// Number of potassium periodic
    pub potassium:u32,
    /// Number of promethium periodic
    pub promethium:u32,
    /// Number of protactinium periodic
    pub protactinium:u32,
    /// Number of radium periodic
    pub radium:u32,
    /// Number of radon periodic
    pub radon:u32,
    /// Number of rhenium periodic
    pub rhenium:u32,
    /// Number of rhodium periodic
    pub rhodium:u32,
    /// Number of roentgenium periodic
    pub roentgenium:u32,
    /// Number of rubidium periodic
    pub rubidium:u32,
    /// Number of ruthenium periodic
    pub ruthenium:u32,
    /// Number of rutherfordium periodic
    pub rutherfordium:u32,
    /// Number of samarium periodic
    pub samarium:u32,
    /// Number of scandium periodic
    pub scandium:u32,
    /// Number of seaborgium periodic
    pub seaborgium:u32,
    /// Number of selenium periodic
    pub selenium:u32,
    /// Number of silicon periodic
    pub silicon:u32,
    /// Number of silver periodic
    pub silver:u32,
    /// Number of sodium periodic
    pub sodium:u32,
    /// Number of strontium periodic
    pub strontium:u32,
    /// Number of sulfur periodic
    pub sulfur:u32,
    /// Number of tantalum periodic
    pub tantalum:u32,
    /// Number of technetium periodic
    pub technetium:u32,
    /// Number of tellurium periodic
    pub tellurium:u32,
    /// Number of tennessine periodic
    pub tennessine:u32,
    /// Number of terbium periodic
    pub terbium:u32,
    /// Number of thallium periodic
    pub thallium:u32,
    /// Number of thorium periodic
    pub thorium:u32,
    /// Number of thulium periodic
    pub thulium:u32,
    /// Number of tin periodic
    pub tin:u32,
    /// Number of titanium periodic
    pub titanium:u32,
    /// Number of tungsten periodic
    pub tungsten:u32,
    /// Number of uranium periodic
    pub uranium:u32,
    /// Number of vanadium periodic
    pub vanadium:u32,
    /// Number of xenon periodic
    pub xenon:u32,
    /// Number of ytterbium periodic
    pub ytterbium:u32,
    /// Number of yttrium periodic
    pub yttrium:u32,
    /// Number of zinc periodic
    pub zinc:u32,
    /// Number of zirconium periodic
    pub zirconium:u32,
}
impl PeriodicTable {
    pub fn new() -> Self {
            PeriodicTable {
            actinium:0,
            aluminum:0,
            americium:0,
            antimony:0,
            argon:0,
            arsenic:0,
            astatine:0,
            barium:0,
            berkelium:0,
            beryllium:0,
            bismuth:0,
            bohrium:0,
            boron:0,
            bromine:0,
            cadmium:0,
            calcium:0,
            californium:0,
            carbon:0,
            cerium:0,
            cesium:0,
            chlorine:0,
            chromium:0,
            cobalt:0,
            copernicium:0,
            copper:0,
            curium:0,
            darmstadtium:0,
            dubnium:0,
            dysprosium:0,
            einsteinium:0,
            erbium:0,
            europium:0,
            fermium:0,
            flerovium:0,
            fluorine:0,
            francium:0,
            gadolinium:0,
            gallium:0,
            germanium:0,
            gold:0,
            hafnium:0,
            hassium:0,
            helium:0,
            holmium:0,
            hydrogen:0,
            indium:0,
            iodine:0,
            iridium:0,
            iron:0,
            krypton:0,
            lanthanum:0,
            lawrencium:0,
            lead:0,
            lithium:0,
            livermorium:0,
            lutetium:0,
            magnesium:0,
            moscovium:0,
            manganese:0,
            meitnerium:0,
            mendelevium:0,
            mercury:0,
            molybdenum:0,
            neilsborium:0,
            neodymium:0,
            neon:0,
            neptunium:0,
            nihonium:0,
            nickel:0,
            niobium:0,
            nitrogen:0,
            nobelium:0,
            oganesson:0,
            osmium:0,
            oxygen:0,
            palladium:0,
            phosphorus:0,
            platinum:0,
            plutonium:0,
            polonium:0,
            praseodymium:0,
            potassium:0,
            promethium:0,
            protactinium:0,
            radium:0,
            radon:0,
            rhenium:0,
            rhodium:0,
            roentgenium:0,
            rubidium:0,
            ruthenium:0,
            rutherfordium:0,
            samarium:0,
            scandium:0,
            seaborgium:0,
            selenium:0,
            silicon:0,
            silver:0,
            sodium:0,
            strontium:0,
            sulfur:0,
            tantalum:0,
            technetium:0,
            tellurium:0,
            tennessine:0,
            terbium:0,
            thallium:0,
            thorium:0,
            thulium:0,
            tin:0,
            titanium:0,
            tungsten:0,
            uranium:0,
            vanadium:0,
            xenon:0,
            ytterbium:0,
            yttrium:0,
            zinc:0,
            zirconium:0,
        }
    }
}
