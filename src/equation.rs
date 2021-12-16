/*!# Math
This contains functions to deal with `T` in ways that don't invovle long num::cast().unwrap() operations.
```
use rpg_stat::equation::Math;
let t:i32 = 16;
assert_eq!(8, Math::half(t));
assert_eq!(4, Math::quarter(t));
assert_eq!(12, Math::three_quarters(t));
assert_eq!(32, Math::double(t));
let numbers:Vec<i32> = vec![2, 4, 6, 8, 10, 12];
assert_eq!(7, Math::mean(numbers.clone()));
assert_eq!(12, Math::sqrt(144));

```
*/
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign};
extern crate num;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
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
    #[allow(dead_code)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
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
    #[allow(dead_code)]
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
#[allow(unused)]
pub struct Math <T:Copy 
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
                 + std::cmp::PartialEq
                 + num::NumCast> {
    pub answer:T,
    pub question:String,
}
impl <T:Copy 
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
    + std::cmp::PartialEq
    + num::NumCast> Math<T> {
    /// `e = m * (c * c)`
    #[allow(unused)]
    pub fn e_mc2 (m:T, c:T) -> T {
        m * (c * c)
    }
/*

*/
    #[allow(unused)]
    pub fn patterns(begin:T, end:T, number_of_entries:usize) -> Vec<T> {
        //30% digit one is 1
        // exponentially less for each number
        let mut ret_vec:Vec<T> = vec![];
        /*let mut tracker = begin;
        for 0..number_of_entries {
            if value < num::cast(30).unwrap() {
                
            }
        }*/
        ret_vec
    }
/*
Divide a number to three quarters
*/
    #[allow(unused)]
    pub fn three_quarters(number: T) -> T {
        let four = num::cast(4).unwrap();
        let three = num::cast(3).unwrap();
        (number / four) * three
    }
/*
Divide a number to one quarter
*/
    #[allow(unused)]
    pub fn quarter(number: T) -> T {
        let four = num::cast(4).unwrap();
        number / four
    }
/*
Divide a number in half
*/
    #[allow(unused)]
    pub fn half(number: T) -> T {
        let two = num::cast(2).unwrap();
        number / two
    }
/*
Double a number
*/
    #[allow(unused)]
    pub fn double(number: T) -> T {
        let two = num::cast(2).unwrap();
        number * two
    }
/*
Get one and one half back
*/
    #[allow(unused)]
    pub fn half_extra(number: T) -> T {
        let two = num::cast(2).unwrap();
        (number / two) + number
    }
/*
Not sure if this is the best equation to use, but it works...
*/
    #[allow(unused)]
    pub fn sqrt(number: T) -> T {
        let mut num:T = num::cast::<u32, T>(0).unwrap();
        let one:T = num::cast::<u32, T>(1).unwrap();
        while num < number {
            if num * num == number {
                return num;
            }
            num += one;
        }
        T::default()
    }
    #[allow(unused)]
    pub fn population_standard_deviation(numbers:Vec<T>) -> T {
        Math::sqrt(Math::variance(numbers))
    }

    #[allow(unused)]
    pub fn variance(numbers:Vec<T>) -> T {
        let mean:T = Math::mean(numbers.clone());
        let mut total:T = T::default();
        let mut counter:T = T::default();
        for number in numbers {
            counter += num::cast(1).unwrap();
            let result:T = number - mean;
            total += result * result
        }
        total / counter
    }
    #[allow(unused)]
    pub fn mean(numbers:Vec<T>) -> T {
        let mut total:T = num::cast(0).unwrap();
        let mut total_count:T = num::cast(0).unwrap();
        for number in numbers {
            total += number;
            total_count += num::cast(1).unwrap();
        }
        total / total_count
    }
    
}
