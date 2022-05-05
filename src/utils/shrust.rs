//! A library for creating interactive command line shells
use prettytable::format;
use prettytable::Table;

use std::error::Error;
use std::fmt;
use std::io;
use std::io::prelude::*;
use std::ops::{Deref, DerefMut, Shl};
use std::string::ToString;
use std::sync::{Arc, Mutex};

use rustyline::config::Configurer;
use std::collections::BTreeMap;

use rustyline::error::ReadlineError;
use rustyline::validate::{ValidationContext, ValidationResult, Validator};
use rustyline::{EditMode, Editor, Helper, OutputStreamType};
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
