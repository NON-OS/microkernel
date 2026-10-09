// NONOS Operating System (AGPL-3.0-or-later)
//! The `help <command>` table, checked for the ways a written table rots.
//!
//! Usage text is written rather than derived, because the flag tables live
//! inside each command's body. Written text drifts, so the shapes that would
//! make it wrong are asserted here instead of trusted.

/// The table as it appears in `command/builtin/help_one.rs`, parsed out of the
/// source rather than copied, so this cannot pass against a stale duplicate.
fn entries() -> Vec<(String, String, String)> {
    let src = include_str!("../../capsule_terminal/src/command/builtin/help_one.rs");
    let mut out = Vec::new();
    for line in src.lines() {
        let line = line.trim();
        if !line.starts_with("(b\"") {
            continue;
        }
        let parts: Vec<&str> = line.split("b\"").skip(1).collect();
        if parts.len() < 3 {
            continue;
        }
        let cut = |s: &str| s.rsplit_once('"').map(|(a, _)| a.to_string()).unwrap_or_default();
        out.push((cut(parts[0]), cut(parts[1]), cut(parts[2])));
    }
    out
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

/// Every name the dispatcher answers to, read from its match arms.
fn dispatched_names() -> Vec<String> {
    let src = include_str!("../../capsule_terminal/src/command/dispatch/exec.rs");
    let mut out = Vec::new();
    for line in src.lines() {
        let Some((head, _)) = line.split_once("=>") else { continue };
        if !head.trim_start().starts_with("b\"") {
            continue;
        }
        for part in head.split("b\"").skip(1) {
            if let Some((name, _)) = part.split_once('"') {
                out.push(name.to_string());
            }
        }
    }
    out
}

/// Every name Tab offers.
fn completed_names() -> Vec<String> {
    let src = include_str!("../../capsule_terminal/src/event/complete.rs");
    let block = src.split("const COMMANDS").nth(1).unwrap().split("];").next().unwrap();
    block
        .split("b\"")
        .skip(1)
        .filter_map(|p| p.split_once('"').map(|(n, _)| n.to_string()))
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
