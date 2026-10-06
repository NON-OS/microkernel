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

//! Every parser on the gen3 path, against the bundled firmware and against
//! malformed input: the firmware description, firmware selection, received
//! packets, ALIVE, the NVM reply, received management frames. The bundled
//! images are iwlwifi-so-a0-gf-a0-86.ucode (sha256 cea0aaeb034aadb36cde1d7b6f2a
//! 0f38cb78209cbe81a40d973318f820ebdd0c) and iwlwifi-so-a0-hr-b0-84.ucode
//! (sha256 20ce23a3c723efddbd909a239b99f9c9451c1f4fbcbdb1568eca9057853aa110);
//! the values pinned for them were read with an independent Python TLV dump.

use crate::gen3::alive::{parse as parse_alive, STATUS_OK};
use crate::gen3::nvm::{parse as parse_nvm, NVM_CHANNELS};
use crate::gen3::packet::parse as parse_packet;
use crate::gen3::rx_frame::{parse as parse_frame, DESC_LEN};
use crate::gen3::select::{mac_type, rf_type, select, transport, Image, Refusal};
use crate::gen3::ucode::Ucode;

static SO_GF: &[u8] =
    include_bytes!("../../../nonos-bootloader/firmware/intel/iwlwifi-so-a0-gf-a0-86.ucode");
static SO_HR: &[u8] =
    include_bytes!("../../../nonos-bootloader/firmware/intel/iwlwifi-so-a0-hr-b0-84.ucode");

#[test]
fn the_bundled_ax211_image_describes_itself() {
    let u = Ucode::parse(SO_GF).expect("parses");
    assert_eq!(u.iml.len(), 13944, "the image loader");
    assert_eq!(u.phy_config, 0x0033_0018);
    assert_eq!((u.valid_tx_ant(), u.valid_rx_ant()), (3, 3), "antennas A and B");
    assert_eq!(u.n_scan_channels, 67);
    assert!(u.capa(110), "MLD API");
    assert!(u.capa(1) && u.capa(2), "LAR and UMAC scan");
    assert!(!u.capa(12), "no DQA command");
    assert!(u.api(48), "NVM reply version 4");
    assert_eq!(u.cmd_version(1, 0x0D), Some(17), "SCAN_REQ_UMAC version 17");
    assert_eq!(u.cmd_version(0, 0x0D), Some(17), "legacy ids are looked up in LONG_GROUP");
    assert_eq!(u.notif_version(0, 0x01), Some(6), "ALIVE notification version 6");
    assert_eq!(u.notif_version(1, 0xC8), Some(6), "MCC reply version 6");
    assert_eq!(u.cmd_version(1, 0x28), None, "MAC_CONTEXT_CMD is not listed");
    assert_eq!(u.cmd_version(0, 0x01), None, "a version 99 entry reads as not reported");
}

#[test]
fn the_bundled_ax201_image_parses_too() {
    let u = Ucode::parse(SO_HR).expect("parses");
    assert!(!u.iml.is_empty());
    assert_eq!(u.notif_version(1, 0xC8), Some(5));
    assert_eq!(u.cmd_version(1, 0x0D), Some(17));
}

// A minimal well-formed file: header, then the given records.
fn file(records: &[(u32, &[u8])]) -> Vec<u8> {
    let mut f = vec![0u8; 88];
    f[4..8].copy_from_slice(&0x0A4C_5749u32.to_le_bytes());
    for (ty, body) in records {
        f.extend_from_slice(&ty.to_le_bytes());
        f.extend_from_slice(&(body.len() as u32).to_le_bytes());
        f.extend_from_slice(body);
        while !f.len().is_multiple_of(4) {
            f.push(0);
        }
    }
    f
}

#[test]
fn malformed_firmware_files_are_refused_whole() {
    let iml: &[u8] = &[1, 2, 3, 4];
    assert!(Ucode::parse(&file(&[(52, iml)])).is_some(), "the minimal file parses");
    assert!(Ucode::parse(&file(&[])).is_none(), "no image loader");
    let mut bad_magic = file(&[(52, iml)]);
    bad_magic[4] ^= 1;
    assert!(Ucode::parse(&bad_magic).is_none(), "wrong magic");
    assert!(Ucode::parse(&bad_magic[..40]).is_none(), "short header");
    let long_phy: &[u8] = &[0; 8];
    assert!(
        Ucode::parse(&file(&[(52, iml), (23, long_phy)])).is_none(),
        "PHY_SKU of the wrong size"
    );
    let mut past_end = file(&[(52, iml)]);
    past_end.extend_from_slice(&23u32.to_le_bytes());
    past_end.extend_from_slice(&64u32.to_le_bytes());
    assert!(Ucode::parse(&past_end).is_none(), "a record running past the end");
    let real = SO_GF;
    for cut in [88, 100, 4096, real.len() / 2, real.len() - 3] {
        let _ = Ucode::parse(&real[..cut]);
    }
}

#[test]
fn bitmap_words_past_the_known_range_are_ignored_and_partial_entries_dropped() {
    let iml: &[u8] = &[9; 4];
    let far: &[u8] = &[9, 0, 0, 0, 0xFF, 0xFF, 0xFF, 0xFF];
    let near: &[u8] = &[3, 0, 0, 0, 0, 0x40, 0, 0];
    let table: &[u8] = &[0x0D, 1, 17, 0, 0x0C, 1];
    let blob = file(&[(52, iml), (30, far), (30, near), (48, table)]);
    let u = Ucode::parse(&blob).expect("parses");
    assert!(u.capa(110), "word 3 bit 14");
    assert!(!u.capa(300), "nothing past 128 bits");
    assert_eq!(u.cmd_version(1, 0x0D), Some(17));
    assert_eq!(u.cmd_version(1, 0x0C), None, "the half entry is not read");
}

#[test]
fn firmware_is_chosen_by_mac_and_rf_type() {
    let so = 0x0000_0370;
    let sof = 0x0000_0430;
    let gf = 0x0010_D000;
    let hr1 = 0x0010_C000;
    let hr2 = 0x0010_A000;
    assert_eq!(mac_type(so), 0x37);
    // CSR_HW_REV_TYPE masks 0xFFF0: bits above it (for example the step
    // field some parts report in bits 16 to 19) do not change the type.
    assert_eq!(mac_type(so | 0x000F_0000), 0x37);
    assert_eq!(select(0x51F0, so | 0x0003_0000, gf).map(|r| r.0), Ok(Image::SoGf));
    assert_eq!(rf_type(gf), 0x10D);
    assert_eq!(rf_type(gf | 0xF000_0FFF), 0x10D);
    assert_eq!(select(0x51F0, so, gf).map(|r| r.0), Ok(Image::SoGf));
    assert_eq!(select(0x7AF0, sof, hr1).map(|r| r.0), Ok(Image::SoHr));
    assert_eq!(select(0x51F1, so, hr2).map(|r| r.0), Ok(Image::SoHr));
    assert_eq!(
        select(0x51F0, so, gf | 0x1000_0000),
        Err(Refusal::CdbNotBundled),
        "gf4 is not bundled"
    );
    assert_eq!(select(0x51F0, so, 0x0010_5000), Err(Refusal::RfNotBundled(0x105)), "JF");
    assert_eq!(select(0x51F0, so, 0), Err(Refusal::RfNotBundled(0)), "blank OTP");
    assert_eq!(
        select(0x2723, so, gf),
        Err(Refusal::NotSoDevice(0x2723)),
        "AX200 is not an AX210-family platform"
    );
    assert_eq!(
        select(0x51F0, 0x0000_0500, gf),
        Err(Refusal::MacNotBundled(0x50)),
        "an unknown MAC"
    );
}

/* The discrete AX210 (TY, 0x2725) and Meteor Lake (MA, 0x7E40 and 0x2729)
share the gen3 boot. Linux names their firmware ty-a0-gf-a0 and ma-b0-gf-a0
(iwl_drv_get_fw_name: TY's step is always 'a'; MA's is the low nibble of
CSR_HW_REV, 1 for B0). */
#[test]
fn the_discrete_ax210_and_meteor_lake_get_their_images() {
    let ty = 0x0000_0420;
    let ma_a0 = 0x0000_0440;
    let ma_b0 = 0x0000_0441;
    let gf = 0x0010_D000;
    let hr1 = 0x0010_C000;
    assert_eq!(select(0x2725, ty, gf).map(|r| r.0), Ok(Image::TyGf));
    assert_eq!(
        select(0x2725, ty | 0x2, gf).map(|r| r.0),
        Ok(Image::TyGf),
        "TY is 'a' whatever the step"
    );
    assert_eq!(select(0x2725, ty, gf | 0x1000_0000), Err(Refusal::CdbNotBundled));
    assert_eq!(select(0x2725, ty, hr1), Err(Refusal::RfNotBundled(0x10C)));
    assert_eq!(select(0x7E40, ma_b0, gf).map(|r| r.0), Ok(Image::MaGf));
    assert_eq!(select(0x2729, ma_b0, gf).map(|r| r.0), Ok(Image::MaGf));
    assert_eq!(
        select(0x7E40, ma_a0, gf),
        Err(Refusal::MacStepNotBundled(0)),
        "ma-a0 is not published at API 86"
    );
    assert_eq!(select(0x7E40, ma_b0 + 1, gf), Err(Refusal::MacStepNotBundled(2)), "ma-c0");
    assert_eq!(
        select(0x7E40, ma_b0, hr1),
        Err(Refusal::RfNotBundled(0x10C)),
        "ma-b0-hr-b0 is not carried"
    );
    assert_eq!(select(0x7E40, ma_b0, gf | 0x1000_0000), Err(Refusal::CdbNotBundled));
}

#[test]
fn the_discrete_ax210_and_meteor_lake_get_linux_transport_values() {
    // cfg/ax210.c iwl_ty_mac_cfg: discrete, crystal latency 500, no LTR delay.
    let ty = transport(0x2725).unwrap();
    assert_eq!(
        (ty.integrated, ty.ltr_delay, ty.xtal_latency, ty.low_latency_xtal, ty.imr_enabled),
        (false, 0, 500, false, false)
    );
    // iwl_ma_mac_cfg: integrated, nothing else set.
    for id in [0x7E40, 0x2729] {
        let ma = transport(id).unwrap();
        assert_eq!(
            (ma.integrated, ma.ltr_delay, ma.xtal_latency, ma.low_latency_xtal, ma.imr_enabled),
            (true, 0, 0, false, false)
        );
    }
}

#[test]
fn each_so_platform_gets_its_transport_values() {
    let short = transport(0x7AF0).unwrap();
    assert_eq!(
        (short.ltr_delay, short.xtal_latency, short.low_latency_xtal, short.imr_enabled),
        (1, 500, false, false)
    );
    let long = transport(0x51F0).unwrap();
    assert_eq!(
        (long.ltr_delay, long.xtal_latency, long.low_latency_xtal, long.imr_enabled),
        (2, 12000, true, false)
    );
    assert!(transport(0x7A70).unwrap().imr_enabled && transport(0x51F1).unwrap().imr_enabled);
    assert!(transport(0x2723).is_none());
}

#[test]
fn a_received_packet_is_read_only_within_its_buffer() {
    let mut rb = vec![0u8; 64];
    rb[0..4].copy_from_slice(&8u32.to_le_bytes());
    rb[4] = 0x01;
    rb[5] = 0x00;
    rb[6..8].copy_from_slice(&0x8000u16.to_le_bytes());
    rb[8..12].copy_from_slice(&[1, 2, 3, 4]);
    let p = parse_packet(&rb).expect("parses");
    assert_eq!((p.cmd, p.group, p.sequence, p.payload), (1, 0, 0x8000, &[1u8, 2, 3, 4][..]));
    assert!(!p.is_reply(), "the firmware originated it");
    rb[0..4].copy_from_slice(&61u32.to_le_bytes());
    assert!(parse_packet(&rb).is_none(), "longer than the buffer");
    rb[0..4].copy_from_slice(&3u32.to_le_bytes());
    assert!(parse_packet(&rb).is_none(), "shorter than its header");
    rb[0..4].copy_from_slice(&0x5555_0000u32.to_le_bytes());
    assert!(parse_packet(&rb).is_none(), "the end marker");
    assert!(parse_packet(&rb[..3]).is_none());
}

#[test]
fn alive_is_read_by_version_and_refused_when_short() {
    let mut v6 = vec![0u8; 144];
    v6[0..2].copy_from_slice(&STATUS_OK.to_le_bytes());
    v6[20..24].copy_from_slice(&0x0040_1000u32.to_le_bytes()); // LMAC error table
    v6[40..44].copy_from_slice(&0x00A0_2000u32.to_le_bytes()); // SCD base
    v6[100..104].copy_from_slice(&0x4Au32.to_le_bytes());
    v6[104..108].copy_from_slice(&0x1Fu32.to_le_bytes());
    v6[108..112].copy_from_slice(&0xC040_0000u32.to_le_bytes()); // with cache bits
    v6[116..120].copy_from_slice(&7u32.to_le_bytes());
    let a = parse_alive(&v6, Some(6)).expect("v6");
    assert!(a.ok());
    assert_eq!((a.umac_major, a.umac_minor), (0x4A, 0x1F));
    assert_eq!(a.lmac_error_table, 0x0040_1000);
    assert_eq!(a.scd_base, 0x00A0_2000);
    assert_eq!(a.umac_error_table, 0x0040_0000, "FW_ADDR_CACHE_CONTROL masked off");
    assert_eq!(a.sku_id, [7, 0, 0]);
    assert!(parse_alive(&v6[..143], Some(6)).is_none(), "shorter than version 6");
    assert!(parse_alive(&v6[..128], Some(5)).is_some(), "version 5 is 128 bytes");
    let v4 = &v6[..116];
    assert_eq!(parse_alive(v4, None).map(|a| a.sku_id), Some([0; 3]), "version 4 by its size");
    let v3 = &v6[..68];
    assert!(parse_alive(v3, None).is_some(), "version 3 by its size");
    assert!(parse_alive(&v6[..100], None).is_none(), "an unknown size");
    let mut dead = v6.clone();
    dead[0..2].copy_from_slice(&0xDEADu16.to_le_bytes());
    assert!(!parse_alive(&dead, Some(6)).unwrap().ok());
}

#[test]
fn the_nvm_reply_gives_antennas_lar_and_channels() {
    let mut v4 = vec![0u8; 468];
    v4[12] = 3;
    v4[16] = 1;
    // No LAR: only channels whose profile says VALID.
    v4[28] = 1; // index 0: channel 1
    v4[28 + 14 * 4] = 1; // index 14: channel 36
    let n = parse_nvm(&v4, true).expect("v4");
    assert_eq!((n.valid_tx_ant, n.valid_rx_ant, n.lar_enabled), (3, 1, false));
    assert_eq!(n.channels, vec![1, 36]);
    v4[20] = 1;
    let lar = parse_nvm(&v4, true).unwrap();
    assert_eq!(lar.channels, NVM_CHANNELS.to_vec(), "LAR: all 2.4 and 5 GHz channels");
    assert!(parse_nvm(&v4[..467], true).is_none(), "not exactly version 4's size");
    assert!(parse_nvm(&v4, false).is_none(), "a v4-sized reply read as v3");
    let mut v3 = vec![0u8; 128];
    v3[24] = 1;
    assert_eq!(parse_nvm(&v3, false).unwrap().channels, vec![1]);
}

// An RX MPDU payload carrying `frame`, with the given flags and status.
fn mpdu(frame: &[u8], flags1: u8, flags2: u8, status: u32) -> Vec<u8> {
    let mut p = vec![0u8; DESC_LEN];
    p[0..2].copy_from_slice(&(frame.len() as u16).to_le_bytes());
    p[2] = flags1;
    p[3] = flags2;
    p[12..16].copy_from_slice(&status.to_le_bytes());
    p[40] = 50;
    p[41] = 60;
    p[42] = 11;
    p.extend_from_slice(frame);
    p
}

fn beacon_frame() -> Vec<u8> {
    let mut f = vec![0x80, 0, 0, 0];
    f.extend_from_slice(&[0xFF; 6]);
    f.extend_from_slice(&[2, 1, 1, 1, 1, 1]);
    f.extend_from_slice(&[2, 1, 1, 1, 1, 1]);
    f.extend_from_slice(&[0, 0]);
    f.extend_from_slice(&[0xAB; 12]);
    f
}

#[test]
fn received_management_frames_are_cleaned_and_checked() {
    let b = beacon_frame();
    let f = parse_frame(&mpdu(&b, 0, 0, 3)).expect("good beacon");
    assert_eq!(f.frame, b);
    assert_eq!((f.channel, f.signal), (11, -50), "the stronger chain");
    assert!(parse_frame(&mpdu(&b, 0, 0, 2)).is_none(), "bad CRC");
    assert!(parse_frame(&mpdu(&b, 0, 0, 1)).is_none(), "FIFO overrun");
    // Pad after the header, and four bytes of MIC/CRC at the end.
    let mut padded = b[..24].to_vec();
    padded.extend_from_slice(&[0xEE, 0xEE]);
    padded.extend_from_slice(&b[24..]);
    padded.extend_from_slice(&[0xCC; 4]);
    let f = parse_frame(&mpdu(&padded, 0x20, 0x20, 3)).expect("padded");
    assert_eq!(f.frame, b, "pad and tail removed");
    let mut data = b.clone();
    data[0] = 0x08;
    assert!(parse_frame(&mpdu(&data, 0, 0, 3)).is_none(), "data frames are not scan results");
    let mut lying = mpdu(&b, 0, 0, 3);
    lying[0..2].copy_from_slice(&500u16.to_le_bytes());
    assert!(parse_frame(&lying).is_none(), "length past the packet");
    assert!(parse_frame(&mpdu(&b[..20], 0, 0, 3)).is_none(), "shorter than a header");
    assert!(
        parse_frame(&mpdu(&b[..24], 0xF0, 0, 3)).is_some(),
        "a MIC length never cuts the header"
    );
    let whole = mpdu(&b, 0, 0x20, 3);
    for n in 0..whole.len() {
        let _ = parse_frame(&whole[..n]);
    }
}
