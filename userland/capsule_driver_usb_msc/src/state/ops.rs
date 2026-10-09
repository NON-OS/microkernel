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

use crate::bot::{validate, CommandStatus, TransferOutcome, ValidateError};
use crate::descriptors::ProbeResult;
use crate::protocol::{E_INVAL, E_PHASE};

use super::types::State;

/// The two commands whose answer Linux uses to find a device that reports
/// bogus residues, with the lengths it asks of them: INQUIRY of 36 bytes and
/// READ CAPACITY(10) of 8.
const INQUIRY: (u8, u32) = (0x12, 36);
const READ_CAPACITY10: (u8, u32) = (0x25, 8);

impl State {
    /// A new device is being bound: what was learned of the last one goes.
    pub fn install_bindings(&mut self, probe: &ProbeResult) {
        self.bindings = probe.bindings;
        self.binding_count = probe.count;
        self.probes = self.probes.saturating_add(1);
        self.ignore_residue = false;
    }

    fn next_tag(&mut self) -> u32 {
        let tag = self.next_tag;
        self.next_tag = self.next_tag.wrapping_add(1).max(1);
        self.last_tag = tag;
        tag
    }

    /// Open a command block: assign it a fresh tag, record how many data bytes
    /// it asks for, and mark a transfer as outstanding. The tag and length are
    /// what the returning status wrapper is later checked against.
    pub fn begin_command(&mut self, data_len: u32) -> u32 {
        self.last_data_len = data_len;
        self.pending = true;
        self.next_tag()
    }

    /// Close a command block against its status wrapper. The CSW is validated
    /// (signature and range by the parser, tag echo and residue here) before it
    /// is trusted. Returns the SCSI-level status (0 passed, 1 CHECK CONDITION)
    /// or a protocol error if the wrapper does not belong to this command.
    ///
    /// Once the device is known to report bogus residues its residue is not
    /// looked at, here or by `residue`.
    pub fn finish_command(&mut self, csw: CommandStatus) -> Result<u8, i32> {
        self.pending = false;
        self.residue_bytes = self.residue_bytes.saturating_add(csw.residue as u64);
        let csw = if self.ignore_residue { CommandStatus { residue: 0, ..csw } } else { csw };
        match validate(csw, self.last_tag, self.last_data_len) {
            Ok(TransferOutcome::Passed { .. }) => {
                self.csw_ok = self.csw_ok.saturating_add(1);
                Ok(0)
            }
            Ok(TransferOutcome::Failed { .. }) => {
                self.csw_failed = self.csw_failed.saturating_add(1);
                Ok(1)
            }
            Err(ValidateError::TagMismatch) => {
                self.phase_errors = self.phase_errors.saturating_add(1);
                Err(E_INVAL)
            }
            Err(ValidateError::PhaseError) => {
                self.phase_errors = self.phase_errors.saturating_add(1);
                Err(E_PHASE)
            }
        }
    }

    /// The residue of the command `op` just finished with `csw`: of the
    /// `data_len` bytes it asked for, `moved` crossed the bus. Taken as
    /// Linux's usb_stor_Bulk_transport takes it, the larger of what the host
    /// saw missing and what the CSW reports, the latter at most `data_len`.
    /// The CSW alone was trusted: a data phase that ended short under a CSW
    /// residue of zero passed as whole, and the bytes never received were
    /// handed on as data.
    ///
    /// Some devices report a residue that has nothing to do with what they
    /// moved. Linux finds them as is done here: an INQUIRY of 36 bytes or a
    /// READ CAPACITY(10) of 8 that passed with every byte received and a
    /// residue all the same. From then on only the bytes that crossed the
    /// bus count. Such a stick failed READ CAPACITY and was never bound.
    pub fn residue(&mut self, op: u8, data_len: u32, moved: u32, csw: CommandStatus) -> u32 {
        let short = data_len.saturating_sub(moved);
        if csw.residue == 0 || self.ignore_residue {
            return short;
        }
        let probe = (op, data_len) == INQUIRY || (op, data_len) == READ_CAPACITY10;
        if probe && csw.status == 0 && short == 0 {
            self.ignore_residue = true;
            return short;
        }
        short.max(csw.residue.min(data_len))
    }
}
