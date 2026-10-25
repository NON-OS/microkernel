// NONOS Operating System (AGPL-3.0-or-later)
//! The order a claim keeps, read from the kernel's source: the ranges
//! admitted then mapped before the device leaves the identity domain, all
//! or nothing, taken back if the attach fails, and every grant placed past
//! them.

fn code(path: &str) -> String {
    let src = std::fs::read_to_string(format!("../../src/{path}")).unwrap();
    src.lines().map(|l| l.split("//").next().unwrap_or("").trim()).collect::<Vec<_>>().join("\n")
}

fn order(src: &str, marks: &[&str]) {
    let mut from = 0;
    for mark in marks {
        let at = src[from..].find(mark).unwrap_or_else(|| panic!("no `{mark}` after {from}"));
        from += at + mark.len();
    }
}

#[test]
fn ranges_go_in_before_the_device_and_come_out_after_it() {
    order(
        &code("hardware/broker/confine/join.rs"),
        &[
            "if let Err(why) = reserve(&mut all[pos], address) {",
            "return refuse(all, pos, why, Refusal::Refused);",
            "let _ = all[pos].domain.detach_device(address);",
            "if all[pos].domain.attach_device(address).is_err() {",
            "release(&mut all[pos], address);",
        ],
    );
    order(&code("hardware/broker/confine/reserve.rs"), &[
        "if let Some(why) = found.unresolved {",
        "let ranges = unheld(&c.reserved, found.ranges());",
        "held.push((IOVA_BASE, c.next_iova));",
        "admit(&ranges, &held, crate::arch::reserved_memory_regions()).map_err(|r| r.reason())?;",
        "c.domain.map(",
        "for done in &ranges[..i] {",
        "name(&mut c.reserved, found.ranges(), device);",
    ]);
    // Out of the domain first, then its last-named ranges; not while another
    // device behind the same requester id is still attached.
    // A failed detach keeps the domain.
    let detach = code("hardware/broker/confine/detach.rs");
    order(&detach, &["kept |= !denied;", "if !kept && all[pos].devices.is_empty() {"]);
    order(
        &detach,
        &[
            "if c.devices.iter().any(|(_, a)| *a == address) {",
            "return Some(true);",
            "if c.domain.detach_device(address).is_err() {",
            "return Some(false);",
            "super::unreserve::release(c, address);",
        ],
    );
    let un = code("hardware/broker/confine/unreserve.rs");
    assert!(un.contains("for range in unname(&mut c.reserved, device) {"));
    assert!(un.contains("c.domain.unmap(range.iova, range.len as usize)"));
}

#[test]
fn every_grant_is_placed_past_them() {
    let iova = code("hardware/broker/confine/iova.rs");
    assert!(iova.contains("place_avoiding(c.next_iova, length, &spans)?"));
    assert!(!iova.contains("= place("), "a grant placed without the reserved ranges");
    // A grant placed by `take`, or refused with nothing mapped (map_fail.rs).
    let map = code("hardware/broker/confine/map.rs");
    assert!(map.contains("let iova = iova::take(c, length).ok_or(NoAddress::Failed)?;"));
}
