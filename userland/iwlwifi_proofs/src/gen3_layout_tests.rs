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

//! The gen3 memory plan, both queues, and every command payload's byte image.
//!
//! The structure sizes and field offsets pinned here were compiled from the
//! Linux v6.12 iwlwifi headers (fw/api/*.h, iwl-context-info*.h, iwl-fh.h,
//! pcie/internal.h) with gcc and printed with `sizeof`/`offsetof`, so a
//! transcription slip in a builder cannot pass: iwl_tfh_tfd 256,
//! iwl_rx_transfer_desc 16, iwl_rx_completion_desc 32 (rbid at 4, flags at
//! 6), iwl_cmd_header_wide 8, iwl_mac_config_cmd 52 (action 4, mac_type 8,
//! local_mld_addr 12, filter_flags 20, nic_not_ack_enabled 32),
//! iwl_link_config_cmd 208 (link_id 4, mac_id 8, phy_id 12, local_link_addr
//! 16), iwl_scan_config 12 (tx_chains 4, rx_chains 8), iwl_mcc_update_cmd 28,
//! iwl_scan_req_umac_v17 1940 (general params at 8, channel params at 44,
//! periodic at 584, probe at 596; in the general params: link id 3, active
//! dwell 4, adaptive dwell defaults 6..8, max budget 10, priority 28, passive
//! dwell 32), iwl_scan_channel_cfg_umac 8.

use crate::gen3::cmdq::{CmdQueue, FIRST_TB};
use crate::gen3::cmds::*;
use crate::gen3::dram_map::classify;
use crate::gen3::layout::{MCR_SIZE, MTR_SIZE};
use crate::gen3::plan::*;
use crate::gen3::prph::write_umac;
use crate::gen3::region::Region;
use crate::gen3::regs::*;
use crate::gen3::rxq::{RxQueue, Taken};
use crate::gen3::scan::{complete, request, SCAN_REQ_V17_LEN};
use crate::gen3_model::{Mem, Model, Polls};
use crate::regs::Mmio;

static SO_GF: &[u8] = include_bytes!("../../../nonos-bootloader/firmware/intel/iwlwifi-so-a0-gf-a0-86.ucode");

#[test]
fn the_control_region_holds_everything_without_overlap() {
    let spans = [
        (CTXT_INFO, 104),
        (PRPH_SCRATCH, 1724),
        (PRPH_INFO, PAGE),
        (RB_STTS, 2),
        (RX_FREE, RX_RING * 16),
        (RX_USED, RX_RING * 32),
        (CMD_TFDS, CMD_RING * 256),
        (FIRST_TBS, CMD_RING * 64),
        (CMD_BUFFER, CMD_BUF),
        (IML, 13944),
    ];
    for w in spans.windows(2) {
        assert!(w[0].0 + w[0].1 <= w[1].0, "{:#x} runs into {:#x}", w[0].0, w[1].0);
    }
    for (off, _) in &spans[..8] {
        assert_eq!(off % PAGE, 0, "{off:#x} is page aligned");
    }
    const { assert!(CMD_BUF >= 8 + SCAN_REQ_V17_LEN, "the scan request fits the command buffer") };
    assert_eq!(control_len(13944), Some(0x19000));
    assert!(control_len(GRANT_MAX).is_none(), "an image loader too large for one grant");
    assert_eq!(RB_REGION, 64 * 2048);
    const { assert!(RB_REGION <= GRANT_MAX) };
    assert_eq!((MTR_SIZE, MCR_SIZE), (4, 9), "TFD_QUEUE_CB_SIZE(128), RX_QUEUE_CB_SIZE(512)");
}

#[test]
fn the_firmware_is_packed_into_broker_sized_page_aligned_blocks() {
    let layout = classify(SO_GF);
    let lens: Vec<usize> =
        layout.lmac.iter().chain(layout.umac.iter()).chain(layout.virt.iter()).map(|s| s.data.len()).collect();
    assert_eq!((layout.lmac.len(), layout.umac.len(), layout.virt.len()), (15, 16, 24));
    let plan = plan_firmware(&lens).expect("plans");
    assert!(plan.regions.iter().all(|&r| r <= GRANT_MAX && r % PAGE == 0));
    assert_eq!(plan.blocks.len(), lens.len());
    for (b, &len) in plan.blocks.iter().zip(&lens) {
        assert_eq!(b.len, len);
        assert_eq!(b.off % PAGE, 0, "each section on its own page");
        assert!(b.off + b.len <= plan.regions[b.region], "inside its region");
    }
    for w in plan.blocks.windows(2) {
        if w[0].region == w[1].region {
            assert!(w[0].off + w[0].len <= w[1].off, "no overlap");
        }
    }
    let total: usize = lens.iter().sum();
    assert!(plan.regions.iter().sum::<usize>() >= total);
    assert!(plan.regions.len() <= 8, "1.6 MB in at most eight 256 KiB grants");
    assert!(plan_firmware(&[0]).is_none(), "an empty section");
    assert!(plan_firmware(&[GRANT_MAX + 1]).is_none(), "a section larger than a grant");
}

fn regions() -> (std::rc::Rc<Mem>, std::rc::Rc<Mem>) {
    (Mem::new(0x19000, 0x1000_0000), Mem::new(RB_REGION, 0x2000_0000))
}

#[test]
fn the_receive_queue_posts_every_buffer_and_tells_the_device_in_eights() {
    let (ctrl, rbs) = regions();
    let model = Model::new(ctrl.clone(), rbs.clone());
    let mut q = RxQueue::new();
    assert!(q.init(ctrl.as_ref(), rbs.as_ref()));
    let mut d = [0u8; 16];
    ctrl.read(RX_FREE + 5 * 16, &mut d);
    assert_eq!(u16::from_le_bytes([d[0], d[1]]), 6, "buffer id, 1-based");
    assert_eq!(u64::from_le_bytes(d[8..16].try_into().unwrap()), rbs.dev() + 5 * 2048);
    q.kick(&model);
    assert_eq!(model.s.borrow().widx, 64);
    model.s.borrow_mut().widx = 0;
    q.kick(&model);
    assert_eq!(model.s.borrow().widx, 0, "an unchanged index is not written again");
}

// Put a completion for buffer `vid` at used index `i` and close up to `closed`.
fn complete_at(ctrl: &Mem, i: usize, vid: u16, flags: u8, closed: u16) {
    let mut cd = [0u8; 32];
    cd[4..6].copy_from_slice(&vid.to_le_bytes());
    cd[6] = flags;
    ctrl.write(RX_USED + i * 32, &cd);
    ctrl.write(RB_STTS, &closed.to_le_bytes());
}

#[test]
fn taking_a_buffer_checks_the_device_named_one_we_posted() {
    let (ctrl, rbs) = regions();
    let mut q = RxQueue::new();
    q.init(ctrl.as_ref(), rbs.as_ref());
    rbs.write(2 * 2048, b"payload");
    let mut out = [0u8; 2048];
    assert_eq!(q.take(ctrl.as_ref(), rbs.as_ref(), &mut out), None, "nothing closed yet");
    complete_at(&ctrl, 0, 3, 0, 1);
    assert_eq!(q.take(ctrl.as_ref(), rbs.as_ref(), &mut out), Some(Taken::Buffer));
    assert_eq!(&out[..7], b"payload");
    let mut d = [0u8; 16];
    ctrl.read(RX_FREE + 64 * 16, &mut d);
    assert_eq!(u16::from_le_bytes([d[0], d[1]]), 3, "the buffer went straight back on the ring");
    complete_at(&ctrl, 1, 3, 0, 2);
    complete_at(&ctrl, 2, 0, 0, 3);
    complete_at(&ctrl, 3, 65, 0, 4);
    assert_eq!(q.take(ctrl.as_ref(), rbs.as_ref(), &mut out), Some(Taken::Buffer), "re-posted id is fine");
    assert_eq!(q.take(ctrl.as_ref(), rbs.as_ref(), &mut out), Some(Taken::BadId), "id zero");
    assert_eq!(q.take(ctrl.as_ref(), rbs.as_ref(), &mut out), Some(Taken::BadId), "id past the pool");
    complete_at(&ctrl, 4, 9, 1, 5);
    complete_at(&ctrl, 5, 10, 0, 6);
    assert_eq!(q.take(ctrl.as_ref(), rbs.as_ref(), &mut out), Some(Taken::Fragment));
    assert_eq!(q.take(ctrl.as_ref(), rbs.as_ref(), &mut out), Some(Taken::Fragment), "the rest of that frame");
    // A closed index wider than the ring is masked to it.
    complete_at(&ctrl, 6, 11, 0, 0x1000 | 7);
    assert_eq!(q.take(ctrl.as_ref(), rbs.as_ref(), &mut out), Some(Taken::Buffer));
}

#[test]
fn a_command_goes_out_as_the_header_and_two_buffers_and_rings_the_doorbell() {
    let (ctrl, rbs) = regions();
    let model = Model::new(ctrl.clone(), rbs.clone());
    let mut q = CmdQueue::new();
    assert!(q.init(ctrl.as_ref()));
    let seq = q.send(&model, ctrl.as_ref(), 0x0C, 0x02, 0, &[0; 4]).unwrap();
    assert_eq!(seq, 0);
    assert_eq!(model.s.borrow().commands[0].payload, vec![0; 4], "decoded from the ring by the device");
    let mut t = [0u8; 256];
    ctrl.read(CMD_TFDS, &mut t);
    assert_eq!(u16::from_le_bytes([t[0], t[1]]), 1, "twelve bytes: one buffer");
    assert_eq!(u16::from_le_bytes([t[2], t[3]]), 12);
    let big = vec![0xA5; SCAN_REQ_V17_LEN];
    let seq = q.send(&model, ctrl.as_ref(), 1, 0x0D, 0, &big).unwrap();
    assert_eq!(seq, 1);
    ctrl.read(CMD_TFDS + 256, &mut t);
    assert_eq!(u16::from_le_bytes([t[0], t[1]]), 2);
    assert_eq!(u16::from_le_bytes([t[2], t[3]]) as usize, FIRST_TB);
    assert_eq!(u16::from_le_bytes([t[12], t[13]]) as usize, 8 + SCAN_REQ_V17_LEN - FIRST_TB);
    assert_eq!(model.s.borrow().commands[1].payload, big, "the device reassembles it whole");
    assert_eq!(model.reg(HBUS_TARG_WRPTR), 2, "queue 0, write pointer 2");
    assert!(q.send(&model, ctrl.as_ref(), 1, 0x0D, 0, &vec![0; CMD_BUF]).is_none(), "too large");
}

#[test]
fn the_command_ring_slot_wraps_at_128_and_the_write_pointer_at_16_bits() {
    let (ctrl, rbs) = regions();
    let model = Model::new(ctrl.clone(), rbs.clone());
    let mut q = CmdQueue::new();
    q.init(ctrl.as_ref());
    for i in 0..130u16 {
        let seq = q.send(&model, ctrl.as_ref(), 2, 3, 0, &[1, 2, 3, 4]).unwrap();
        assert_eq!(seq, i & 0xFF);
    }
    assert_eq!(model.reg(HBUS_TARG_WRPTR), 130, "the pointer does not wrap at the ring");
    assert_eq!(model.s.borrow().commands.len(), 130, "slots 0 and 1 reused cleanly");
}

#[test]
fn a_umac_register_write_uses_the_24_bit_window_and_releases_the_mac() {
    let (ctrl, rbs) = regions();
    let model = Model::new(ctrl, rbs);
    let mut c = Polls { per_wait: 4, delays_us: 0 };
    assert!(write_umac(&model, &mut c, UREG_CPU_INIT_RUN, 1));
    assert_eq!(model.reg(HBUS_TARG_PRPH_WADDR), 0x03D0_5C44, "address and byte enables");
    assert_eq!(model.read32(CSR_GP_CNTRL) & GP_MAC_ACCESS_REQ, 0, "access released");
}

#[test]
fn every_command_payload_has_its_linux_shape() {
    assert_eq!(init_extended_cfg(), [2, 0, 0, 0]);
    assert_eq!(reserved_word(), [0; 4]);
    assert_eq!(tx_ant_cfg(3), [3, 0, 0, 0]);
    assert_eq!(bt_config(true), vec![1, 0, 0, 0, 0x15, 0, 0, 0], "NW, MPLUT, SYNC2SCO, high-band");
    assert_eq!(bt_config(false)[4], 0x14);
    let soc = soc_configuration(true, 2, true, 12000);
    assert_eq!(u32::from_le_bytes(soc[0..4].try_into().unwrap()), (2 << 2) | 2, "LTR 2500 us, low latency");
    assert_eq!(u32::from_le_bytes(soc[4..8].try_into().unwrap()), 12000);
    assert_eq!(soc_configuration(false, 0, false, 0)[0], 1, "discrete");
    assert_eq!(device_power(), [0; 4], "CAM");
    let mcc = mcc_update_world();
    assert_eq!((mcc.len(), u16::from_le_bytes([mcc[0], mcc[1]]), mcc[2]), (28, 0x5A5A, 0), "ZZ, from the firmware");
    let sc = scan_config(3, 1);
    assert_eq!((sc.len(), sc[2], sc[4], sc[8]), (12, 0, 3, 1));
    let mac = mac_config_add([2, 0, 0, 0, 0, 1]);
    assert_eq!(u32::from_le_bytes(mac[4..8].try_into().unwrap()), 1, "add");
    assert_eq!(u32::from_le_bytes(mac[8..12].try_into().unwrap()), 5, "BSS station");
    assert_eq!(&mac[12..18], &[2, 0, 0, 0, 0, 1]);
    assert_eq!(u32::from_le_bytes(mac[20..24].try_into().unwrap()), 0b1100, "group frames and beacons");
    assert_eq!(u32::from_le_bytes(mac[32..36].try_into().unwrap()), 1, "no ACK-enabled aggregation");
    assert!(mac[36..].iter().all(|&b| b == 0), "unassociated");
    let link = link_config_add([2, 0, 0, 0, 0, 1]);
    assert_eq!(link.len(), 208);
    assert_eq!(u32::from_le_bytes(link[0..4].try_into().unwrap()), 1);
    assert_eq!(u32::from_le_bytes(link[12..16].try_into().unwrap()), 0xFFFF_FFFF, "no PHY context");
    assert_eq!(&link[16..22], &[2, 0, 0, 0, 0, 1]);
}

#[test]
fn the_mcc_reply_is_read_by_its_version_and_its_length_checked() {
    let mut v4 = vec![0u8; 16 + 4 + 8];
    v4[4..6].copy_from_slice(&0x4445u16.to_le_bytes());
    v4[16..20].copy_from_slice(&2u32.to_le_bytes());
    assert_eq!(mcc_update_reply(&v4, 6, true), Some((0, 0x4445)));
    assert_eq!(mcc_update_reply(&v4[..27], 6, true), None, "two channels promised, one and a half given");
    let mut v3 = vec![0u8; 16];
    v3[0] = 1;
    assert_eq!(mcc_update_reply(&v3, 5, false), Some((1, 0)));
    let v8 = vec![0u8; 24];
    assert!(mcc_update_reply(&v8, 8, true).is_some());
    let mut huge = vec![0u8; 20];
    huge[16..20].copy_from_slice(&u32::MAX.to_le_bytes());
    assert_eq!(mcc_update_reply(&huge, 6, true), None, "a channel count past the reply");
}

#[test]
fn the_scan_request_is_a_passive_one_iteration_scan_of_the_given_channels() {
    let r = request(&[1, 6, 36, 165]).expect("builds");
    assert_eq!(r.len(), 1940);
    let u16at = |o: usize| u16::from_le_bytes([r[o], r[o + 1]]);
    let u32at = |o: usize| u32::from_le_bytes(r[o..o + 4].try_into().unwrap());
    assert_eq!((u32at(0), u32at(4)), (0, 6), "uid 0, out-of-channel priority 6");
    assert_eq!(u16at(8), (1 << 11) | (1 << 7) | (1 << 1), "passive, adaptive dwell, pass all");
    assert_eq!(r[11], 0, "link 0");
    assert_eq!((r[12], r[13], r[14], r[15], r[16]), (10, 10, 2, 8, 10));
    assert_eq!(u16at(18), 300, "full-scan budget");
    assert_eq!((u32at(20), u32at(28)), (0, 0), "unassociated: no time limits");
    assert_eq!(u32at(36), 6);
    assert_eq!((r[40], r[41]), (110, 110), "passive dwell");
    assert_eq!((r[44], r[45], r[46], r[47]), (0x20, 4, 10, 2), "channel order, count, n_aps overrides");
    let ch = |i: usize| (u32at(48 + i * 8), r[48 + i * 8 + 4], r[48 + i * 8 + 6]);
    assert_eq!(ch(0), (1 << 30, 1, 1), "2.4 GHz band, one iteration");
    assert_eq!(ch(1), (1 << 30, 6, 1));
    assert_eq!(ch(2), (0, 36, 1), "5 GHz band is 0");
    assert_eq!(ch(3), (0, 165, 1));
    assert_eq!((u16at(584), r[586]), (0, 1), "one plan, one iteration");
    assert!(r[596..].iter().all(|&b| b == 0), "no probe request, no SSIDs");
    assert!(request(&[]).is_none());
    assert!(request(&[1; 68]).is_none(), "more channels than the request holds");
    let mut done = vec![0u8; 16];
    done[6] = 1;
    assert_eq!(complete(&done), Some((0, 1)));
    assert_eq!(complete(&done[..15]), None);
}
