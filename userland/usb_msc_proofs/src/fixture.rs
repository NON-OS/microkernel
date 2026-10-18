// NONOS Operating System (AGPL-3.0-or-later)
//! What the tests build their devices, answers and requests from.

use crate::descriptors::MscBinding;
use crate::disk::Disk;
use crate::protocol::{HDR_LEN, MAGIC, VERSION};
use crate::xhci::{device, Answer};

const TEST_UNIT_READY: u8 = 0x00;

/// A bound stick on root port 1: 1000 blocks of 512 bytes.
pub fn disk() -> Disk {
    let b = MscBinding { interface: 0, bulk_in: 0x81, bulk_out: 0x02, ..MscBinding::default() };
    let mut d = Disk::bound(1, 1, &b);
    (d.port, d.blocks, d.block_len) = (1, 1000, 512);
    d
}

/// Bulk-Only Mass Storage Resets the transport sent.
pub fn class_resets() -> usize {
    device(|d| d.controls.iter().filter(|c| (c.0, c.1) == (0x21, 0xFF)).count())
}

pub fn answers_left() -> usize {
    device(|d| d.answers.len())
}

/// The TEST UNIT READYs sent.
pub fn turs() -> usize {
    device(|d| d.commands.iter().filter(|&&op| op == TEST_UNIT_READY).count())
}

/// Fixed-format sense data with `key` and additional sense code `asc`.
pub fn sense(key: u8, asc: u8) -> Vec<u8> {
    let mut s = vec![0u8; 18];
    (s[0], s[2], s[7], s[12]) = (0x70, key, 10, asc);
    s
}

/// A configuration whose one interface is SCSI-transparent Bulk-Only.
pub fn bot_config() -> Vec<u8> {
    let mut c = vec![9, 2, 32, 0, 1, 1, 0, 0x80, 50, 9, 4, 0, 0, 2, 8, 6, 0x50, 0];
    c.extend_from_slice(&[7, 5, 0x81, 2, 0, 2, 0, 7, 5, 0x02, 2, 0, 2, 0]);
    c
}

/// A unit that answers INQUIRY and TEST UNIT READY, then READ CAPACITY(10)
/// with `blocks` of 512 bytes.
pub fn ready_unit(blocks: u32) -> Vec<Answer> {
    let mut cap = (blocks - 1).to_be_bytes().to_vec();
    cap.extend_from_slice(&512u32.to_be_bytes());
    let ok = || Answer::Csw { residue: 0, status: 0 };
    vec![Answer::Data(vec![0; 36]), ok(), ok(), Answer::Data(cap), ok()]
}

/// A request frame for `op` with `body`.
pub fn frame(op: u16, request_id: u32, body: &[u8]) -> Vec<u8> {
    let mut f = vec![0u8; HDR_LEN];
    f[0..4].copy_from_slice(&MAGIC.to_le_bytes());
    f[4..6].copy_from_slice(&VERSION.to_le_bytes());
    f[6..8].copy_from_slice(&op.to_le_bytes());
    f[12..16].copy_from_slice(&request_id.to_le_bytes());
    f[16..20].copy_from_slice(&(body.len() as u32).to_le_bytes());
    f.extend_from_slice(body);
    f
}

/// A block request body: one sector at `lba`.
pub fn one_sector(lba: u64) -> Vec<u8> {
    let mut b = lba.to_le_bytes().to_vec();
    b.extend_from_slice(&1u32.to_le_bytes());
    b
}
