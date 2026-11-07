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

//! Any bytes as what a USB stick or the xHCI capsule answers: the readers
//! never panic; a configuration descriptor binds at most `MAX_BINDINGS`
//! interfaces, each with an IN and an OUT bulk endpoint and a burst of at
//! most 15, and encodes to exactly its own length; a status wrapper passes
//! validation only when it echoes the tag and moved no more than was asked;
//! READ CAPACITY(16) never reports a count that wrapped; a UAS sense lies
//! inside its IU; and an xHCI reply never names data past the bytes that
//! came or the buffer it is copied to.

#![no_main]

use libfuzzer_sys::fuzz_target;
use nonos_uas::{status_iu, StatusIu};
use usb_msc_proofs::bot::{is_csw, parse as csw, validate, TransferOutcome};
use usb_msc_proofs::descriptors::{encode_probe, parse_config};
use usb_msc_proofs::protocol::{parse as request, CSW_LEN, MAX_BINDINGS};
use usb_msc_proofs::scsi::{block_request, parse_capacity, parse_capacity16, parse_inquiry, parse_sense};
use usb_msc_proofs::xhci::port_list::connected;
use usb_msc_proofs::xhci::reply::{data_in, moved, reply};
use usb_msc_proofs::xhci::wire::DATA_AT;

const IN: u8 = 0x80;

fn config(data: &[u8]) {
    let Ok(p) = parse_config(data) else { return };
    assert!(p.count >= 1 && p.count <= MAX_BINDINGS);
    for b in &p.bindings[..p.count] {
        assert!(b.bulk_in & IN != 0 && b.bulk_out != 0 && b.bulk_out & IN == 0);
        assert!(b.max_burst_in <= 15 && b.max_burst_out <= 15);
    }
    let mut out = [0u8; 4 + 8 * MAX_BINDINGS];
    assert_eq!(encode_probe(&p, &mut out), 4 + 8 * p.count);
}

fn status(data: &[u8], asked: u32) {
    let raw = data.get(..CSW_LEN).unwrap_or(data);
    let Ok(s) = csw(raw) else { return };
    assert!(is_csw(raw) && s.status <= 2);
    for tag in [s.tag, s.tag.wrapping_add(1)] {
        match validate(s, tag, asked) {
            Ok(TransferOutcome::Passed { transferred } | TransferOutcome::Failed { transferred }) => {
                assert!(tag == s.tag && s.status != 2 && transferred + s.residue == asked);
            }
            Err(_) => assert!(tag != s.tag || s.status == 2 || s.residue > asked),
        }
    }
}

fn scsi(data: &[u8]) {
    if let Some(c) = parse_capacity(data) {
        assert!(c.block_count() >= 1 && c.capacity_bytes() >= u64::from(c.block_len));
    }
    if let Some((count, _)) = parse_capacity16(data) {
        assert!(count >= 1);
    }
    if let Some(i) = parse_inquiry(data) {
        assert!(i.peripheral_type <= 0x1F && i.vendor == data[8..16]);
    }
    if let Some(s) = parse_sense(data) {
        assert!(s.sense_key <= 0x0F && matches!(data[0] & 0x7F, 0x70 | 0x71));
    }
    if let Ok((_, blocks)) = block_request(data) {
        assert!(data.len() == 6 && blocks != 0);
    }
    if let Some((_, body)) = request(data) {
        assert!(body.len() <= data.len());
    }
}

fn uas(data: &[u8]) {
    let Some(iu) = status_iu(data) else { return };
    assert_eq!(iu.tag(), u16::from_be_bytes([data[2], data[3]]));
    if let StatusIu::Sense { sense, .. } = iu {
        let len = usize::from(u16::from_be_bytes([data[14], data[15]]));
        assert!(sense.len() == len && 16 + len <= data.len());
    }
}

fn xhci(data: &[u8], n: i64, len: usize, head: usize) {
    if let Ok(payload) = reply(data, n, 0) {
        assert!(n >= 0 && DATA_AT + payload <= n as usize && n as usize <= data.len());
    }
    let mut out = [0u8; 64];
    if let Ok(count) = data_in(data, len, head, &mut out) {
        assert!(count <= out.len() && head + count <= len && DATA_AT + head + count <= data.len());
    }
    if let Ok(ports) = connected(data, len) {
        assert!(4 + 8 * ports.len() <= len);
    }
    assert!(moved(data, len).is_err() || len >= 4);
}

fuzz_target!(|data: &[u8]| {
    let Some((knobs, rest)) = data.split_first_chunk::<4>() else { return };
    let asked = u32::from(u16::from_le_bytes([knobs[0], knobs[1]]));
    config(rest);
    status(rest, asked);
    scsi(rest);
    uas(rest);
    let n = i64::from(knobs[2]) - 16;
    xhci(rest, n, usize::from(knobs[3]), 2 + 2 * usize::from(knobs[0] & 1));
});
