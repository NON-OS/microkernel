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

//! The transport's waits: how long a command may take, why a wait ended,
//! and what a packet handler wants next.

/// `HOST_COMPLETE_TIMEOUT`: how long a command may take to answer.
pub const COMMAND_MS: u32 = 2000;
/// Received buffers taken per pass.
pub(super) const PASS: usize = 32;
/// `TX_CMD`: in the legacy group, a frame's transmit response. It carries
/// the frame's queue and index as its sequence and no `SEQ_RX_FRAME`, so it
/// can look like a command's reply; Linux never takes it for one
/// (`no_reclaim_cmds` in `iwl_pcie_rx_handle_rb`).
pub(super) const TX_RESPONSE: u8 = 0x1C;

/// Why a wait ended without what it waited for.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum WaitError {
    TimedOut,
    /// The firmware or the hardware raised its error cause.
    FirmwareError,
    /// The command did not fit the command buffer.
    TooLarge,
    /// An earlier command is still unanswered, and its buffer may still be
    /// read: nothing more is sent until it is retired.
    Busy,
}

/// What a packet handler wants next.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Flow {
    Continue,
    Done,
}
