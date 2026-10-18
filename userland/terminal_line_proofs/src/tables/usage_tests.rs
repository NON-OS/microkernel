// NONOS Operating System (AGPL-3.0-or-later)
//! The `help <command>` table, checked for the ways a written table rots.
//!
//! Usage text is written rather than derived, because the flag tables live
//! inside each command's body. Written text drifts, so the shapes that would
//! make it wrong are asserted here instead of trusted.

/// The table the capsule answers `help <command>` from, mounted as the value it
/// is, so this cannot pass against a stale duplicate.
fn entries() -> Vec<(String, String, String)> {
    let text = |b: &[u8]| String::from_utf8_lossy(b).into_owned();
    super::help_usage::table::USAGE
        .iter()
        .flat_map(|g| g.iter())
        .map(|(n, u, w)| (text(n), text(u), text(w)))
        .collect()
}

#[test]
fn the_table_was_found() {
    assert!(entries().len() >= 25, "parsed {} entries", entries().len());
}

#[test]
fn no_command_is_documented_twice() {
    let mut names: Vec<String> = entries().into_iter().map(|(n, _, _)| n).collect();
    names.sort();
    let before = names.len();
    names.dedup();
    assert_eq!(before, names.len(), "a command appears twice in the usage table");
}

/// A usage line that does not begin with the command is a copy/paste from the
/// row above, which is the way this kind of table usually breaks.
#[test]
fn every_usage_line_starts_with_its_own_command() {
    for (name, usage, _) in entries() {
        assert!(
            usage == name || usage.starts_with(&format!("{name} ")),
            "{name}: usage line reads {usage:?}"
        );
    }
}

#[test]
fn every_command_says_what_it_does() {
    for (name, _, what) in entries() {
        assert!(what.len() > 10, "{name}: description too short to be one");
        assert!(!what.ends_with('.'), "{name}: descriptions do not take a full stop");
    }
}

/// Nothing in the table may be wider than the terminal, or the line it
/// describes wraps and the column it lines up with stops meaning anything.
#[test]
fn nothing_overflows_the_terminal_width() {
    const COLS: usize = 96;
    for (name, usage, what) in entries() {
        assert!(usage.len() <= COLS, "{name}: usage is {} cols", usage.len());
        assert!(what.len() + 2 <= 118, "{name}: description is {} cols", what.len());
    }
}
