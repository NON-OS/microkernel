// NONOS Operating System (AGPL-3.0-or-later)
/*
 * The tier `qwen` runs with none named: the one policy holds, if it is a
 * tier; else the default (`default_tier`), which is said. A typed tier
 * always wins.
 */

use super::ask::parse;
use super::default_tier::{resolve, Source};
use super::need::Room;
use super::tiers::{default_line as said, pick_said, TIERS, UNANSWERED, UNKNOWN};

#[test]
fn a_typed_tier_still_wins() {
    let a = parse(b"qwen large what is 2+2").unwrap();
    assert_eq!(a.tier(), b"large");
}

/*
 * What the policy service answers, read on every qwen command: its leading
 * tier word, so a padded or cut reply still names the tier Settings chose.
 */
#[test]
fn a_policy_reply_is_read_to_its_tier_word() {
    let chosen = |r: &[u8]| pick_said(Some(r)).0;
    assert_eq!(chosen(b"qwen3-4b"), Some(&b"qwen3-4b"[..]));
    assert_eq!(chosen(b"qwen3-0.6b\0\0junk"), Some(&b"qwen3-0.6b"[..]));
    assert_eq!(chosen(b"small "), Some(&b"small"[..]));
    assert_eq!(chosen(b"coder-32b\n"), Some(&b"coder-32b"[..]));
    for t in TIERS {
        assert_eq!(chosen(t), Some(*t));
    }
}

/*
 * Nothing chosen runs the default with nothing to add; an answer that could
 * not be had, or a tier this terminal does not run, says so as well.
 */
#[test]
fn running_another_tier_than_the_one_chosen_is_said() {
    assert_eq!(pick_said(None), (None, Some(UNANSWERED)));
    assert_eq!(pick_said(Some(b"")), (None, None));
    assert_eq!(pick_said(Some(b"\0\0")), (None, None));
    assert_eq!(pick_said(Some(b"qwen9-1t")), (None, Some(UNKNOWN)));
    assert_eq!(pick_said(Some(b"QWEN3-4B")), (None, Some(UNKNOWN)));
    let (tier, source) = resolve(b"", None, Room::Memory);
    assert_eq!((tier, source), ("qwen3-0.6b", Source::Stick));
    let line = said(tier, source).unwrap();
    assert!(line.contains("no tier chosen") && line.contains("running the default, qwen3-0.6b"), "{line}");
    assert_eq!(said("qwen3-4b", Source::Chosen), None);
}

/* Every word the default rule can give is one this terminal runs. */
#[test]
fn the_default_is_a_tier_this_terminal_runs() {
    for t in super::default_tier::WEIGHTS {
        assert!(TIERS.contains(&t.0.as_bytes()), "{}", t.0);
    }
    assert_eq!(super::default_tier::WEIGHTS.len(), TIERS.len());
}
