//! A library for creating interactive command line shells
use prettytable::Table;
use prettytable::format;

use std::io;
use std::io::prelude::*;
use std::string::ToString;
use std::error::Error;
use std::fmt;
use std::ops::{Deref, DerefMut, Shl};
use std::sync::{Arc, Mutex};

use std::collections::BTreeMap;
use rustyline::config::Configurer;

use rustyline::error::ReadlineError;
use rustyline::{EditMode, Editor, Helper, OutputStreamType};
use rustyline::validate::{ValidationContext, ValidationResult, Validator};
use rustyline_derive::{Completer, Helper, Highlighter, Hinter};


#[derive(Completer, Helper, Highlighter, Hinter)]
pub struct MatchScriptEndValidator {
    _priv: (),
}

// Completer + Hinter + Highlighter + Validator
impl MatchScriptEndValidator {
    /// Constructor
    pub fn new() -> Self {
        Self { _priv: () }
    }
}

impl Validator for MatchScriptEndValidator {
    fn validate(&self, ctx: &mut ValidationContext) -> rustyline::Result<ValidationResult> {
        let i = ctx.input();
        if i.ends_with(";") {
            Ok(ValidationResult::Valid(None))
        } else {
            Ok(ValidationResult::Incomplete)
        }
    }
}
