// NONOS Operating System (AGPL-3.0-or-later)
/*
 * `qwen get TIER...` and `qwen tiers` as typed: which lines ask for them,
 * what the model fetcher is started with, what history keeps of them, and
 * what Tab offers after `get`.
 */

use super::ask::parse as ask;
use super::complete::words;
use super::fetch_check::{kept, refusal};
use super::fetch_words::{parse, request, Fetch};
use super::tiers::TIERS;

fn fetch(line: &[u8]) -> Option<Fetch<'_>> {
    parse(&ask(line).expect("a qwen line"))
}

#[test]
fn get_and_tiers_are_only_the_first_word_after_qwen() {
    assert_eq!(fetch(b"qwen get small  qwen3-8b"), Some(Fetch::Get(vec![b"small", b"qwen3-8b"])));
    assert_eq!(fetch(b"qwen get"), Some(Fetch::Get(vec![])));
    assert_eq!(fetch(b" qwen   tiers "), Some(Fetch::List));
    assert_eq!(fetch(b"qwen tiers of what?"), None);
    assert_eq!(fetch(b"qwen getting started"), None);
    assert_eq!(fetch(b"qwen small get me a plan"), None);
    assert_eq!(fetch(b"qwen what is get"), None);
}

#[test]
fn the_fetcher_gets_the_verb_then_each_tier_nul_separated() {
    assert_eq!(request(&Fetch::List), b"tiers");
    assert_eq!(request(&Fetch::Get(vec![b"xxl", b"coder-7b"])), b"get\0xxl\0coder-7b");
}

#[test]
fn tab_offers_every_tier_after_get() {
    assert_eq!(words(b"qwen get ", b"").unwrap(), TIERS);
    assert_eq!(
        words(b"qwen get small ", b"qwen3-3").unwrap(),
        [&b"qwen3-30b-a3b"[..], b"qwen3-32b"]
    );
    assert_eq!(words(b"qwen ", b"ti").unwrap(), [&b"tiers"[..]]);
}

#[test]
fn history_keeps_the_checked_words_and_never_a_refused_line() {
    let got = |line: &[u8]| kept(&fetch(line).expect("a fetch line"));
    assert_eq!(got(b"qwen  get small   qwen3-8b"), b"qwen get small qwen3-8b");
    assert_eq!(got(b"qwen tiers"), b"qwen tiers");
    assert_eq!(got(b"qwen get me my bank password"), b"qwen");
    assert_eq!(got(b"qwen get"), b"qwen");
    assert!(refusal(&fetch(b"qwen get small").unwrap()).is_none());
    assert!(refusal(&fetch(b"qwen get secret").unwrap()).is_some());
}
