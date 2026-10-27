// NONOS Operating System (AGPL-3.0-or-later)
//! The hardware report's line, as the boot prints it: ids and class in
//! lower-case hex, then the driver, the reason none takes the part, or
//! "unknown"; never longer than its buffer, however long the reason.

#[path = "../../../../src/hardware/inventory/report_text.rs"]
mod report_text;

use report_text::{text, Verdict, TEXT_MAX};

fn line(ids: (u16, u16), class: (u8, u8), v: Verdict) -> String {
    let mut out = [0u8; TEXT_MAX];
    let n = text(&mut out, ids, class, &v);
    String::from_utf8(out[..n].to_vec()).expect("ascii")
}

#[test]
fn each_verdict_reads_as_the_report_says() {
    assert_eq!(
        line((0x8086, 0x125C), (0x02, 0x00), Verdict::Driver("driver_igc")),
        "[HW] 8086:125c 02/00 driven by driver_igc"
    );
    assert_eq!(
        line((0x10EC, 0xB852), (0x02, 0x80), Verdict::Held("no driver for this part")),
        "[HW] 10ec:b852 02/80 known, held: no driver for this part"
    );
    assert_eq!(
        line((0x14C3, 0x0616), (0x02, 0x80), Verdict::Unknown),
        "[HW] 14c3:0616 02/80 unknown"
    );
}

#[test]
fn a_reason_longer_than_the_line_is_cut_not_overrun() {
    let why = "x".repeat(4 * TEXT_MAX);
    let got = line((0xFFFF, 0xFFFF), (0xFF, 0xFF), Verdict::Held(&why));
    assert_eq!(got.len(), TEXT_MAX);
    assert!(got.starts_with("[HW] ffff:ffff ff/ff known, held: x"));
}
