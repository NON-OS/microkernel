// NONOS Operating System (AGPL-3.0-or-later)
//! A request carries its network's cookies and the browser's User-Agent.

use super::fetch_fixtures::url_of;
use super::fetch_wire::FakeWire;
use crate::browser::cookie::store::{
    absorb, page as page_net, request_header, script_get, script_set, set_page,
};
use crate::browser::fetch::open::open;
use crate::browser::fetch::run::run;
use crate::browser::fetch::types::Phase;
use crate::browser::http::request::{build, USER_AGENT};
use crate::browser::net::mixnet::{Network, Routes, Way};
use nonos_route_link::Proxy;

/* 2026-10-03T12:00:00Z. FakeWire's clock is far earlier, so requests are
 * judged at the floor; the cookies here have no expiry. */
const NOW: i64 = 1_791_028_800;

/// A way out over `net`, which is all a request's jar is read from.
fn way_of(net: Network) -> Way {
    match net {
        Network::Direct => Way::Direct,
        Network::Nym => Way::Proxy { net, port: 41, proxy: Proxy::Nym },
        Network::Anyone => Way::Proxy { net, port: 42, proxy: Proxy::Anyone },
    }
}

fn sent_for(target: &str, net: Network) -> Vec<u8> {
    let mut w = FakeWire::at(0);
    let mut f = open(&mut w, url_of(target), None).expect("open");
    f.way = way_of(net);
    run(&mut w, &mut f, 30);
    w.writable = true;
    run(&mut w, &mut f, 30);
    assert_eq!(f.phase, Phase::ReadBody);
    w.sent_on(f.handle)
}

fn has_line(sent: &[u8], line: &str) -> bool {
    sent.windows(line.len()).any(|w| w == line.as_bytes())
}

#[test]
fn a_cookie_a_response_set_rides_the_next_request_on_that_network_only() {
    let page = url_of("http://10.77.0.1:8000/login");
    let raw = b"HTTP/1.1 302 Found\r\nSet-Cookie: sid=n1; Path=/\r\nContent-Length: 0\r\n\r\n";
    absorb(Network::Anyone, &page, raw, NOW);
    let same = sent_for("http://10.77.0.1:8000/home", Network::Anyone);
    assert!(has_line(&same, "\r\nCookie: sid=n1\r\n"), "{}", String::from_utf8_lossy(&same));
    for other in [Network::Direct, Network::Nym] {
        let sent = sent_for("http://10.77.0.1:8000/home", other);
        assert!(!has_line(&sent, "Cookie:"), "never across networks");
    }
    let elsewhere = sent_for("http://10.77.0.2:8000/home", Network::Anyone);
    assert!(!has_line(&elsewhere, "Cookie:"), "nor to another host");
}

#[test]
fn every_request_says_it_is_the_common_firefox() {
    let sent = sent_for("http://10.77.0.3:8000/", Network::Direct);
    let ua = alloc::format!("\r\nUser-Agent: {USER_AGENT}\r\n");
    assert!(has_line(&sent, &ua));
    assert!(USER_AGENT.starts_with("Mozilla/5.0 (") && USER_AGENT.ends_with("Firefox/128.0"));
}

#[test]
fn a_request_without_cookies_has_no_cookie_line() {
    let req = build(&url_of("https://a.example/"), None, None);
    assert!(!req.contains("Cookie"));
    let req = build(&url_of("https://a.example/"), Some("q=1"), Some("a=1; b=2"));
    assert!(req.contains("\r\nCookie: a=1; b=2\r\n"));
    assert!(req.ends_with("\r\n\r\nq=1"), "the body still follows the head");
}

/// What a page with `routes` sends for `target` once the proxy has opened
/// the stream, and the fetch that sent it.
fn proxied_request(routes: Routes, target: &str) -> (Vec<u8>, crate::browser::fetch::types::Fetch) {
    let mut w = FakeWire::at(0);
    w.routes = Some(routes);
    let mut f = open(&mut w, url_of(target), None).expect("open");
    run(&mut w, &mut f, 30);
    w.deliver(f.handle, &[5, 0]);
    run(&mut w, &mut f, 30);
    w.deliver(f.handle, &[5, 0, 0, 1, 0, 0, 0, 0, 0, 0]);
    run(&mut w, &mut f, 30);
    assert_eq!(f.phase, Phase::ReadBody, "{target}");
    (w.sent_on(f.handle), f)
}

#[test]
fn an_anyone_page_with_nym_chosen_reads_and_writes_only_the_anyone_jar() {
    let routes = Routes::for_page("jar.anyone", Network::Nym, 41, 42);
    let page = url_of("http://jar.anyone/");
    let set = |v: &str| alloc::format!("HTTP/1.1 200 OK\r\nSet-Cookie: {v}; Path=/\r\n\r\n");
    absorb(Network::Anyone, &page, set("on_anyone=1").as_bytes(), NOW);
    absorb(Network::Nym, &page, set("on_nym=1").as_bytes(), NOW);
    absorb(Network::Direct, &page, set("on_direct=1").as_bytes(), NOW);

    /* The request reads the Anyone jar and no other. */
    let (sent, f) = proxied_request(routes, "http://jar.anyone/");
    assert_eq!(f.net(), Network::Anyone, "the jar is the way the bytes take");
    assert!(has_line(&sent, "\r\nCookie: on_anyone=1\r\n"), "{}", String::from_utf8_lossy(&sent));

    /* What its response sets goes to the Anyone jar (land::respond absorbs
     * into the fetch's own network), never Nym's or Direct's. */
    absorb(f.net(), &f.url, set("fresh=2").as_bytes(), NOW);
    let nym = request_header(Network::Nym, &page, NOW).unwrap_or_default();
    let direct = request_header(Network::Direct, &page, NOW).unwrap_or_default();
    assert!(!nym.contains("fresh") && !direct.contains("fresh"));

    /* The committed page's document.cookie reads and writes that jar too. */
    set_page(f.net());
    assert_eq!(page_net(), Network::Anyone);
    let seen = script_get(&page, NOW);
    assert!(seen.contains("on_anyone=1") && seen.contains("fresh=2") && !seen.contains("on_nym"));
    script_set(&page, "from_script=3", NOW);
    let anyone = request_header(Network::Anyone, &page, NOW).unwrap_or_default();
    assert!(anyone.contains("from_script=3"));
    let nym = request_header(Network::Nym, &page, NOW).unwrap_or_default();
    assert!(!nym.contains("from_script"), "a script on an onion page never writes Nym's jar");

    /* A clearnet image of that page rides Anyone, and so reads Anyone's jar. */
    let (_, img) = proxied_request(routes, "http://cdn.jar-test.example/a.png");
    assert_eq!(img.net(), Network::Anyone);
}

#[test]
fn a_page_whose_route_is_off_touches_no_jar() {
    let mut w = FakeWire::at(0);
    w.routes = Some(Routes::for_page("example.org", Network::Nym, 0, 0));
    assert!(open(&mut w, url_of("http://example.org/"), None).is_err(), "no fetch, no jar");
    assert!(w.ways.is_empty());
}

#[test]
fn a_fetchs_network_is_its_way_on_every_path() {
    for (chosen, host, want) in [
        (Network::Nym, "a.example", Network::Nym),
        (Network::Anyone, "a.example", Network::Anyone),
        (Network::Direct, "a.example", Network::Direct),
        (Network::Direct, "b.anyone", Network::Anyone),
        (Network::Nym, "b.anyone", Network::Anyone),
    ] {
        let mut w = FakeWire::at(0);
        w.routes = Some(Routes::for_page("a.example", chosen, 41, 42));
        let f = open(&mut w, url_of(&alloc::format!("http://{host}/")), None).expect("open");
        assert_eq!(f.net(), want, "{chosen:?} {host}");
    }
}
