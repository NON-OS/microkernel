// NONOS Operating System (AGPL-3.0-or-later)
//! Commands, STEs, the CD and page table entries, field by field.

use crate::smmu::encode::{cd, cmd, pte, ste};

#[test]
fn commands_carry_opcode_and_operands_where_the_spec_puts_them() {
    assert_eq!(cmd::cfgi_ste(0x1234), [0x03 | 0x1234 << 32, 1]);
    assert_eq!(cmd::cfgi_cd(7), [0x05 | 7 << 32, 1]);
    assert_eq!(cmd::cfgi_all(), [0x04, 31]);
    assert_eq!(cmd::tlbi_nh_asid(0xbeef), [0x11 | 0xbeef << 48, 0]);
    assert_eq!(cmd::tlbi_nsnh_all(), [0x30, 0]);
    assert_eq!(cmd::sync(), [0x46, 0]);
    assert_eq!(cmd::cons_error(0x0b << 24 | 0x1f), 0x0b);
}

#[test]
fn stream_table_entries() {
    assert_eq!(ste::abort(), [1, 0, 0, 0, 0, 0, 0, 0]);
    let s = ste::stage1(0x8000_1040 | 0x3f);
    assert_eq!(s[0], 1 | 0b101 << 1 | 0x8000_1040, "pointer bits 51:6 only");
    assert_eq!(s[1], 0b01 << 2 | 0b01 << 4 | 0b11 << 6);
    assert_eq!(&s[2..], &[0; 6]);
    assert_eq!(ste::l1_descriptor(0x4_0000_1000, 6), 7 | 0x4_0000_1000);
    assert_eq!(ste::l1_descriptor(0, 6) & 0x1f, 7, "span is split + 1");
}

#[test]
fn context_descriptor() {
    let c = cd::cd(0x8765_4000 | 0xf, 0x42, 4);
    let w0 = c[0];
    assert_eq!(w0 & 0x3f, 25, "T0SZ");
    assert_eq!((w0 >> 6) & 0b11, 0, "TG0 4 KiB");
    assert_eq!((w0 >> 8) & 0x3f, 0b11_01_01, "IR0, OR0, SH0");
    assert_eq!((w0 >> 30) & 0b11, 0b11, "EPD1 and V");
    assert_eq!((w0 >> 32) & 0b111, 4, "IPS");
    assert_eq!(w0 >> 35 & 1, 1, "AFFD");
    assert_eq!((w0 >> 41) & 1, 1, "AA64");
    assert_eq!((w0 >> 45) & 0b111, 0b011, "R and A set, ASET clear");
    assert_eq!(w0 >> 48, 0x42, "ASID");
    assert_eq!(c[1], 0x8765_4000, "TTB0 bits 51:4");
    assert_eq!(c[3], 0x04ff, "MAIR attribute 0 normal write-back, 1 device");
}

#[test]
fn page_table_entries() {
    let rw = pte::page(0x1234_5000, true, true, false).unwrap();
    let ro = pte::page(0x1234_5000, true, false, false).unwrap();
    assert_eq!(rw & 0b11, 0b11);
    assert_eq!((rw >> 6) & 0b11, 0b01, "unprivileged read-write");
    assert_eq!((ro >> 6) & 0b11, 0b11, "unprivileged read-only");
    assert_eq!(pte::page(0x1234_5000, false, true, false), Some(rw), "write-only maps read-write");
    assert_eq!(pte::page(0x1234_5000, false, false, false), None);
    assert_eq!((rw >> 8) & 0b111, 0b111, "inner shareable, accessed");
    assert_eq!(rw >> 53, 0b11, "never executable");
    assert_eq!(pte::address(rw), 0x1234_5000);
    assert_eq!(pte::table(0x9000 | 0xfff), 0x9003);
    assert!(pte::is_valid(rw) && !pte::is_valid(0));
    let bell = pte::page(0x0808_0000 | 0x10000, false, true, true).unwrap();
    assert_eq!((bell >> 2) & 0b111, 1, "MAIR attribute 1, device");
    assert_eq!((bell >> 8) & 0b11, 0b10, "outer shareable");
    assert_eq!(bell, 0b11 | 0x0809_0000 | 1 << 6 | 1 << 2 | 0b10 << 8 | 1 << 10 | 0b11 << 53);
}

#[test]
fn walk_indices_and_the_input_range() {
    let iova = 0x7f_c060_3000u64;
    let idx: Vec<usize> = (1..=3).map(|l| pte::level_index(iova, l)).collect();
    assert_eq!(idx, [0x1ff, 0x003, 0x003]);
    assert_eq!(pte::level_index(0x1000, 3), 1);
    assert!(pte::in_range(0x10_0000, 0x1000));
    assert!(pte::in_range((1 << 39) - 0x1000, 0x1000));
    assert!(!pte::in_range(1 << 39, 0x1000));
    assert!(!pte::in_range(u64::MAX - 0xfff, 0x2000));
}
