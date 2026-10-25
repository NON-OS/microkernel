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

//! The air between the capsule and one HID device: a controller as Vol 4,
//! Part E describes it (or lying, `lies`), SMP's responder, HOGP's peripheral.

use std::cell::Cell;
use std::collections::VecDeque;
use std::rc::Rc;

use super::act::Act;
use super::lies::Lies;
use super::seen::Seen;
use crate::hci_sim::config::Config;
use crate::hogp_sim::Peripheral;
use crate::smp_sim::Peer;
use nonos_bt_wire::l2cap::Reassembler;

pub const HANDLE: u16 = 0x0040;
/// The two addresses the SMP responder computes its checks with
/// (`smp_sim::A` and `smp_sim::B`), in wire order: ours public, the
/// peer's static random.
pub const OURS: [u8; 6] = [0x33, 0x22, 0x11, 0x00, 0xde, 0xc0];
pub const PEER: [u8; 6] = [0x77, 0x66, 0x55, 0x44, 0x5a, 0xd6];

pub struct Air {
    pub cfg: Config,
    /// The peer's advertising data.
    pub adv: Vec<u8>,
    /// The peer's report type: ADV_IND unless a test makes it directed.
    pub adv_kind: u8,
    /// Advertising Reports from other devices, heard before the peer's.
    pub others: Vec<Vec<u8>>,
    pub smp: Peer,
    pub hid: Peripheral,
    /// The passkey the host showed its user, typed on the peer.
    pub shown: Rc<Cell<Option<u32>>>,
    pub script: VecDeque<Act>,
    /// Every command packet, in order.
    pub commands: Vec<Vec<u8>>,
    /// ATT PDUs the peer sends as soon as the link is up.
    pub early_att: Vec<Vec<u8>>,
    /// The host's answers to them.
    pub att_answers: Vec<Vec<u8>>,
    /// Every signaling frame the host sent.
    pub signals: Vec<Vec<u8>>,
    /// The key the host encrypted with, once it did.
    pub ltk: Option<[u8; 16]>,
    pub refuse_encryption: bool,
    /// Addresses, identity keys and pairing, as the peer saw them.
    pub seen: Seen,
    /// Number of Completed Packets held until the host next pauses, so it
    /// meets a controller with no buffer free.
    pub lazy_credits: bool,
    pub(super) held: Vec<Vec<u8>>,
    pub pauses: u64,
    pub(super) interrupt: VecDeque<Vec<u8>>,
    pub(super) bulk_in: VecDeque<Vec<u8>>,
    pub(super) joiner: Reassembler,
    pub(super) idle: u32,
    pub lies: Lies,
}
