// NONOS Operating System (AGPL-3.0-or-later)
#![cfg(test)]
//! Enter on the address bar: an address or a search. Before, an empty bar
//! and "rust borrow checker" loaded a "bad url" page, "rust" went to
//! https://rust/ and localhost:8080 was tried over TLS.

use crate::browser::omnibox::parts::{encode_query, host_kind, HostKind};
use crate::browser::omnibox::{classify, Nav};
use crate::browser::url::parse;

const S: &str = "https://search.example/?q=%s";

fn search(q: &str) -> Nav {
    Nav::Search(alloc::format!("https://search.example/?q={q}"))
}

#[test]
fn text_is_classified_as_a_browser_would() {
    assert_eq!(classify("", S), Nav::Nothing);
    assert_eq!(classify("   ", S), Nav::Nothing);
    assert_eq!(classify("rust borrow checker", S), search("rust+borrow+checker"));
    assert_eq!(classify("rust", S), search("rust"));
    assert_eq!(classify("localhost:8080", S), Nav::Url("http://localhost:8080".into()));
    assert_eq!(classify("about:blank", S), Nav::Internal("about:blank".into()));
    let q = "example.com/?u=https://x.org";
    assert_eq!(classify(q, S), Nav::Url(alloc::format!("https://{q}")));
    assert_eq!(classify("nonos.software", S), Nav::Url("https://nonos.software".into()));
    assert_eq!(classify("search.anyone/q", S), Nav::Url("http://search.anyone/q".into()));
    assert_eq!(classify("10.0.2.2:8088/x", S), Nav::Url("http://10.0.2.2:8088/x".into()));
    let full = "https://example.com/a?b=c";
    assert_eq!(classify(full, S), Nav::Url(full.into()));
    assert_eq!(classify("javascript:alert(1)", S), search("javascript%3Aalert%281%29"));
    assert_eq!(classify("\u{e4}\u{e4}", S), search("%C3%A4%C3%A4"));
}

#[test]
fn a_url_inside_a_query_is_not_a_scheme() {
    let u = parse("example.com/?u=https://x.org").expect("parses");
    assert_eq!(u.host, "example.com");
    assert_eq!(u.path, "/?u=https://x.org");
    assert!(parse("ftp://x.org").is_none());
    assert_eq!(parse("http://a.org/b").map(|u| u.host), Some("a.org".into()));
}

#[test]
fn host_recognition() {
    assert_eq!(host_kind("example.com"), Some(HostKind::Public));
    assert_eq!(host_kind("xn--bcher-kva.example"), Some(HostKind::Public));
    assert_eq!(host_kind("127.0.0.1:80"), Some(HostKind::Local));
    assert_eq!(host_kind("search.anyone"), Some(HostKind::Onion));
    assert_eq!(host_kind("SEARCH.Anyone:8080"), Some(HostKind::Onion));
    assert_eq!(host_kind("anyone"), None, "a bare word is a search");
    assert_eq!(host_kind("a.b.c.999"), None);
    assert_eq!(host_kind("file.txt2"), None);
    assert_eq!(host_kind("-bad.com"), None);
    assert_eq!(host_kind("example.com:0"), None);
    assert_eq!(encode_query("a b&c"), "a+b%26c");
}

#[test]
fn a_typed_address_with_a_scheme_or_a_port_is_gone_to_never_searched() {
    for typed in ["file:///notes.txt", "ftp://files.example.org/a", "http://10.0.2.2:9/notes.txt"] {
        assert_eq!(classify(typed, S), Nav::Url(typed.into()), "{typed}");
    }
    assert_eq!(classify("nas:5000", S), Nav::Url("http://nas:5000".into()));
    assert_eq!(classify("nas:5000/admin", S), Nav::Url("http://nas:5000/admin".into()));
    // No authority, no navigation; with one, the load refuses any scheme
    // but http(s) as a bad url (fetch/nav/load.rs) and runs nothing.
    assert_eq!(classify("data:text/html,hi", S), search("data%3Atext%2Fhtml%2Chi"));
    assert_eq!(classify("javascript://x", S), Nav::Url("javascript://x".into()));
    assert_eq!(classify("nas:0", S), search("nas%3A0"));
    assert_eq!(classify("nas:", S), search("nas%3A"));
}

#[test]
fn an_address_of_another_scheme_is_refused_as_not_a_web_address() {
    use crate::browser::url::refusal;
    assert_eq!(refusal("file:///notes.txt"), "not a web address");
    assert_eq!(refusal("ftp://files.example.org/a"), "not a web address");
    assert_eq!(refusal("http://"), "bad url", "a web scheme it could not read");
    assert_eq!(refusal("://x"), "bad url");
}
