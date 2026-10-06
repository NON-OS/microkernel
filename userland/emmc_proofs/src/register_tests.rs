// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The card's registers: OCR, the R2 reconstruction, CID, CSD, EXT_CSD and
//! the R1 status, decoded from values laid out by hand from JEDEC eMMC 5.1.

use crate::emmc::mmc::ext_csd::*;
use crate::emmc::mmc::r1::*;
use crate::emmc::mmc::regs::*;
use crate::emmc::sdhci::status::{judge, Seen};

#[test]
fn ocr_ready_and_access_mode() {
    assert!(ocr_ready(0xc0ff_8080));
    assert!(!ocr_ready(0x40ff_8080));
    assert!(ocr_sector_mode(0xc0ff_8080));
    assert!(!ocr_sector_mode(0x80ff_8080));
    assert!(!ocr_sector_mode(0xe0ff_8080));
}

/// The four response words a host holds for a 128-bit register.
fn words(reg: u128) -> [u32; 4] {
    let r = reg >> 8;
    [r as u32, (r >> 32) as u32, (r >> 64) as u32, (r >> 96) as u32]
}

#[test]
fn r2_puts_the_register_back_in_place_without_its_crc() {
    let reg: u128 = 0x1501_0044_4634_3033_3210_1234_5678_51ff;
    assert_eq!(r2(words(reg)), reg & !0xff);
}

#[test]
fn cid_fields() {
    // MID 0x15, CBX 01, OID 0x00, PNM "8GTF4R", PRV 0x02, PSN 0xdeadbeef,
    // MDT 0x7a, CRC and stop bit.
    let mut reg: u128 = 0x15u128 << 120 | 1u128 << 112;
    for (i, &b) in b"8GTF4R".iter().enumerate() {
        reg |= (b as u128) << (96 - 8 * i as u32);
    }
    reg |= 0x02u128 << 48 | 0xdead_beefu128 << 16 | 0x7au128 << 8 | 0x55u128;
    let cid = Cid::parse(r2(words(reg)));
    assert_eq!(cid.mid, 0x15);
    assert_eq!(cid.cbx, 1);
    assert_eq!(cid.oid, 0);
    assert_eq!(&cid.pnm, b"8GTF4R");
    assert_eq!(cid.prv, 0x02);
    assert_eq!(cid.psn, 0xdead_beef);
    assert_eq!(cid.mdt, 0x7a);
}

#[test]
fn csd_fields_and_byte_mode_capacity() {
    let reg: u128 =
        3u128 << 126 | 4u128 << 122 | 0x32u128 << 96 | 9u128 << 80 | 0x7ffu128 << 62 | 7u128 << 47;
    let csd = Csd::parse(reg);
    assert_eq!((csd.structure, csd.spec_vers, csd.tran_speed), (3, 4, 0x32));
    assert_eq!((csd.read_bl_len, csd.c_size, csd.c_size_mult), (9, 0x7ff, 7));
    // 2048 x 2^9 blocks of 512 bytes = 512 MiB.
    assert_eq!(csd.sectors(), Some(1_048_576));
    // 1 GiB with 1024-byte blocks.
    let c = Csd { read_bl_len: 10, ..csd };
    assert_eq!(c.sectors(), Some(2_097_152));
    // C_SIZE all ones defers to EXT_CSD.
    assert_eq!(Csd { c_size: C_SIZE_EXT, ..csd }.sectors(), None);
    assert_eq!(Csd { read_bl_len: 8, ..csd }.sectors(), None);
    assert_eq!(Csd { read_bl_len: 12, ..csd }.sectors(), None);
}

fn ext() -> ExtCsd {
    let mut raw = [0u8; 512];
    raw[192] = 8;
    raw[196] = 0x57;
    raw[179] = 0x49;
    raw[212..216].copy_from_slice(&0x03a3_e000u32.to_le_bytes());
    raw[226] = 32;
    raw[168] = 32;
    raw[248] = 25;
    raw[199] = 0;
    raw[249..253].copy_from_slice(&512u32.to_le_bytes());
    raw[33] = 1;
    ExtCsd::new(raw)
}

#[test]
fn ext_csd_fields() {
    let e = ext();
    assert_eq!(e.rev(), 8);
    assert_eq!(e.sec_count(), 0x03a3_e000);
    assert_eq!(e.partition_config(), 0x49);
    assert_eq!(e.partition_access(), 1);
    assert_eq!(e.boot_sectors(), 8192);
    assert_eq!(e.rpmb_sectors(), 8192);
    assert_eq!(e.cache_kib(), 512);
    assert!(e.cache_on());
    assert_eq!(e.hs_hz(), 52_000_000);
    assert_eq!(e.switch_ms(HS_TIMING), 500);
    assert_eq!(e.switch_ms(PARTITION_CONFIG), 500);
    let mut raw = e.raw;
    raw[248] = 100;
    raw[199] = 255;
    let e2 = ExtCsd::new(raw);
    assert_eq!(e2.switch_ms(BUS_WIDTH), 1000);
    // The longest a byte can name stays inside the cap.
    assert_eq!(e2.switch_ms(PARTITION_CONFIG), 2550);
    raw[196] = 0x01;
    assert_eq!(ExtCsd::new(raw).hs_hz(), 26_000_000);
    raw[196] = 0x00;
    assert_eq!(ExtCsd::new(raw).hs_hz(), 0);
    raw[33] = 0;
    assert!(!ExtCsd::new(raw).cache_on());
    assert_eq!(bus_width_value(8), 2);
    assert_eq!(bus_width_value(4), 1);
    assert_eq!(bus_width_value(1), 0);
}

#[test]
fn the_bus_test_compares_only_read_only_fields() {
    let a = ext();
    let mut raw = a.raw;
    // Writable fields may differ.
    raw[BUS_WIDTH as usize] = 2;
    raw[HS_TIMING as usize] = 1;
    raw[PARTITION_CONFIG as usize] = 0;
    assert!(a.bus_test_same(&ExtCsd::new(raw)));
    for &i in BUS_TEST_BYTES.iter() {
        let mut r = a.raw;
        r[i] ^= 0x10;
        assert!(!a.bus_test_same(&ExtCsd::new(r)), "byte {i}");
    }
}

#[test]
fn r1_state_errors_and_readiness() {
    let tran_ready = 4 << 9 | READY_FOR_DATA;
    assert_eq!(state(tran_ready), STATE_TRAN);
    assert!(ready(tran_ready));
    assert!(!ready(4 << 9));
    assert!(!ready(7 << 9 | READY_FOR_DATA));
    assert_eq!(errors(tran_ready), 0);
    for bit in [31, 30, 29, 26, 21, 20, 19] {
        assert_ne!(errors(tran_ready | 1 << bit), 0, "bit {bit}");
    }
    // About the previous command, and status-only bits: not errors here.
    for bit in [23, 22, 25, 8, 7, 5] {
        assert_eq!(errors(tran_ready | 1 << bit), 0, "bit {bit}");
    }
}

#[test]
fn switch_verdicts() {
    assert_eq!(switch_verdict(4 << 9 | READY_FOR_DATA), SwitchVerdict::Done);
    assert_eq!(switch_verdict(7 << 9), SwitchVerdict::Busy);
    assert_eq!(switch_verdict(4 << 9 | SWITCH_ERROR), SwitchVerdict::Refused);
    assert_eq!(switch_verdict(7 << 9 | SWITCH_ERROR), SwitchVerdict::Refused);
    assert_eq!(switch_verdict(4 << 9 | ADDRESS_ERROR), SwitchVerdict::Failed);
    assert_eq!(switch_verdict(5 << 9), SwitchVerdict::Failed);
}

#[test]
fn interrupt_status_judgement() {
    assert_eq!(judge(0, 1), Seen::Pending);
    assert_eq!(judge(1, 1), Seen::Done);
    assert_eq!(judge(2, 1), Seen::Pending);
    assert_eq!(judge(3, 2), Seen::Done);
    // An error wins over completion.
    assert_eq!(judge(1 << 15 | 1 << 16 | 1, 1), Seen::Failed(1));
    assert_eq!(judge(1 << 21 | 2, 2), Seen::Failed(1 << 5));
    assert_eq!(judge(1 << 15, 1), Seen::Failed(0));
}

#[test]
fn a_failed_status_names_its_error_and_state() {
    use crate::emmc::mmc::r1::{first_error, state_name, ADDRESS_ERROR, SWITCH_ERROR};
    let tran = 4 << 9;
    assert_eq!(first_error(tran), None);
    assert_eq!(first_error(tran | 1 << 8), None, "READY_FOR_DATA is no error");
    assert_eq!(first_error(tran | SWITCH_ERROR), Some("SWITCH_ERROR"));
    assert_eq!(first_error(ADDRESS_ERROR | SWITCH_ERROR), Some("ADDRESS_ERROR"));
    assert_eq!(state_name(tran), "tran");
    assert_eq!(state_name(7 << 9), "prg");
    assert_eq!(state_name(15 << 9), "reserved");
    for bit in 0..32 {
        if let Some(name) = first_error(1 << bit) {
            assert!(name.bytes().all(|c| c.is_ascii_uppercase() || c == b'_'));
        }
    }
}
