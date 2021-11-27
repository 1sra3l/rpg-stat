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
//mod legendary;
mod tests;
mod types;
mod npc;
mod random;
mod equation;
mod special;
mod attributes;
mod effect;
mod item;

//use crate::stats::Advanced as ReadmeBuilder;
//use crate::creature::Person;
//use crate::legendary::Legendary;
//use crate::stats::Builder;
//use crate::attributes::Rate;
extern crate clap;
use clap::{App, load_yaml};
//use toml::*;
//use serde::{Deserialize, Serialize};


// fltkform features
#[cfg(feature = "fltkform")]
use fltk::{prelude::*, *};
#[cfg(feature = "fltkform")]
use fltk_form::{FltkForm, HasProps};



#[cfg(feature = "fltkform")]
fn fltk_main() {
    use crate::creature::Stats as Stats;

    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    app::set_background_color(222, 222, 222);

    let mut win = window::Window::default().with_size(400, 300);
    let my_struct = Stats::default();
    let grp = group::Scroll::default()
        .with_size(300, 200)
        .center_of_parent();
    // inside group
    let form = my_struct.view();
    grp.end();
    let mut btn = button::Button::default()
        .with_label("print")
        .with_size(80, 30)
        .below_of(&grp, 5)
        .center_x(&grp);
    //grp.set_frame(enums::FrameType::EngravedFrame);
    win.end();
    win.show();

    //let v = form.get_prop("hp"); // <-- get a single property
    //assert_eq!(v, Some("3.0".to_owned()));

    btn.set_callback(move |_| {
        println!("{:?}", form.get_props()); // <-- get a HashMap of the properties
    });

    while app.wait() {
       win.redraw();
    }
}

fn main() {
    #[cfg(feature = "fltkform")]
    {
        fltk_main();
        return;
    }
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

