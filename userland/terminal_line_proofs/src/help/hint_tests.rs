// NONOS Operating System (AGPL-3.0-or-later)
//! The hint bar teaches only what the keys do, and says the same as `help keys`.

use super::help_pages::KEYS;
use super::hint_sets::{hints, mode, Mode, FIND, PROMPT, RUNNING, SEARCH};

fn keys_page() -> String {
    KEYS.iter().map(|(_, l)| String::from_utf8_lossy(l).into_owned()).collect::<Vec<_>>().join(" ")
}

fn says(set: &[(&str, &str)], key: &str, what: &str) -> bool {
    set.iter().any(|(k, l)| *k == key && *l == what)
}

#[test]
fn ctrl_k_is_taught_as_the_palette_never_as_a_cut() {
    assert!(says(PROMPT, "Ctrl+K", "commands"));
    assert!(!PROMPT.iter().any(|(k, l)| k.contains('K') && l.contains("kill")));
    let page = keys_page();
    assert!(page.contains("Ctrl+K commands,") && page.contains("Ctrl+Shift+K to end"));
    assert!(!page.contains("Ctrl-K") && !page.contains("Ctrl+K to end"), "{page}");
}

#[test]
fn each_state_names_its_way_out() {
    assert!(says(SEARCH, "Esc", "back to your line"));
    assert!(says(SEARCH, "Enter", "run"));
    assert!(says(FIND, "Esc", "close"));
    assert!(says(RUNNING, "Ctrl+C", "stop"));
}

#[test]
fn every_key_the_bar_names_is_on_the_keys_page() {
    let page = keys_page();
    for set in [PROMPT, SEARCH, FIND] {
        for (key, _) in set {
            let known = page.contains(key) || ["Bksp", "Alt+C", "help keys"].contains(key);
            assert!(known, "the bar names {key} and help keys does not");
        }
    }
}

#[test]
fn a_running_program_outranks_the_line_states() {
    assert_eq!(mode(true, true, true), Mode::Running);
    assert_eq!(mode(false, true, true), Mode::Find);
    assert_eq!(mode(false, false, true), Mode::Search);
    assert_eq!(mode(false, false, false), Mode::Prompt);
    assert_eq!(hints(Mode::Search), SEARCH);
    assert_eq!(hints(Mode::Prompt), PROMPT);
    assert_eq!(hints(Mode::Find), FIND);
    assert_eq!(hints(Mode::Running), RUNNING);
}
