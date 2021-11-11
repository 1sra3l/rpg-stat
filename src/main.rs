// Rusified in 2021 Copyright Israel Dahl. All rights reserved.
// 
//        /VVVV\
//      /V      V\
//    /V          V\
//   /      0 0     \
//   \|\|\</\/\>/|/|/
//        \_/\_/
// 
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License version 2 as
// published by the Free Software Foundation.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
// 
// You should have received a copy of the GNU General Public License
// along with this program; if not, write to the Free Software
// Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA

mod stats;
mod class;
mod creature;
mod body;
mod legendary;
mod tests;
mod types;
mod npc;
mod random;
mod equation;
mod special;
mod attributes;
mod effect;

//use crate::stats::Basic as Stats;
//use crate::creature::Person;
//use crate::legendary::Legendary;
//use crate::stats::Builder;
//use crate::attributes::Rate;
extern crate clap;
use clap::{App, load_yaml};

fn main() {
    //TODO make unused warnings go away by putting things here
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();

// stats.rs
    if let Some(matches) = matches.subcommand_matches("stats") {
        //use crate::stats::Basic as Basic_stat
        if matches.is_present("Basic_stat") {
            // basic stat
        }
        //use crate::stats::Normal as Normal_stat
        //use crate::stats::Advanced as Advanced_stat
        //use crate::stats::
    }
// class.rs
    if let Some(matches) = matches.subcommand_matches("class") {
        //use crate::class::Basic as Basic_class
        if matches.is_present("Basic_class") {
            // 
        }
        //use crate::class::Normal as Normal_class
        //use crate::class::Advanced as Advanced_class
        //use crate::class::Alignment
    }
// type.rs
    if let Some(matches) = matches.subcommand_matches("type") {
        //use crate::type::Rate
        if matches.is_present("Rate") {
            // 
        }
        //use crate::type::Element
        if matches.is_present("Element") {
            // 
        }
        //use crate::type::
    }
}
