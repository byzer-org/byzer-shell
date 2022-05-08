//! A library for creating interactive command line shells
use rustyline::highlight::Highlighter;
use colored::*;

use std::string::ToString;
use std::borrow::Cow;
use std::borrow::Cow::Borrowed;
use std::borrow::Cow::Owned;

use rustyline::validate::{ValidationContext, ValidationResult, Validator};
use rustyline_derive::{Completer, Helper, Hinter};

use std::collections::HashSet;
#[derive(Completer, Helper, Hinter)]
pub struct EditHelper {
    _match_script_end_validator: (),
    _highlighter: (),
    sql_keyword_list: HashSet<String>,
}

impl EditHelper {
    /// Constructor
    pub fn new() -> Self {
        let sql_keyword_list: HashSet<String> = HashSet::from(
            [
                "add", "all", "alter", "and", "any", "as", "asc", "backup", "between", "by", "connect", "constraint", 
                "column", "case", "check", "create", "database","default", "delete", "desc", "distinct", "drop","exec", 
                "exists", "foreign", "from", "full", "group","having", "in", "is", "index", "inner", "into", "join", 
                "key", "load", "left", "like", "limit", "local", "null", "not", "outer", "or", "order", "primary", 
                "procedure", "replace", "right", "rownum", "set", "select", "table", "top", "truncate", "unique", "union", 
                "update", "view", "values", "where", "!if", "!else", "!show"
            ].map(|s| s.to_string())
        );

        Self { 
            _match_script_end_validator: (),
            _highlighter: (),
            sql_keyword_list,
        }
    }
}

impl EditHelper {
    fn word_matching_color_mode(&self, key_word: &str) -> Option<String> {
        if self.sql_keyword_list.contains(&key_word.to_lowercase()) {
            Some(format!("\x1b[1;34m{}\x1b[0m", key_word))
        } else {
            None
        }
    }
}

fn split_with_whitespace(line: &str) -> Option<Vec<&str>> {
    let mut partition_set = Vec::new();
    let mut last = 0;
    for (index, matched) in line.match_indices(|c: char| !(c.is_alphanumeric() || c == '!')) {
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
                match self.word_matching_color_mode(*partition) {
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