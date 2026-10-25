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

//! The storage protocol's packets, built and read (constants/vstor.rs has
//! the layout). Pure: what a request says and what an answer means.

use nonos_scsi::{Addr, Dir, Fault, Outcome, SENSE_MAX};

use crate::constants::vstor::*;

pub type Packet = [u8; PACKET];

fn put16(p: &mut Packet, at: usize, v: u16) {
    p[at..at + 2].copy_from_slice(&v.to_le_bytes());
}

fn put32(p: &mut Packet, at: usize, v: u32) {
    p[at..at + 4].copy_from_slice(&v.to_le_bytes());
}

fn get32(p: &[u8], at: usize) -> u32 {
    u32::from_le_bytes([p[at], p[at + 1], p[at + 2], p[at + 3]])
}

/// A request of `operation` with nothing in its body, a completion asked.
pub fn op(operation: u32) -> Packet {
    let mut p = [0u8; PACKET];
    put32(&mut p, 0, operation);
    put32(&mut p, 4, FLAG_REQUEST_COMPLETION);
    p
}

/// QUERY_PROTOCOL_VERSION for `version`, revision 0 (Windows' alone).
pub fn version(version: u16) -> Packet {
    let mut p = op(OP_QUERY_PROTOCOL_VERSION);
    put16(&mut p, 12, version);
    p
}

/// EXECUTE_SRB for `cdb` on `at`, moving `len` bytes in `dir`, as Linux's
/// storvsc_queuecommand fills it: untagged simple queueing, synchronous
/// transfer off, sense of 20 bytes asked, 60 seconds for the host.
pub fn srb(at: Addr, cdb: &[u8], dir: Dir, len: u32) -> Option<Packet> {
    if cdb.is_empty() || cdb.len() > CDB_MAX || at.lun > 0xFF || at.target > 0xFF {
        return None;
    }
    let (data_in, flags) = match dir {
        Dir::None => (DATA_UNKNOWN, 0),
        Dir::In => (DATA_READ, SRB_FLAGS_DATA_IN),
        Dir::Out => (DATA_WRITE, SRB_FLAGS_DATA_OUT),
    };
    let mut p = op(OP_EXECUTE_SRB);
    put16(&mut p, 12, SRB_LENGTH);
    p[18] = at.target as u8;
    p[19] = at.lun as u8;
    p[20] = cdb.len() as u8;
    p[21] = SENSE_BYTES as u8;
    p[22] = data_in;
    put32(&mut p, 24, len);
    p[28..28 + cdb.len()].copy_from_slice(cdb);
    p[50] = QUEUE_TAG_UNTAGGED;
    p[51] = QUEUE_ACTION_SIMPLE;
    let queue = SRB_FLAGS_QUEUE_ACTION_ENABLE | SRB_FLAGS_NO_QUEUE_FREEZE;
    put32(&mut p, 52, SRB_FLAGS_DISABLE_SYNCH_TRANSFER | queue | flags);
    put32(&mut p, 56, TIME_OUT_S);
    Some(p)
}

/// An answer's operation and status, the first 12 bytes every packet has.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Reply {
    pub operation: u32,
    pub status: u32,
}

/// The head of a packet the host sent, or None for one too short to be one.
pub fn reply(b: &[u8]) -> Option<Reply> {
    (b.len() >= 12).then(|| Reply { operation: get32(b, 0), status: get32(b, 8) })
}

/// Whether an answer to one of the start's requests says it was done.
pub fn done(b: &[u8]) -> bool {
    b.len() >= PACKET && reply(b) == Some(Reply { operation: OP_COMPLETE_IO, status: 0 })
}

/// QUERY_PROPERTIES' answer: the largest transfer the host takes.
pub fn max_transfer(b: &[u8]) -> Option<u32> {
    done(b).then(|| get32(b, 24))
}

/// Commands Hyper-V's SCSI emulation answers wrongly, whose failure Linux
/// takes as success with whatever data came (storvsc_host_mishandles_cmd):
/// INQUIRY, MODE SENSE (6) and MODE SENSE (10).
fn mishandled(op: u8) -> bool {
    matches!(op, 0x12 | 0x1A | 0x5A)
}

const TEST_UNIT_READY: u8 = 0x00;
const CHECK_CONDITION: u8 = 0x02;

/// What a completed EXECUTE_SRB means for a command `cdb` that was to move
/// `len` bytes, as Linux's storvsc_on_io_completion and
/// storvsc_command_completion read it.
pub fn outcome(b: &[u8], cdb: &[u8], len: u32) -> Result<Outcome, Fault> {
    if b.len() < PACKET || reply(b).map(|r| r.operation) != Some(OP_COMPLETE_IO) {
        return Err(Fault::Failed);
    }
    let (mut srb, mut scsi) = (b[14], b[15]);
    let op = cdb.first().copied().unwrap_or(0xFF);
    if mishandled(op) {
        (srb, scsi) = (SRB_STATUS_SUCCESS, 0);
    }
    let autosense = srb & SRB_STATUS_AUTOSENSE_VALID != 0;
    let status = srb & !(SRB_STATUS_AUTOSENSE_VALID | SRB_STATUS_QUEUE_FROZEN);
    // The host says how much moved on success and on a short transfer
    // alone; never more than was asked.
    let moved = match status {
        SRB_STATUS_SUCCESS | SRB_STATUS_DATA_OVERRUN => get32(b, 24).min(len),
        _ => 0,
    };
    let mut o = Outcome::good();
    o.status = scsi;
    o.residual = len - moved;
    if scsi == CHECK_CONDITION && autosense {
        o.sense_len = (b[21] as usize).min(SENSE_BYTES).min(SENSE_MAX);
        o.sense[..o.sense_len].copy_from_slice(&b[28..28 + o.sense_len]);
    }
    match status {
        SRB_STATUS_SUCCESS | SRB_STATUS_DATA_OVERRUN => Ok(o),
        SRB_STATUS_INVALID_LUN | SRB_STATUS_SELECTION_TIMEOUT => Err(Fault::NoTarget),
        // The target's own answer, with its sense: the command layer reads it.
        _ if autosense => Ok(o),
        // Some hosts fail TEST UNIT READY so; its status byte says the rest.
        _ if op == TEST_UNIT_READY => Ok(o),
        SRB_STATUS_BUS_RESET | SRB_STATUS_ABORTED => Err(Fault::Reset),
        SRB_STATUS_TIMEOUT => Err(Fault::Timeout),
        _ => Err(Fault::Failed),
    }
}
