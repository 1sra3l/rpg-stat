//mod stats;
//mod class;
mod creature;

//use crate::stats::Basic as Stats;
use crate::creature::Hero;

fn main() {
    let h:Hero = Hero::Elf;
    println!("{}", h);
}
