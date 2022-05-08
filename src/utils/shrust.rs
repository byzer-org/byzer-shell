//! A library for creating interactive command line shells
use prettytable::format;
use prettytable::Table;
use rustyline::highlight::Highlighter;

use std::error::Error;
use std::fmt;
use std::io;
use std::io::prelude::*;
use std::ops::{Deref, DerefMut, Shl};
use std::string::ToString;
use std::sync::{Arc, Mutex};
use std::borrow::Cow;
use std::borrow::Cow::Borrowed;
use std::borrow::Cow::Owned;

use rustyline::error::ReadlineError;
use rustyline::validate::{ValidationContext, ValidationResult, Validator};
use rustyline::{EditMode, Editor, Helper, OutputStreamType};
use rustyline_derive::{Completer, Helper, Highlighter, Hinter};

// #[derive(Completer, Helper, Highlighter, Hinter)]
// pub struct MatchScriptEndValidator {
//     _priv: (),
// }

// // Completer + Hinter + Highlighter + Validator
// impl MatchScriptEndValidator {
//     /// Constructor
//     pub fn new() -> Self {
//         Self { _priv: () }
//     }
// }

// impl Validator for MatchScriptEndValidator {
//     fn validate(&self, ctx: &mut ValidationContext) -> rustyline::Result<ValidationResult> {
//         let i = ctx.input();
//         if i.ends_with(";") {
//             Ok(ValidationResult::Valid(None))
//         } else {
//             Ok(ValidationResult::Incomplete)
//         }
//     }
// }

#[derive(Completer, Helper, Hinter)]
pub struct EditHelper {
    _match_script_end_validator: (),
    _highlighter: (),
}

impl EditHelper {
    /// Constructor
    pub fn new() -> Self {
        Self { 
            _match_script_end_validator: (),
            _highlighter: (),
        }
    }
}

fn word_matching_color_mode(key_word: &str) -> Option<String> {
    // need to refactor the key_word list
    match key_word {
        "select" | "as" | "load" | "set" | "from" | "where" | "connect" | "!show"  => Some(format!("\x1b[1;34m{}\x1b[0m", key_word)),
        _ => None
    }
}

fn split_with_whitespace(line: &str) -> Option<Vec<&str>> {
    let mut partition_set = Vec::new();
    let mut last = 0;
    for (index, matched) in line.match_indices(|c: char| !c.is_alphanumeric()) {
        if last != index {
            partition_set.push(&line[last..index]);
        }
        partition_set.push(matched);
        last = index + matched.len();
    }
    if last < line.len() {
        partition_set.push(&line[last..]);
    }

    if !partition_set.is_empty() {
        return Some(partition_set);
    } else {
        return None;
    }
}

impl Validator for EditHelper {
    fn validate(&self, ctx: &mut ValidationContext) -> rustyline::Result<ValidationResult> {
        let i = ctx.input();
        if i.ends_with(";") {
            Ok(ValidationResult::Valid(None))
        } else {
            Ok(ValidationResult::Incomplete)
        }
    }
}

impl Highlighter for EditHelper {
    fn highlight<'l>(&self, line: &'l str, _pos: usize) -> Cow<'l, str> {
        if line.len() <= 1 {
            return Borrowed(line);
        }

        if let Some(partition_set) = split_with_whitespace(line) {
            let mut replaced_partition_set = Vec::new();
            for partition in partition_set.iter() {
                match word_matching_color_mode(*partition) {
                    Some(pat) => {
                        replaced_partition_set.push(pat);
                    },
                    None => {
                        replaced_partition_set.push(partition.to_string());
                    }
                }
            }
            let new_line = replaced_partition_set.join("");
            return Owned(new_line);
        }
        
        Borrowed(line)
    }

    fn highlight_char(&self, _line: &str, _pos: usize) -> bool {
        // this function is used for triggering syntax highligher
        // as currently it is trigger everytime the new character
        // insert/move and cursor move, return true
        true
    }
}