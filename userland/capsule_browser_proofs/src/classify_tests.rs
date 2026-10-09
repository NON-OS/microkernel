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
