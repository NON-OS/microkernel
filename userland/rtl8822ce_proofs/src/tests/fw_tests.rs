// NONOS Operating System (AGPL-3.0-or-later)
//! The embedded image is linux-firmware's rtw88/rtw8822c_fw.bin, pinned by
//! its sha256; its header parses as an 8822C image, version 9.9, H2C format
//! 15; its three sections fill the file exactly; the RTL8821C image is
//! refused; and the version line is whole.

use crate::fw::header::{parse, FW_HDR_LEN};
use crate::fw::sections::sections;
use crate::fwload::image::version_line;

const FW: &[u8] = include_bytes!("../../../../nonos-bootloader/firmware/realtek/rtw8822c_fw.bin");
const FW_8821C: &[u8] =
    include_bytes!("../../../../nonos-bootloader/firmware/realtek/rtw8821c_fw.bin");
/// sha256 of rtw88/rtw8822c_fw.bin, as scripts/baselines/upstream.sha256 pins it.
const FW_SHA256: &str = "3deecb31210986d98cdbfb000391e08d602a6eee4ffc883969faa2b907ab03ba";

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

#[test]
fn the_image_is_the_pinned_one() {
    assert_eq!(FW.len(), 202_600);
    assert_eq!(hex(&nonos_hash::sha256(FW)), FW_SHA256);
}

#[test]
fn the_header_is_an_8822c_image() {
    let h = parse(FW).expect("a header");
    assert!(h.is_for_8822c());
    assert_eq!((h.signature, h.version(), h.h2c_fmt_ver), (0x8822, (9, 9), 15));
    assert_eq!((h.dmem_addr, h.imem_addr, h.emem_addr), (0x8020_0000, 0x8003_0000, 0x8010_0000));
}

#[test]
fn the_sections_fill_the_file() {
    let h = parse(FW).expect("a header");
    let (s, n) = sections(&h, FW.len()).expect("sections");
    assert_eq!(n, 3, "DMEM, IMEM and EMEM");
    assert_eq!(s[0].offset, FW_HDR_LEN);
    assert_eq!(s[2].offset + s[2].len, FW.len());
    assert_eq!(s.map(|x| x.dst), [0x0020_0000, 0x0003_0000, 0x0010_0000]);
}

#[test]
fn the_8821c_image_and_a_short_one_are_refused() {
    assert!(!parse(FW_8821C).expect("a header").is_for_8822c());
    assert!(parse(&FW[..63]).is_none());
    let mut no_h2c = parse(FW).expect("a header");
    no_h2c.h2c_fmt_ver = 0;
    assert!(!no_h2c.is_for_8822c());
}

#[test]
fn the_version_line_is_whole() {
    let mut out = [0u8; 48];
    let n = version_line(&parse(FW).expect("a header"), &mut out);
    assert_eq!(&out[..n], b"[rtl8822ce] firmware v9.9\n");
}
