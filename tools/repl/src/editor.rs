/*
 * Copyright 2020 Fluence Labs Limited
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::ReplResult;

use rustyline::completion::{Completer, FilenameCompleter, Pair};
use rustyline::error::ReadlineError;
use rustyline::highlight::{Highlighter, MatchingBracketHighlighter};
use rustyline::hint::{Hinter, HistoryHinter};
use rustyline::validate::{self, MatchingBracketValidator, Validator};
use rustyline::{Cmd, CompletionType, Config, Context, EditMode, Editor, KeyEvent};
use rustyline_derive::Helper;

use std::borrow::Cow::{self, Borrowed, Owned};
use std::collections::HashSet;

pub(super) fn init_editor() -> ReplResult<Editor<REPLHelper>> {
    let config = Config::builder()
        .history_ignore_space(true)
        .completion_type(CompletionType::Fuzzy)
        .edit_mode(EditMode::Emacs)
        .build();

    let repl_hinter = REPLHinter {
        commands_hints: commands_hints(),
        history_hinter: HistoryHinter {},
    };
    let repl_helper = REPLHelper {
        completer: FilenameCompleter::new(),
        highlighter: MatchingBracketHighlighter::new(),
        hinter: repl_hinter,
        colored_prompt: "".to_owned(),
        validator: MatchingBracketValidator::new(),
    };

    let mut rl = Editor::with_config(config)?;
    rl.set_helper(Some(repl_helper));
    // On MacBook with MacOS Monterey 12.3.1
    // KeyEvent::alt(key) triggers on control+key of laptop keyboard,
    // KeyEvent::ctrl(key) triggers on nothing on the same setup.
    // Did not test it on other keyboards/OS.
    // Perhaps both versions needed to support same behavior on setups.
    rl.bind_sequence(KeyEvent::alt('n'), Cmd::HistorySearchForward);
    rl.bind_sequence(KeyEvent::alt('p'), Cmd::HistorySearchBackward);

    Ok(rl)
}

#[derive(Helper)]
pub(super) struct REPLHelper {
    completer: FilenameCompleter,
    highlighter: MatchingBracketHighlighter,
    validator: MatchingBracketValidator,
    hinter: REPLHinter,
    colored_prompt: String,
}

impl REPLHelper {
    pub(super) fn set_prompt_color(&mut self, color: String) {
        self.colored_prompt = color;
    }
}

/// Tries to find hint from history if its failed from supported command list.
pub(super) struct REPLHinter {
    commands_hints: HashSet<String>,
    history_hinter: HistoryHinter,
}

impl Completer for REPLHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &Context<'_>,
    ) -> std::result::Result<(usize, Vec<Pair>), ReadlineError> {
        self.completer.complete(line, pos, ctx)
    }
}

impl Hinter for REPLHelper {
    type Hint = String;

    fn hint(&self, line: &str, pos: usize, ctx: &Context<'_>) -> Option<String> {
        if pos < line.len() {
            return None;
        }

        if let Some(hint) = self.hinter.history_hinter.hint(line, pos, ctx) {
            return Some(hint);
        }

        self.hinter
            .commands_hints
            .iter()
            .filter_map(|hint| {
                // expect hint after word complete, like redis cli, add condition:
                // line.ends_with(" ")
                if pos > 0 && hint.starts_with(&line[..pos]) {
                    Some(hint[pos..].to_owned())
                } else {
                    None
                }
            })
            .next()
    }
}

impl Highlighter for REPLHelper {
    fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
        self.highlighter.highlight(line, pos)
    }

    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        default: bool,
    ) -> Cow<'b, str> {
        if default {
            Borrowed(&self.colored_prompt)
        } else {
            Borrowed(prompt)
        }
    }

    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Owned("\x1b[1m".to_owned() + hint + "\x1b[m")
    }

    fn highlight_char(&self, line: &str, pos: usize) -> bool {
        self.highlighter.highlight_char(line, pos)
    }
}

impl Validator for REPLHelper {
    fn validate(
        &self,
        ctx: &mut validate::ValidationContext<'_>,
    ) -> rustyline::Result<validate::ValidationResult> {
        self.validator.validate(ctx)
    }

    fn validate_while_typing(&self) -> bool {
        self.validator.validate_while_typing()
    }
}

fn commands_hints() -> HashSet<String> {
    let mut set = HashSet::new();
    set.insert(String::from("load"));
    set.insert(String::from("unload"));
    set.insert(String::from("call"));
    set.insert(String::from("envs"));
    set.insert(String::from("fs"));
    set.insert(String::from("interface"));
    set.insert(String::from("heap"));
    set.insert(String::from("help"));
    set
}
