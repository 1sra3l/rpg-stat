mod stats;
mod class;
mod creature;
mod body;
mod legendary;
mod tests;
mod types;
//use crate::stats::Basic as Stats;
use crate::creature::Person;
use crate::legendary::Legendary;
use crate::stats::Builder;
use crate::types::Rate;

fn main() {
    let h:Person = Person::Elf;
    let s:String = h.to_string();
    println!("{}\n{:?}", h, s);
    let sc:Legendary = Legendary::SantaClaus;
    sc.build_basic(0.0,1.0);
    let r = Rate::Often;
    for _i in 0..30 {
        if r.worked() {
            println!("worked");
        }
        else {
            println!("failed");
        }
    }
}
