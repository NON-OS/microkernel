// NONOS Operating System (AGPL-3.0-or-later)
//! Only a `run:` or `type:` reply is a command, and a path never is.

use super::reply;
use crate::handed_parse::{parse, Handed, RUN, TYPE};

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
