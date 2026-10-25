// NONOS Operating System (AGPL-3.0-or-later)
//! The line's operators, typed against a word or apart from it. `linux cat
//! </dev/null` handed cat the file name "</dev/null", and `echo hi>f` echoed
//! "hi>f": `<`, `>`, `>>` and `|` were operators only standing alone.

mod admit;
mod operators;
mod plan;
mod refusals;
