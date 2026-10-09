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

//! The data path's layouts: a frame on a TVQM transmit queue (descriptor,
//! first buffer, byte count entry, doorbell), the transmit command header,
//! the TX response, and a received frame of any kind with the firmware's
//! decryption status. Every byte the device reads or writes is checked at
//! its offset, and every length the device reports is held to its packet.

use std::cell::RefCell;

use crate::gen3::region::Region;
use crate::gen3::regs::HBUS_TARG_WRPTR;
use crate::gen3::rx_data::{parse as parse_rx, RxMpdu};
use crate::gen3::tx_cmd::{header, FRAME_MAX, IWL_TX_FLAGS_CMD_RATE, IWL_TX_FLAGS_ENCRYPT_DIS, TX_CMD_OFFLD_PAD};
use crate::gen3::tx_resp::{parse as parse_resp, TxStatus};
use crate::gen3::txq::{TxQueue, BC_TABLE, TXQ_ENTRIES, TXQ_FRAMES, TXQ_STRIDE, TX_BUF, TX_REGION};
use crate::gen3_model::Mem;
use crate::regs::Mmio;

const DEV: u64 = 0x8000_0000;

/// Registers that only record writes.
#[derive(Default)]
struct Doorbells(RefCell<Vec<(usize, u32)>>);

impl Mmio for Doorbells {
    fn read32(&self, _off: usize) -> u32 {
        0
    }
    fn write32(&self, off: usize, val: u32) {
        self.0.borrow_mut().push((off, val));
    }
}

fn mgmt_frame(body: usize) -> Vec<u8> {
    let mut f = vec![0xB0, 0x00, 0, 0];
    f.extend_from_slice(&[0xAA; 18]);
    f.extend_from_slice(&[0x10, 0x00]);
    f.extend((0..body).map(|i| i as u8));
    f
}

fn bytes(m: &Mem, off: usize, n: usize) -> Vec<u8> {
    let mut b = vec![0u8; n];
    assert!(m.read(off, &mut b));
    b
}

fn le16(b: &[u8], at: usize) -> u16 {
    u16::from_le_bytes([b[at], b[at + 1]])
}

fn le64(b: &[u8], at: usize) -> u64 {
    u64::from_le_bytes(b[at..at + 8].try_into().unwrap())
}

#[test]
fn the_transmit_region_holds_both_queues_in_one_grant() {
    assert_eq!(TXQ_STRIDE % 4096, 0);
    const { assert!(TX_REGION <= crate::gen3::plan::GRANT_MAX) };
    // Ring, byte count table, first buffers and frame buffers fit the stride.
    const { assert!(TXQ_ENTRIES * 256 + 4096 + TXQ_ENTRIES * 64 + TXQ_FRAMES * TX_BUF <= TXQ_STRIDE) };
    const { assert!(BC_TABLE <= 4096) };
}

#[test]
fn the_transmit_header_sends_at_its_own_rate_with_no_firmware_key() {
    let h = header(60, 24, 0x4000).unwrap();
    let mut want = [0u8; 28];
    want[0] = 60;
    want[2] = (IWL_TX_FLAGS_CMD_RATE | IWL_TX_FLAGS_ENCRYPT_DIS) as u8;
    want[16..20].copy_from_slice(&0x4000u32.to_le_bytes());
    assert_eq!(h, want);
    // A 26-byte QoS header is padded by the transport.
    let qos = header(60, 26, 0).unwrap();
    assert_eq!(u32::from_le_bytes(qos[4..8].try_into().unwrap()), TX_CMD_OFFLD_PAD);
    assert!(header(FRAME_MAX, 24, 0).is_some());
    assert!(header(FRAME_MAX + 1, 24, 0).is_none(), "longer than the byte count can say");
}

#[test]
fn a_frame_is_three_buffers_a_byte_count_and_a_doorbell() {
    let tx = Mem::new(TX_REGION, DEV);
    let m = Doorbells::default();
    let mut q = TxQueue::new(TXQ_STRIDE);
    assert!(q.clear(&*tx));
    q.start(9, 5);
    assert_eq!(q.tfds(&*tx), DEV + TXQ_STRIDE as u64);
    assert_eq!(q.bc(&*tx), DEV + (TXQ_STRIDE + 128 * 256) as u64);
    let frame = mgmt_frame(40);
    assert!(q.send(&m, &*tx, &frame, 0x4000));

    let base = TXQ_STRIDE;
    let tfd = bytes(&tx, base + 5 * 256, 32);
    assert_eq!(le16(&tfd, 0), 3, "first buffer, command and header, body");
    let first_at = base + 128 * 256 + 4096 + 5 * 64;
    let buf_at = base + 128 * 256 + 4096 + 128 * 64 + 5 * TX_BUF;
    assert_eq!((le16(&tfd, 2), le64(&tfd, 4)), (20, DEV + first_at as u64));
    assert_eq!((le16(&tfd, 12), le64(&tfd, 14)), (36, DEV + (buf_at + 20) as u64));
    assert_eq!((le16(&tfd, 22), le64(&tfd, 24)), (40, DEV + (buf_at + 56) as u64));

    // The device command: TX_CMD, group 0, sequence queue 9 index 5.
    let cmd = bytes(&tx, buf_at, 56);
    assert_eq!(&cmd[0..4], &[0x1C, 0, 0x05, 0x09]);
    assert_eq!(&cmd[4..32], &header(frame.len(), 24, 0x4000).unwrap());
    assert_eq!(&cmd[32..56], &frame[..24], "the 802.11 header follows the command");
    assert_eq!(bytes(&tx, first_at, 20), cmd[..20].to_vec(), "the first buffer is its first 20 bytes");
    assert_eq!(bytes(&tx, buf_at + 56, 40), frame[24..].to_vec(), "then the body");
    let bc = bytes(&tx, base + 128 * 256 + 5 * 2, 2);
    assert_eq!(le16(&bc, 0), frame.len() as u16, "one fetch chunk: bits 14-15 clear");
    assert_eq!(*m.0.borrow(), vec![(HBUS_TARG_WRPTR, 6 | 9 << 16)]);
}

#[test]
fn a_header_that_is_not_a_multiple_of_four_is_padded_in_the_second_buffer() {
    let tx = Mem::new(TX_REGION, DEV);
    let m = Doorbells::default();
    let mut q = TxQueue::new(0);
    q.start(4, 0);
    // A QoS data frame: a 26-byte header.
    let mut f = vec![0x88, 0x01];
    f.extend_from_slice(&[0u8; 22]);
    f.extend_from_slice(&[0x06, 0x00]);
    f.extend_from_slice(b"payload");
    assert!(q.send(&m, &*tx, &f, 0));
    let tfd = bytes(&tx, 0, 32);
    let buf_at = 128 * 256 + 4096 + 128 * 64;
    // 4 + 28 + 26 = 58 bytes of command and header, 20 in the first buffer,
    // 38 rounded up to 40 in the second; the body after the pad.
    assert_eq!((le16(&tfd, 12), le64(&tfd, 14)), (40, DEV + (buf_at + 20) as u64));
    assert_eq!((le16(&tfd, 22), le64(&tfd, 24)), (7, DEV + (buf_at + 60) as u64));
    let cmd = bytes(&tx, buf_at, 60);
    assert_eq!(u32::from_le_bytes(cmd[8..12].try_into().unwrap()), TX_CMD_OFFLD_PAD);
    assert_eq!(&cmd[32..58], &f[..26]);
    assert_eq!(&cmd[58..60], &[0, 0], "the pad");
}

#[test]
fn a_frame_with_no_body_takes_two_buffers() {
    let tx = Mem::new(TX_REGION, DEV);
    let m = Doorbells::default();
    let mut q = TxQueue::new(0);
    q.start(2, 0);
    assert!(q.send(&m, &*tx, &mgmt_frame(0), 0));
    assert_eq!(le16(&bytes(&tx, 0, 2), 0), 2);
}

#[test]
fn at_most_sixteen_frames_are_in_flight_until_the_firmware_answers() {
    let tx = Mem::new(TX_REGION, DEV);
    let m = Doorbells::default();
    let mut q = TxQueue::new(0);
    q.start(3, 0xFFF8);
    for i in 0..TXQ_FRAMES {
        assert!(q.send(&m, &*tx, &mgmt_frame(8), 0), "frame {i}");
    }
    assert!(!q.send(&m, &*tx, &mgmt_frame(8), 0), "the seventeenth waits");
    assert_eq!(m.0.borrow().len(), TXQ_FRAMES, "no doorbell for the refused frame");
    // The write pointer ran over 16 bits: 0xFFF8 + 16 = 8.
    assert_eq!(m.0.borrow().last(), Some(&(HBUS_TARG_WRPTR, 8 | 3 << 16)));
    // An answer naming more slots than are in flight frees nothing.
    assert!(!q.reclaim(0xFFF8u16.wrapping_add(17)));
    assert_eq!(q.in_flight(), TXQ_FRAMES);
    assert!(q.reclaim(0xFFF8u16.wrapping_add(10)));
    assert_eq!(q.in_flight(), 6);
    assert!(q.send(&m, &*tx, &mgmt_frame(8), 0));
    assert!(q.reclaim(9));
    assert_eq!(q.in_flight(), 0);
}

#[test]
fn a_frame_too_large_for_its_buffer_or_malformed_is_refused() {
    let tx = Mem::new(TX_REGION, DEV);
    let m = Doorbells::default();
    let mut q = TxQueue::new(0);
    q.start(1, 0);
    assert!(!q.send(&m, &*tx, &mgmt_frame(TX_BUF), 0));
    assert!(!q.send(&m, &*tx, &[0xB0, 0, 0, 0], 0), "shorter than its header");
    assert!(m.0.borrow().is_empty());
    assert_eq!(q.in_flight(), 0);
}

fn tx_response(frames: u8, queue: u16, status: u16, ssn: u16, len: usize) -> Vec<u8> {
    let mut p = vec![0u8; len];
    p[0] = frames;
    p[36..38].copy_from_slice(&queue.to_le_bytes());
    p[40..42].copy_from_slice(&status.to_le_bytes());
    let at = 40 + 4 * frames as usize;
    if at + 4 <= len {
        p[at..at + 2].copy_from_slice(&ssn.to_le_bytes());
    }
    p
}

#[test]
fn a_tx_response_names_its_queue_its_next_index_and_the_outcome() {
    assert_eq!(parse_resp(&tx_response(1, 9, 0x0001, 6, 48)), Some(TxStatus { queue: 9, ssn: 6, sent: true }));
    assert_eq!(parse_resp(&tx_response(1, 9, 0x0002, 6, 48)).map(|s| s.sent), Some(true));
    assert_eq!(parse_resp(&tx_response(1, 9, 0x0083, 6, 48)).map(|s| s.sent), Some(false), "long retry limit");
    assert_eq!(parse_resp(&tx_response(1, 9, 0x0001, 6, 47)), None, "the index runs past the packet");
    assert_eq!(parse_resp(&tx_response(0, 9, 0x0001, 6, 48)), None, "no frames");
    assert_eq!(parse_resp(&tx_response(200, 9, 0x0001, 6, 48)), None, "more statuses than bytes");
    assert_eq!(parse_resp(&[]), None);
}

// An RX MPDU notification: the 64-byte descriptor then `mpdu`.
fn rx(mpdu: &[u8], status: u32, flags1: u8, flags2: u8) -> Vec<u8> {
    let mut p = vec![0u8; 64];
    p[0..2].copy_from_slice(&(mpdu.len() as u16).to_le_bytes());
    p[2] = flags1;
    p[3] = flags2;
    p[12..16].copy_from_slice(&status.to_le_bytes());
    p.extend_from_slice(mpdu);
    p
}

const OK: u32 = 0x3;
const SEC_CCM: u32 = 2 << 8;
const MIC_OK: u32 = 1 << 6;

fn data(protected: bool, body: &[u8]) -> Vec<u8> {
    let fc: u16 = 0x0208 | if protected { 0x4000 } else { 0 };
    let mut f = fc.to_le_bytes().to_vec();
    f.extend_from_slice(&[0u8; 22]);
    f.extend_from_slice(body);
    f
}

#[test]
fn a_clear_data_frame_is_passed_whole() {
    let f = data(false, b"\xAA\xAA\x03\x00\x00\x00\x88\x8Ehandshake");
    assert_eq!(parse_rx(&rx(&f, OK, 0, 0)), Some(RxMpdu { frame: f.clone(), decrypted: false }));
    assert_eq!(parse_rx(&rx(&f, 0x1, 0, 0)), None, "overrun");
    assert_eq!(parse_rx(&rx(&f, 0x2, 0, 0)), None, "bad CRC");
}

#[test]
fn a_decrypted_frame_keeps_its_ccmp_header_and_loses_the_pad_after_it() {
    let ccmp = [1, 0, 0, 0x20, 0, 0, 0, 0];
    let mut on_air = data(true, &ccmp);
    on_air.extend_from_slice(&[0xEE, 0xEE]); // the pad, after the CCMP header
    on_air.extend_from_slice(b"plaintext");
    on_air.extend_from_slice(&[0xCC; 4]); // a CRC the accelerator left
    let got = parse_rx(&rx(&on_air, OK | SEC_CCM | MIC_OK, 2 << 4, 0x20)).unwrap();
    let mut want = data(true, &ccmp);
    want.extend_from_slice(b"plaintext");
    assert_eq!(got, RxMpdu { frame: want, decrypted: true });
    assert_eq!(parse_rx(&rx(&on_air, OK | SEC_CCM, 2 << 4, 0x20)), None, "the MIC failed");
}

#[test]
fn a_frame_the_firmware_had_no_key_for_is_passed_on_encrypted() {
    let mut on_air = data(true, &[1, 0, 0, 0x20, 0, 0, 0, 0]);
    on_air.extend_from_slice(&[0x55; 20]);
    for sec in [0, 7 << 8] {
        let got = parse_rx(&rx(&on_air, OK | sec, 0, 0)).unwrap();
        assert_eq!(got, RxMpdu { frame: on_air.clone(), decrypted: false });
    }
    for sec in [1 << 8, 3 << 8, 4 << 8, 5 << 8] {
        assert_eq!(parse_rx(&rx(&on_air, OK | sec | MIC_OK, 0, 0)), None, "cipher {sec:#x}");
    }
}

#[test]
fn the_pad_of_a_clear_frame_sits_after_its_qos_header() {
    let mut f = vec![0x88, 0x02];
    f.extend_from_slice(&[0u8; 22]);
    f.extend_from_slice(&[0x07, 0x00]); // QoS control: 26-byte header
    let mut on_air = f.clone();
    on_air.extend_from_slice(&[0xEE, 0xEE]);
    on_air.extend_from_slice(b"body");
    let got = parse_rx(&rx(&on_air, OK, 0, 0x20)).unwrap();
    f.extend_from_slice(b"body");
    assert_eq!(got.frame, f);
}

#[test]
fn device_lengths_are_held_to_the_packet() {
    let f = data(false, b"abcd");
    let mut p = rx(&f, OK, 0, 0);
    p[0..2].copy_from_slice(&((f.len() + 1) as u16).to_le_bytes());
    assert_eq!(parse_rx(&p), None, "the frame runs past the packet");
    assert_eq!(parse_rx(&rx(&[0x08, 0x02, 0, 0], OK, 0, 0)), None, "shorter than its header");
    let mut control = vec![0xD4, 0];
    control.resize(30, 0);
    assert_eq!(parse_rx(&rx(&control, OK, 0, 0)), None, "a control frame");
    assert_eq!(parse_rx(&p[..63]), None, "shorter than the descriptor");
    // A MIC and CRC count longer than the frame cuts nothing; the header
    // still has to fit.
    let short = rx(&f[..24], OK, 0xF0, 0x20);
    assert_eq!(parse_rx(&short), None, "the pad leaves less than the header");
}
