// NONOS Operating System (AGPL-3.0-or-later)
//! What the reader is told when a page calls alert, confirm or prompt. The
//! page was answered at once, never with a yes, and the line says so and
//! quotes what it asked.

use super::dialog_line::{dialog_line, Asked, QUOTED};

#[test]
fn an_alert_is_quoted() {
    assert_eq!(dialog_line(Asked::Alert, "Saved", 1), "This page says: \"Saved\"");
    assert_eq!(dialog_line(Asked::Alert, "  ", 1), "This page showed an alert with no text.");
}

#[test]
fn a_question_is_quoted_and_answered_no() {
    let line = dialog_line(Asked::Confirm, "Leave this page?", 1);
    assert_eq!(
        line,
        "This page asked \"Leave this page?\". \
         This browser cannot wait for an answer, so it answered No."
    );
    let line = dialog_line(Asked::Prompt, "Your name?", 1);
    assert!(line.starts_with("This page asked for text: \"Your name?\"."), "{line}");
    assert!(line.ends_with("so it gave none."), "{line}");
    for kind in [Asked::Confirm, Asked::Prompt] {
        let line = dialog_line(kind, "Delete everything?", 1).to_lowercase();
        assert!(!line.contains("yes") && !line.contains("agreed"), "never a yes: {line}");
    }
}

#[test]
fn a_message_is_one_line_and_cut_at_its_length() {
    assert_eq!(dialog_line(Asked::Alert, "two\n\tlines\r\n", 1), "This page says: \"two lines\"");
    let long = "\u{e9}".repeat(QUOTED + 40);
    let line = dialog_line(Asked::Alert, &long, 1);
    let want = format!("This page says: \"{}...\"", "\u{e9}".repeat(QUOTED));
    assert_eq!(line, want, "cut on a character, not a byte");
}

#[test]
fn earlier_dialogs_are_counted() {
    assert!(
        dialog_line(Asked::Alert, "b", 2).ends_with("(1 earlier message from it is not shown.)")
    );
    assert!(
        dialog_line(Asked::Alert, "c", 4).ends_with("(3 earlier messages from it are not shown.)")
    );
}
