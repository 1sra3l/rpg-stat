use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign};
extern crate num;
use num::NumCast;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, Deserialize, Serialize)]
pub enum Equation<T:Copy 
                 + Default
                 + AddAssign
                 + Add<Output = T>
                 + Div<Output = T>
                 + DivAssign
                 + Mul<Output = T>
                 + MulAssign
                 + Neg<Output = T>
                 + Rem<Output = T>
                 + RemAssign
                 + Sub<Output = T>
                 + SubAssign
                 + std::cmp::PartialOrd
                 + num::NumCast> {
    Add(T, T),
    Subtract(T, T),
    Multiply(T, T),
    Divide(T, T),
    Power(T, T),
}
impl fmt::Display for Equation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v:String;
        let fisrt:T;
        let second:T;
        match *self {
            Equation::Add(a, b) => {
                first=a;
                second=b;
                v = String::from("+");
            },
            Equation::Subtract(a, b) => {
                first=a;
                second=b;
                v = String::from("-");
            },
            Equation::Multiply(a, b) => {
                first=a;
                second=b;
                v = String::from("*");
            },
            Equation::Divide(a, b) => {
                first=a;
                second=b;
                v = String::from("/");
            },
            Equation::Power(a, b) => {
                first=a;
                second=b;
                v = String::from("^");
            },
            //Equation::(a, b) => {first=a;second=b;v = String::from("");},

        }
        write!(f, "{} {} {}", first, v.as_str(), second)
    }
}
