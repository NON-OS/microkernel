// NONOS Operating System (AGPL-3.0-or-later)
//! Nothing the keyboard could not have typed is put on the prompt.

use super::reply;
use crate::handed_parse::{parse, Handed};
use crate::term::dimensions::LINE_MAX;

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
