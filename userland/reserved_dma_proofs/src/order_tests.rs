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
fn ranges_go_in_before_the_device_and_come_out_if_it_does_not() {
    order(
        &code("hardware/broker/confine/attach.rs"),
        &[
            "let added = match reserve(&mut all[pos], address) {",
            "Err(why) => return refuse(&mut all, pos, why, pid, address),",
            "let _ = all[pos].domain.detach_device(address);",
            "if all[pos].domain.attach_device(address).is_err() {",
            "unreserve(&mut all[pos], added);",
        ],
    );
    order(&code("hardware/broker/confine/reserve.rs"), &[
        "if let Some(why) = found.unresolved {",
        "filter(|r| !c.reserved.contains(r))",
        "held.push((IOVA_BASE, c.next_iova));",
        "admit(&ranges, &held, crate::arch::reserved_memory_regions()).map_err(|r| r.reason())?;",
        "c.domain.map(",
        "for done in &ranges[..i] {",
        "c.reserved.extend(ranges);",
    ]);
    let un = code("hardware/broker/confine/unreserve.rs");
    assert!(un.contains("c.reserved.split_off(c.reserved.len().saturating_sub(added))"));
    assert!(un.contains("c.domain.unmap(range.iova, range.len as usize)"));
}

#[test]
fn every_grant_is_placed_past_them() {
    let iova = code("hardware/broker/confine/iova.rs");
    assert!(iova.contains("place_avoiding(c.next_iova, length, &spans)?"));
    assert!(!iova.contains("= place("), "a grant placed without the reserved ranges");
    assert!(code("hardware/broker/confine/map.rs").contains("let iova = iova::take(c, length)?;"));
}
