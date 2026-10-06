// NONOS Operating System (AGPL-3.0-or-later)
//! A command the shell hands the Terminal from a Launchpad tool tile. Only a
//! `run:` or `type:` reply is a command, a path never is, and nothing the
//! keyboard could not have typed is put on the prompt. The command lands in
//! the untouched tab the tile just opened, or a tab of its own, and never
//! over a program or a half-typed line.

use crate::handed_cadence::{next_ask, EVERY_MS, STARTUP_MS};
use crate::handed_parse::{parse, Handed, RUN, TYPE};
use crate::handed_pick::{pick, Active, Pick};
use crate::term::dimensions::LINE_MAX;

/// A reply body as the shell sends it: the 4-byte status, then the argument.
fn reply(arg: &[u8]) -> Vec<u8> {
    let mut body = vec![0u8; 4];
    body.extend_from_slice(arg);
    body
}

#[test]
fn run_is_run_as_typed() {
    assert_eq!(parse(&reply(b"run:pastel")), Some(Handed::Run(b"pastel")));
    assert_eq!(parse(&reply(b"run:tokei -s lines")), Some(Handed::Run(b"tokei -s lines")));
}

#[test]
fn type_is_left_on_the_prompt_with_its_trailing_space() {
    assert_eq!(parse(&reply(b"type:grex ")), Some(Handed::Type(b"grex ")));
}

#[test]
fn a_bare_status_or_a_short_reply_is_nothing() {
    assert_eq!(parse(&[0u8; 4]), None);
    assert_eq!(parse(&[0u8; 2]), None);
    assert_eq!(parse(&[]), None);
}

#[test]
fn a_path_is_never_a_command() {
    for path in [&b"/home/nonos/run:x"[..], b"/bin/grex", b"/"] {
        assert_eq!(parse(&reply(path)), None, "{path:?}");
    }
}

#[test]
fn only_the_two_prefixes_count_and_only_exactly() {
    for odd in [&b"grex"[..], b"RUN:grex", b"run grex", b" run:grex", b"runs:grex", b"typ:grex"] {
        assert_eq!(parse(&reply(odd)), None, "{odd:?}");
    }
    assert!(RUN != TYPE && !RUN.starts_with(b"/") && !TYPE.starts_with(b"/"));
}

#[test]
fn an_empty_or_blank_command_runs_nothing() {
    assert_eq!(parse(&reply(b"run:")), None);
    assert_eq!(parse(&reply(b"run:   ")), None);
    assert_eq!(parse(&reply(b"type:")), None);
}

#[test]
fn a_control_byte_is_refused_so_a_second_line_cannot_ride_along() {
    for bad in [&b"run:grex\nrm -r /"[..], b"run:grex\r", b"type:a\x1b[2J", b"run:a\x7f", b"run:\0"]
    {
        assert_eq!(parse(&reply(bad)), None, "{bad:?}");
    }
}

#[test]
fn a_line_that_is_not_utf8_or_too_long_for_the_prompt_is_refused() {
    assert_eq!(parse(&reply(b"run:\xff\xfe")), None);
    let mut long = b"run:".to_vec();
    long.extend(std::iter::repeat_n(b'a', LINE_MAX + 1));
    assert_eq!(parse(&reply(&long)), None);
    let mut fits = b"run:".to_vec();
    fits.extend(std::iter::repeat_n(b'a', LINE_MAX));
    assert!(parse(&reply(&fits)).is_some());
    assert_eq!(
        parse(&reply("type:echo café ".as_bytes())),
        Some(Handed::Type("echo café ".as_bytes()))
    );
}

const MAX: usize = 9;

#[test]
fn the_untouched_tab_a_tile_just_opened_takes_the_command() {
    let fresh = Active { fresh: true, idle: true };
    assert_eq!(pick(fresh, 1, MAX), Pick::Current);
    assert_eq!(pick(fresh, MAX, MAX), Pick::Current);
}

#[test]
fn a_used_tab_is_kept_and_the_command_gets_its_own() {
    for active in [
        Active { fresh: false, idle: true },
        Active { fresh: false, idle: false },
        // Something typed on the splash, or a program running from it.
        Active { fresh: true, idle: false },
    ] {
        assert_eq!(pick(active, 1, MAX), Pick::NewTab);
        assert_eq!(pick(active, MAX - 1, MAX), Pick::NewTab);
    }
}

#[test]
fn with_every_tab_taken_only_an_idle_prompt_will_do() {
    assert_eq!(pick(Active { fresh: false, idle: true }, MAX, MAX), Pick::Current);
    assert_eq!(pick(Active { fresh: false, idle: false }, MAX, MAX), Pick::Nowhere);
    assert_eq!(pick(Active { fresh: true, idle: false }, MAX, MAX), Pick::Nowhere);
}

#[test]
fn a_new_window_asks_every_tick_then_twice_a_second() {
    let start = 50_000;
    assert_eq!(next_ask(start, start), start);
    assert_eq!(next_ask(start + STARTUP_MS - 1, start), start + STARTUP_MS - 1);
    assert_eq!(next_ask(start + STARTUP_MS, start), start + STARTUP_MS + EVERY_MS);
    assert_eq!(EVERY_MS, 500);
    // A clock read before the start does not stretch the quick phase forever.
    assert_eq!(next_ask(start - 10, start), start - 10);
    assert_eq!(next_ask(i64::MAX, 0), i64::MAX);
}
