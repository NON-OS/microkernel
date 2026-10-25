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

//! The fake controller's state.

use super::ring::Ring;
use crate::admin::Submission;

#[derive(Default)]
pub(crate) struct Fake {
    /// CSTS reads all ones and nothing is served: the drive was pulled.
    pub gone: bool,
    /// A predecessor left it running: until RDY falls it writes the frames
    /// its queues and host memory buffer named, whenever it may master.
    pub armed: bool,
    /// CSTS.CFS is set until a disable; nothing is served.
    pub fatal: bool,
    /// RDY and CFS never clear: no reset brings it back.
    pub wedged: bool,
    /// Admin commands go unanswered until a disable.
    pub mute: bool,
    /// An answered Abort completes the command it names.
    pub abort_frees: bool,
    /// DW0 of the Abort's own completion: bit 0 set says not aborted.
    pub abort_dw0: u32,
    /// SET FEATURES Host Memory Buffer is refused (Invalid Field).
    pub refuse_hmb: bool,
    /// SET FEATURES Host Memory Buffer is never answered.
    pub ignore_hmb: bool,
    /// I/O commands never completed on their own.
    pub stall: Vec<u16>,
    /// I/O commands completed with this status field, not success.
    pub fail: Vec<(u16, u16)>,
    /// Clock reads a command takes to complete once fetched.
    pub hold: u64,
    /// Clock reads RDY takes to follow a change of CC.EN.
    pub slow: u64,
    /// Clock reads SHST takes to read complete after CC.SHN is set; with
    /// `never_shut` it never does.
    pub shut_after: u64,
    pub never_shut: bool,
    /// The clock read CC.SHN was first seen at, until a disable.
    pub(super) shut: Option<u64>,
    /// Every I/O command id fetched; ids carry on across resets (io::take).
    pub(super) fetched: Vec<u16>,
    /// I/O commands fetched and not yet completed, with the read they are due.
    pub(super) pending: Vec<(u16, u64)>,
    /// Admin commands fetched and not yet answered, likewise.
    pub(super) asked: Vec<(Submission, u64)>,
    /// Every admin command fetched, answered or not.
    pub admin_seen: Vec<Submission>,
    /// Clock reads so far: every wait the driver makes reads the clock.
    pub ticks: u64,
    /// CC.EN as last read, and the clock read it changed at.
    pub(super) en: (bool, u64),
    pub(super) admin: Ring,
    pub(super) io: Ring,
}
