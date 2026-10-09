// NONOS Operating System (AGPL-3.0-or-later)
//! The cookie jar: parsing, matching, bounds, and one jar per network.

use crate::browser::cookie::absorb::set_cookie_lines;
use crate::browser::cookie::civil::unix;
use crate::browser::cookie::clock::{unix_secs, FLOOR};
use crate::browser::cookie::date::cookie_date;
use crate::browser::cookie::jar::{Jar, MAX_COOKIES, MAX_PER_DOMAIN};
use crate::browser::cookie::jars::Jars;
use crate::browser::cookie::matching::{default_path, domain_match, path_match};
use crate::browser::cookie::parse::{parse, Origin, MAX_NAME_VALUE};
use crate::browser::net::mixnet::Network;

/* 2026-10-03T12:00:00Z. */
const NOW: i64 = 1_791_028_800;

fn https(host: &str) -> Origin<'_> {
    Origin { host, path: "/account/login", secure: true }
}

fn http(host: &str) -> Origin<'_> {
    Origin { host, path: "/", secure: false }
}

fn store(jar: &mut Jar, line: &str, at: &Origin) -> bool {
    match parse(line, at, NOW, false) {
        Some(c) => jar.set(c, at.secure, false, NOW),
        None => false,
    }
}

#[test]
fn the_three_date_formats_servers_use_all_read() {
    let want = unix(2015, 10, 21, 7, 28, 0);
    assert_eq!(want, Some(1_445_412_480));
    assert_eq!(cookie_date("Wed, 21 Oct 2015 07:28:00 GMT"), want, "RFC 1123");
    assert_eq!(cookie_date("Wednesday, 21-Oct-15 07:28:00 GMT"), want, "RFC 850");
    assert_eq!(cookie_date("Wed Oct 21 07:28:00 2015"), want, "asctime");
    assert_eq!(cookie_date("Thu, 01 Jan 1970 00:00:00 GMT"), Some(0));
}

#[test]
fn a_date_that_does_not_exist_is_no_date() {
    for bad in ["", "tomorrow", "Wed, 31 Feb 2015 07:28:00 GMT", "21 Oct 2015", "Wed, 21 Oct 2015 25:00:00"] {
        assert_eq!(cookie_date(bad), None, "{bad}");
    }
    assert_eq!(cookie_date("Sat, 29 Feb 2020 00:00:00 GMT"), unix(2020, 2, 29, 0, 0, 0));
}

#[test]
fn domains_match_at_a_label_boundary_only() {
    assert!(domain_match("example.com", "example.com"));
    assert!(domain_match("www.example.com", "example.com"));
    assert!(domain_match("WWW.Example.COM", "example.com"));
    assert!(!domain_match("badexample.com", "example.com"));
    assert!(!domain_match("example.com", "www.example.com"));
    assert!(!domain_match("1.2.3.4", "2.3.4"), "an address is not under a shorter one");
}

#[test]
fn paths_match_at_a_slash_boundary_only() {
    assert!(path_match("/shop", "/shop"));
    assert!(path_match("/shop/cart", "/shop"));
    assert!(path_match("/shop/cart", "/shop/"));
    assert!(!path_match("/shopping", "/shop"));
    assert!(path_match("/anything", "/"));
    assert_eq!(default_path("/account/login"), "/account");
    assert_eq!(default_path("/login"), "/");
    assert_eq!(default_path("/a/b/?q=/x"), "/a/b");
    assert_eq!(default_path(""), "/");
}

#[test]
fn a_header_line_reads_every_attribute() {
    let line = "sid=abc123; Path=/; Domain=.Example.com; Secure; HttpOnly; Max-Age=3600; SameSite=Lax";
    let c = parse(line, &https("www.example.com"), NOW, false).expect("a cookie");
    assert_eq!((c.name.as_str(), c.value.as_str()), ("sid", "abc123"));
    assert_eq!((c.domain.as_str(), c.host_only), ("example.com", false));
    assert_eq!(c.path, "/");
    assert!(c.secure && c.http_only);
    assert_eq!(c.expires, Some(NOW + 3600));
}

#[test]
fn without_domain_or_path_a_cookie_is_host_only_under_the_request_path() {
    let c = parse("a=1", &https("www.example.com"), NOW, false).expect("a cookie");
    assert_eq!((c.domain.as_str(), c.host_only, c.path.as_str()), ("www.example.com", true, "/account"));
    assert_eq!(c.expires, None, "a session cookie");
}

#[test]
fn max_age_wins_over_expires_and_zero_deletes() {
    let both = "a=1; Expires=Wed, 21 Oct 2099 07:28:00 GMT; Max-Age=60";
    assert_eq!(parse(both, &https("a.example"), NOW, false).expect("c").expires, Some(NOW + 60));
    let gone = parse("a=1; Max-Age=0", &https("a.example"), NOW, false).expect("c");
    assert!(gone.expired(NOW));
    let neg = parse("a=1; Max-Age=-5", &https("a.example"), NOW, false).expect("c");
    assert!(neg.expired(NOW));
    let junk = parse("a=1; Max-Age=soon", &https("a.example"), NOW, false).expect("c");
    assert_eq!(junk.expires, None, "a Max-Age that is not a number is ignored");
}

#[test]
fn what_must_be_refused_is_refused() {
    let at = https("www.example.com");
    assert!(parse("noequals", &at, NOW, false).is_none(), "no name=value pair");
    assert!(parse("=v", &at, NOW, false).is_none(), "no name");
    assert!(parse("a=1; Domain=other.com", &at, NOW, false).is_none(), "another site");
    assert!(parse("a=1; Domain=com", &at, NOW, false).is_none(), "a whole top-level domain");
    assert!(parse("a=1; Secure", &http("www.example.com"), NOW, false).is_none(), "Secure over http");
    assert!(parse("a=1; HttpOnly", &at, NOW, true).is_none(), "a script setting HttpOnly");
    assert!(parse("a=1\r\nX-Evil: 1", &at, NOW, false).is_none(), "a control character");
    let big = alloc::format!("a={}", "x".repeat(MAX_NAME_VALUE));
    assert!(parse(&big, &at, NOW, false).is_none(), "longer than 4096 bytes");
}

#[test]
fn the_name_prefixes_mean_what_they_promise() {
    let at = https("www.example.com");
    assert!(parse("__Secure-a=1", &at, NOW, false).is_none());
    assert!(parse("__Secure-a=1; Secure", &at, NOW, false).is_some());
    assert!(parse("__Host-a=1; Secure; Path=/", &at, NOW, false).is_some());
    assert!(parse("__Host-a=1; Secure; Path=/; Domain=example.com", &at, NOW, false).is_none());
    assert!(parse("__Host-a=1; Secure", &at, NOW, false).is_none(), "the default path is not /");
}

#[test]
fn a_request_carries_what_matches_longest_path_first() {
    let mut jar = Jar::new();
    let at = https("www.example.com");
    assert!(store(&mut jar, "root=1; Path=/", &at));
    assert!(store(&mut jar, "deep=2; Path=/account", &at));
    assert!(store(&mut jar, "wide=3; Domain=example.com; Path=/", &at));
    assert!(store(&mut jar, "safe=4; Secure; Path=/", &at));
    assert!(store(&mut jar, "hidden=5; HttpOnly; Path=/", &at));
    let header = jar.header("www.example.com", "/account/x", true, false, NOW);
    assert_eq!(header, "deep=2; root=1; wide=3; safe=4; hidden=5");
    assert_eq!(jar.header("www.example.com", "/", false, false, NOW), "root=1; wide=3; hidden=5");
    assert_eq!(jar.header("api.example.com", "/", true, false, NOW), "wide=3", "host-only stays home");
    assert_eq!(jar.header("www.example.com", "/", true, true, NOW), "root=1; wide=3; safe=4");
    assert_eq!(jar.header("example.org", "/", true, false, NOW), "");
}

#[test]
fn a_cookie_is_replaced_and_deleted_in_place() {
    let mut jar = Jar::new();
    let at = https("a.example");
    store(&mut jar, "a=1; Path=/", &at);
    store(&mut jar, "b=2; Path=/", &at);
    store(&mut jar, "a=3; Path=/", &at);
    assert_eq!(jar.header("a.example", "/", true, false, NOW), "a=3; b=2", "kept its creation order");
    store(&mut jar, "a=; Path=/; Expires=Thu, 01 Jan 1970 00:00:00 GMT", &at);
    assert_eq!(jar.header("a.example", "/", true, false, NOW), "b=2", "the logout cookie deletes");
    let soon = parse("c=1; Path=/; Max-Age=10", &at, NOW, false).expect("c");
    jar.set(soon, true, false, NOW);
    assert_eq!(jar.header("a.example", "/", true, false, NOW + 11), "b=2", "expired on time");
}

#[test]
fn neither_a_script_nor_plain_http_overwrites_what_it_may_not() {
    let mut jar = Jar::new();
    let at = https("a.example");
    store(&mut jar, "s=server; Path=/; HttpOnly", &at);
    let from_script = parse("s=script; Path=/", &at, NOW, true).expect("c");
    assert!(!jar.set(from_script, true, true, NOW), "a script cannot replace an HttpOnly cookie");
    store(&mut jar, "t=secure; Path=/; Secure", &at);
    let plain = parse("t=plain; Path=/", &http("a.example"), NOW, false).expect("c");
    assert!(!jar.set(plain, false, false, NOW), "http cannot replace a Secure cookie");
    assert_eq!(jar.header("a.example", "/", true, false, NOW), "s=server; t=secure");
}

#[test]
fn the_jar_is_bounded_per_domain_and_in_all() {
    let mut jar = Jar::new();
    let at = https("one.example");
    for i in 0..MAX_PER_DOMAIN + 10 {
        store(&mut jar, &alloc::format!("c{i}=v; Path=/"), &at);
    }
    let one = jar.header("one.example", "/", true, false, NOW);
    assert_eq!(one.split("; ").count(), MAX_PER_DOMAIN);
    assert!(!one.contains("c0=") && one.contains(&alloc::format!("c{}=", MAX_PER_DOMAIN + 9)));
    for d in 0..10 {
        let host = alloc::format!("d{d}.example");
        for i in 0..40 {
            store(&mut jar, &alloc::format!("k{i}=v; Path=/"), &https(&host));
        }
    }
    let total: usize = (0..10)
        .map(|d| alloc::format!("d{d}.example"))
        .chain(core::iter::once(alloc::string::String::from("one.example")))
        .map(|h| {
            let s = jar.header(&h, "/", true, false, NOW);
            if s.is_empty() { 0 } else { s.split("; ").count() }
        })
        .sum();
    assert_eq!(total, MAX_COOKIES, "the oldest went first once the jar was full");
    assert_eq!(jar.header("one.example", "/", true, false, NOW), "", "and the oldest were these");
}

#[test]
fn a_cookie_set_on_one_network_is_never_sent_on_another() {
    let mut jars = Jars::new();
    let at = https("site.example");
    let c = parse("track=me; Path=/", &at, NOW, false).expect("c");
    jars.of(Network::Nym).set(c, true, false, NOW);
    assert_eq!(jars.get(Network::Nym).header("site.example", "/", true, false, NOW), "track=me");
    assert_eq!(jars.get(Network::Direct).header("site.example", "/", true, false, NOW), "");
    assert_eq!(jars.get(Network::Anyone).header("site.example", "/", true, false, NOW), "");
}

#[test]
fn every_set_cookie_line_of_a_response_is_read() {
    let raw = b"HTTP/1.1 302 Found\r\nLocation: /home\r\nSet-Cookie: a=1; Path=/\r\nset-cookie: b=2\r\nContent-Length: 0\r\n\r\n";
    assert_eq!(set_cookie_lines(raw), ["a=1; Path=/", "b=2"]);
    assert!(set_cookie_lines(b"HTTP/1.1 200 OK\r\nSet-Cookie: a=1").is_empty(), "no whole head");
    let body = b"HTTP/1.1 200 OK\r\n\r\nSet-Cookie: x=1\r\n\r\n";
    assert!(set_cookie_lines(body).is_empty(), "a body line is not a header");
}

#[test]
fn an_unset_clock_still_deletes_with_a_past_date() {
    assert_eq!(unix_secs(5_000), FLOOR, "a clock never set reads as the floor");
    assert_eq!(unix_secs(NOW * 1000), NOW);
    let mut jar = Jar::new();
    let at = https("a.example");
    let c = parse("a=1; Path=/", &at, FLOOR, false).expect("c");
    jar.set(c, true, false, FLOOR);
    let gone = parse("a=; Path=/; Expires=Thu, 01 Jan 1970 00:00:00 GMT", &at, FLOOR, false).expect("c");
    jar.set(gone, true, false, FLOOR);
    assert_eq!(jar.header("a.example", "/", true, false, FLOOR), "");
}
