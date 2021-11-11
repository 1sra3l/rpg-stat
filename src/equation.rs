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
                 + std::cmp::PartialEq
                 + num::NumCast> {
    AddRight(Operation<T>, T),
    AddLeft(T, Operation<T>),
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
    + std::cmp::PartialEq
    + num::NumCast> Equation<T> {
    /// 
    pub fn result(&self) -> T {
        match *self {
            Equation::AddRight(op, val) => {
                op.result() + val
            },
            _=> Default::default(),
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
    + std::cmp::PartialEq
    + num::NumCast> fmt::Display for Equation<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
match *self {
            Equation::AddRight(op, val) => write!(f, "({}) + {}", op, val),
            Equation::AddLeft(val, op) => write!(f, "{} + ({})", val, op),
            _=> write!(f, ""),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, Deserialize, Serialize)]
pub enum Operation<T:Copy 
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
                 + std::cmp::PartialEq
                 + num::NumCast> {
    Add(T, T),
    Subtract(T, T),
    Multiply(T, T),
    Divide(T, T),
    Power(T, T),
    None(T),
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
    + std::cmp::PartialEq
    + num::NumCast> Default for Operation<T> {
    fn default() -> Self {
        Self::None(Default::default())
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
    + std::cmp::PartialEq
    + num::NumCast> Operation<T> {
    /// 
    pub fn result(&self) -> T {
        match *self {
            Operation::Add(a, b) => {
                a + b
            },
            Operation::Subtract(a, b) => {
                a - b
            },
            Operation::Multiply(a, b) => {
                a * b
            },
            Operation::Divide(a, b) => {
                a / b
            },
            Operation::Power(a, b) => {
                let mut result:T = a;
                let mut iter:T = b;
                iter -= num::cast(1).unwrap();
                let nada:T = num::cast(0).unwrap();
                while iter > nada {
                    result *= a;
                    iter -= num::cast(1).unwrap();
                }
                result
            },
            Operation::None(a) => a,
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
    + std::cmp::PartialEq
    + num::NumCast> fmt::Display for Operation<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v:String;
        let first:T;
        let second:T;
        match *self {
            Operation::Add(a, b) => {
                first = a;
                second = b;
                v = String::from("+");
            },
            Operation::Subtract(a, b) => {
                first = a;
                second = b;
                v = String::from("-");
            },
            Operation::Multiply(a, b) => {
                first = a;
                second = b;
                v = String::from("*");
            },
            Operation::Divide(a, b) => {
                first = a;
                second = b;
                v = String::from("/");
            },
            Operation::Power(a, b) => {
                first = a;
                second = b;
                v = String::from("^");
            },
            Operation::None(a) => {
                first = a;
                return write!(f, "{}", first)
            },
            //Operation::(a, b) => {first = a;second = b;v = String::from("");},
            

        }
        write!(f, "{} {} {}", first, v.as_str(), second)
    }
}
