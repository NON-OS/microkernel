// NONOS Operating System (AGPL-3.0-or-later)
//! Everything the shell dispatches can be completed, and nothing is offered
//! twice: the route table and the completion word groups, as values.

use super::complete_words::{FILES, SHELL};
use super::complete_words_more::{LATE, SYSTEM, TEXT};

/// Every name the shell's dispatcher answers to, from the table it routes by.
fn dispatched_names() -> Vec<String> {
    super::shell_route::find::names().map(|n| String::from_utf8_lossy(n).into_owned()).collect()
}

/// Every name Tab offers: the groups complete.rs builds COMMANDS from, in its order.
fn completed_names() -> Vec<String> {
    [SHELL, FILES, SYSTEM, TEXT, LATE]
        .iter()
        .flat_map(|g| g.iter())
        .map(|n| String::from_utf8_lossy(n).into_owned())
        .collect()
}

/// A command that runs and cannot be completed is a command nobody finds, and
/// the two lists are edited at different times by different changes. This is
/// the check that keeps them together.
#[test]
fn everything_the_dispatcher_answers_to_can_be_completed() {
    let completed = completed_names();
    let missing: Vec<String> =
        dispatched_names().into_iter().filter(|n| !completed.contains(n)).collect();
    assert!(missing.is_empty(), "dispatched but not completable: {missing:?}");
}

#[test]
fn the_completion_list_has_no_duplicates() {
    let mut names = completed_names();
    names.sort();
    let before = names.len();
    names.dedup();
    assert_eq!(before, names.len(), "a name is listed twice for completion");
}
