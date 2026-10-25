// NONOS Operating System (AGPL-3.0-or-later)
//! The order a described device's claim and release keep, read from the
//! kernel's source: dispatched by kind, every stream in or none, refused
//! rather than unconfined when its unit is not in service, and every one
//! of its streams out at release.

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
fn a_described_device_is_confined_or_refused() {
    order(
        &code("hardware/broker/confine/attach.rs"),
        &[
            "let Some(address) = pci_address(device_id) else {",
            "if let Some(iommus) = crate::hardware::broker::described::iommus(device_id) {",
            "return Ok(attach_streams(pid, device_id, iommus, port_only)?);",
            "attach_one(pid, device_id, address, (false, port_only),",
        ],
    );
    order(
        &code("hardware/broker/confine/streams.rs"),
        &[
            "match verdict(&iommus, in_service, |sid| translates(DeviceAddress::stream(sid))) {",
            "Verdict::Refuse(why) => {",
            "Err(Refused)",
            "for sid in iommus.sids() {",
            "let stream = DeviceAddress::stream(*sid);",
            "if attach_one(pid, device_id, stream, (true, port_only), (&[], &[])).is_err() {",
            "detach(pid, device_id);",
            "return Err(Refused);",
        ],
    );
    // `must`: no unit in service refuses it rather than leaving it unconfined.
    let capsule = code("hardware/broker/confine/capsule.rs");
    order(
        &capsule,
        &[
            "Err(e) if unconfined_allowed(e) && !must => {",
            "return Ok(None);",
            "Err(e) if unconfined_allowed(e) => {",
            "return Err(Refused);",
        ],
    );
    let unit = code("arch/iommu_unit.rs");
    assert!(unit.contains("in_service_phandle() == Some(unit);"));
    assert!(code("arch/aarch64/iommu/smmu/state.rs").contains(".filter(|p| *p != 0)"));
}

#[test]
fn every_stream_comes_out_at_release() {
    // Each requester once, so a claim racing the release keeps its own.
    order(
        &code("hardware/broker/confine/detach.rs"),
        &[
            "fn detach_held(",
            "while let Some(denied) = detach_one(&mut all[pos], device_id, &mut done) {",
            "let i = c.devices.iter().position(|(d, a)| *d == device_id && !done.contains(a))?;",
            "let (_, address) = c.devices.remove(i);",
            "done.push(address);",
        ],
    );
}
