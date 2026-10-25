// NONOS Operating System (AGPL-3.0-or-later)

/*
 * A record or marker is read one byte past its length (the policy service's
 * restore/tick.rs, the wizard's keep/skip.rs), so a longer file reaches
 * these checks whole and is refused, never cut into a record that passes.
 */

use nonos_policy_proto::setup_record::{check_record, is_done, Answers, Host, Name, Record};
use nonos_policy_proto::setup_record::{Refused, Tier, ANSWERS_LEN, DONE};
use nonos_policy_proto::wallpapers_kept::ALL;

#[test]
fn a_record_one_byte_longer_is_refused_and_the_record_itself_is_taken() {
    let answers = Answers {
        keyboard_layout: 2,
        timezone: -5,
        wallpaper: 3,
        username: Name::new(b"ada").unwrap(),
        qwen_tier: Tier::new(b"small").unwrap(),
    };
    let r = Record { answers, apps_off: 1, hostname: Host::EMPTY, route: 0, wallpapers_kept: ALL };
    let raw = r.encode();
    assert_eq!(raw.len(), ANSWERS_LEN);
    assert_eq!(check_record(&raw), Ok(r), "ANSWERS_LEN taken");
    let longer = [raw.as_slice(), &[0]].concat();
    assert_eq!(check_record(&longer), Err(Refused::Length), "ANSWERS_LEN + 1 refused");
    assert_eq!(Record::decode(&longer), None);
}

#[test]
fn a_done_marker_one_byte_longer_is_not_done() {
    assert!(is_done(&DONE), "the marker taken");
    assert!(!is_done(&[DONE.as_slice(), &[0]].concat()), "one byte longer refused");
}
