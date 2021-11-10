use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign};
extern crate num;
use num::NumCast;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, Deserialize, Serialize)]
pub enum Equation<T:Copy 
                 + Default
                 + Display
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
impl<T:Copy 
    + Default
    + Display
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
    + num::NumCast> Equation<T> {
    /// 
    pub fn result(&self) -> T {
        match *self {
            Equation::Add(a, b) => {
                return a + b
            },
            Equation::Subtract(a, b) => {
                return a - b
            },
            Equation::Multiply(a, b) => {
                return a * b
            },
            Equation::Divide(a, b) => {
                return a / b
            },
            Equation::Power(a, b) => {
                let mut result:T = a;
                let mut iter:T = b;
                iter -= num::cast(1).unwrap();
                let nada:T = num::cast(0).unwrap();
                while iter > nada {
                    result *= a;
                    iter -= num::cast(1).unwrap();
                }
                return result
            }, 
        }
    }
}
impl<T:Copy 
    + Default
    + Display
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
    + num::NumCast> fmt::Display for Equation<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v:String;
        let first:T;
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
