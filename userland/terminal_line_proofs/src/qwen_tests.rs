// NONOS Operating System (AGPL-3.0-or-later)
//! `qwen window [tier]` as typed: which lines ask for a window, what the
//! kernel is sent for one, and that every word the terminal offers is a
//! word the kernel's allowlist takes.

use super::ask::parse as ask;
use super::complete::words;
use super::kernel_tier::{argv, package, parse as kernel_parse};
use super::tiers::TIERS;
use super::window::{parse, request, Window, WORD};

fn window(line: &[u8]) -> Option<Window<'_>> {
    parse(&ask(line).expect("a qwen line"))
}

#[test]
fn window_opens_the_named_tier_or_the_first() {
    assert_eq!(window(b"qwen window"), Some(Window::Open(b"small")));
    assert_eq!(window(b"  qwen   window  \t"), Some(Window::Open(b"small")));
    for tier in TIERS {
        let line = [&b"qwen window "[..], tier].concat();
        assert_eq!(window(&line), Some(Window::Open(*tier)));
    }
}

#[test]
fn only_the_first_word_after_qwen_asks_for_a_window() {
    assert_eq!(window(b"qwen windows are nice?"), None);
    assert_eq!(window(b"qwen small window please"), None);
    assert_eq!(window(b"qwen what is a window"), None);
    assert_eq!(window(b"qwen window huge"), Some(Window::NotATier(b"huge")));
    assert_eq!(window(b"qwen window small now"), Some(Window::NotATier(b"small now")));
}

#[test]
fn the_kernel_gets_the_word_a_nul_and_a_tier_it_allows() {
    assert_eq!(request(b"coder-7b"), b"window\0coder-7b");
    for (i, tier) in TIERS.iter().enumerate() {
        let sent = request(tier);
        let word = sent.strip_prefix(WORD).and_then(|r| r.strip_prefix(b"\0")).unwrap();
        let index = kernel_parse(word).expect("the kernel takes every offered tier");
        assert_eq!(usize::from(index), i);
        let name = format!("qwen-{}", core::str::from_utf8(tier).unwrap());
        assert_eq!(package(index).as_deref(), Some(name.as_str()));
        assert_eq!(argv(index).unwrap(), ["run", name.as_str(), "cli"]);
    }
    assert_eq!(kernel_parse(b"huge"), None);
    assert_eq!(package(TIERS.len() as u8), None);
}

#[test]
fn tab_offers_window_and_tiers_after_qwen_and_tiers_after_window() {
    let after_qwen = words(b"qwen ", b"").unwrap();
    assert_eq!(after_qwen[0], WORD);
    assert_eq!(&after_qwen[1..], TIERS);
    assert_eq!(words(b"qwen ", b"wi").unwrap(), [WORD]);
    assert_eq!(words(b"qwen window ", b"").unwrap(), TIERS);
    assert_eq!(words(b"qwen window ", b"coder-1").unwrap(), [&b"coder-1.5b"[..], b"coder-14b"]);
    assert_eq!(words(b"qwen small ", b""), None);
    assert_eq!(words(b"ls ", b""), None);
}
