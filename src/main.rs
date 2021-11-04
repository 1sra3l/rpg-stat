mod stats;
mod class;
mod creature;
mod tests;
//use crate::stats::Basic as Stats;
use crate::creature::Person;

fn main() {
    let h:Person = Person::Elf;
    let s:String = h.to_string();
    println!("{}\n{:?}", h, s);
}
