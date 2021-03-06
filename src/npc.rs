/*
# NPC - Non-Player Characters

These are what make the grind worth it sometimes.  When the interaction is quirky and humorous, as well as decently random (see Class based below) it makes the environment more fun.

Fortunately we live in a world where there are tons of quotes in the public domain (like the entire KJV Bible or Shakespear as examples of massive English works with numerous well understood quotes)

We also provide `AI` Logic as well.
*/
use std::fmt;
use std::fmt::Debug;
extern crate num;
use serde::{Deserialize, Serialize};

#[cfg(feature = "fltkform")]
use fltk::{prelude::*, *};
#[cfg(feature = "fltkform")]
use fltk_form_derive::*;
#[cfg(feature = "fltkform")]
use fltk_form::FltkForm;

// our module
use crate::random::Random;

/*
# State
The state the individual is in.
There are two states `Broken` and `Ordered`  These are assesments of the individual's alignment and choices.  If their choices match the Alignment, the State becomes `Ordered` when out of sync it becomes `Broken`
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "fltkform", derive(FltkForm))]
pub enum State {
    /// Alignment does not match actions
    Broken,
    /// Actions match Alignment
    Ordered,
}
impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v:String;
        match *self {
            State::Broken => v = String::from("Broken"),
            State::Ordered => v = String::from("Ordered"),

        }
        write!(f, "{}", v.as_str())
    }
}
impl Default for State {
    fn default() -> Self {
        Self::Ordered
    }
}

/*

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "fltkform", derive(FltkForm))]
pub enum Purpose {
    Random,
    Story,
    Task,
}
impl fmt::Display for Purpose {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v:String;
        match *self {
            Purpose::Random => v = String::from("Random"),
            Purpose::Story => v = String::from("Story"),
            Purpose::Task => v = String::from("Task"),

        }
        write!(f, "{}", v.as_str())
    }
}
impl Default for Purpose {
    fn default() -> Self {
        Self::Random
    }
}

/*
# Conversation
This is used to generate conversational content for NPCs
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "fltkform", derive(FltkForm))]
pub enum Conversation {
    /// General life advice, such as proverbs
    Advice,
    /// Describing anything in great detail
    Details,
    /// Talks about their own dreams
    Dreams,
    /// Talking about an in-gmae event
    Event,
    /// Breaking the fourth wall, being self aware
    Fourth,
    /// A simple "Hello", "Good day", "How are you?"
    Greeting,
    /// General gameplay tips
    Hint,
    /// Random puns
    Jokes,
    /// Talks about personal problems
    Problems,
    /// In game hints to drive the story/quest
    Quest,
    /// Random public domain material
    Quotes,
    /// Any number of random things about the weather, sports, etc.
    SmallTalk,
    /// Talks about geological/biological features of the surroundings
    Surroundings,
}
impl fmt::Display for Conversation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v:String;
        match *self {
            Conversation::Quest => v = String::from("Quest"),
            Conversation::Hint => v = String::from("Hint"),
            Conversation::Advice => v = String::from("Advice"),
            Conversation::Greeting => v = String::from("Greeting"),
            Conversation::SmallTalk => v = String::from("SmallTalk"),
            Conversation::Event => v = String::from("Event"),
            Conversation::Surroundings => v = String::from("Surroundings"),
            Conversation::Problems => v = String::from("Problems"),
            Conversation::Dreams => v = String::from("Dreams"),
            Conversation::Jokes => v = String::from("Jokes"),
            Conversation::Quotes => v = String::from("Quotes"),
            Conversation::Details => v = String::from("Details"),
            Conversation::Fourth => v = String::from("Fourth"),

        }
        write!(f, "{}", v.as_str())
    }
}
impl Default for Conversation {
    fn default() -> Self {
        Self::SmallTalk
    }
}

/*
*/
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "fltkform", derive(FltkForm))]
pub struct Script {
    /// The list of lines
    pub lines:Vec<u32>,
    /// Type of conversation
    pub conversation:Conversation,
    /// The file to read the text from
    pub file:String,
    /// Are the lines to be ready randomly or in order?
    pub random:bool,
    /// Are we done here?
    pub finished:bool
}
impl Random for Script{
    type Type = Script;
    fn random_type(&self) -> Self::Type {
        Self::empty()
    }
}
impl Script {
    /// Make a generic conversation script
    #[allow(dead_code)]
    pub fn empty() -> Self where Self:Sized {
        Script {
            lines:vec![],
            conversation:Conversation::SmallTalk,
            file:String::from(""),
            random:true,
            finished:false,
        }
    }
    #[allow(dead_code)]
    pub fn default_words(&self)-> String {
        if self.half() {
            return String::from("Math is way more exciting than people really think it is...")
        }
        String::from("Aren't role playing games fascinating!")
    }
    #[allow(dead_code)]
    pub fn speak(&self) -> Option<String> {
        if self.finished {
            return None
        }
        if self.file != *"" {
            if !self.random {
                // look at lines
                // look at conversation type
                
            }
            //TODO
            return Some(self.default_words())
        }
        Some(self.default_words())
    }
}

/*
# NPC
This holds our Non-Player Characters
*/
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "fltkform", derive(FltkForm))]
pub struct NPC {
    /// Whether the NPC is on task
    pub state:State,
    /// Are they a Random, a Story, or an NPC with a specific Task, like a merchant?
    pub purpose:Purpose,
    /// The first time you speak **only**
    pub meet:Conversation, // Script
    /// Every other time
    pub conversations:Vec<Conversation>, // Vec<Script>
}
impl NPC {

    /// Make an empty conversation NPC
    #[allow(dead_code)]
    pub fn empty() -> Self where Self:Sized {
        NPC {
            state:State::Broken,
            purpose:Purpose::Random,
            meet:Conversation::Greeting,
            conversations:vec!{},
        }
    }

    /// Make a generic conversation NPC
    pub fn basic() -> Self where Self:Sized {
        let conversations:Vec<Conversation> = vec![ Conversation::SmallTalk, Conversation::Surroundings,  Conversation::Quotes, Conversation::Details, Conversation::Advice ];
        NPC {
            state:State::Ordered,
            purpose:Purpose::Random,
            meet:Conversation::Greeting,
            conversations,
        }
    }
}
impl Default for NPC {
    fn default() -> Self where Self:Sized {
        Self::basic()
    }
}
