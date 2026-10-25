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

//! One command to the PPM and its acknowledgement: CONTROL written and
//! signalled, the CCI polled until the command completes, MESSAGE IN read,
//! then ACK_CC_CI and its own completion. A PPM that never answers costs a
//! bounded number of polls, never a hang. Every connector the CCI names on
//! the way is kept in `pending`, so a change is not lost to a command.

use super::command::{ack, reset as reset_command};
use super::poll::{poll, send};
use super::ppm::Ppm;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fault {
    /// The mailbox or the _DSM could not be reached.
    Signal,
    /// No completion within `POLLS`.
    Stuck,
    NotSupported,
    Failed,
}

/// PPM_RESET, complete when the CCI says so; it needs no acknowledgement.
pub fn reset(ppm: &mut impl Ppm, pending: &mut u128) -> Result<(), Fault> {
    send(ppm, reset_command())?;
    poll(ppm, pending, |c| c.reset_completed).map(|_| ())
}

/// The command's MESSAGE IN; `change` also acknowledges a connector change.
pub fn command(
    ppm: &mut impl Ppm,
    pending: &mut u128,
    control: u64,
    change: bool,
) -> Result<[u8; 16], Fault> {
    send(ppm, control)?;
    let done = poll(ppm, pending, |c| c.command_completed && !c.busy)?;
    let mut data = [0u8; 16];
    if done.data_len != 0 && !ppm.read_message(&mut data) {
        return Err(Fault::Signal);
    }
    send(ppm, ack(true, change))?;
    poll(ppm, pending, |c| c.ack_command)?;
    match (done.not_supported, done.error) {
        (true, _) => Err(Fault::NotSupported),
        (_, true) => Err(Fault::Failed),
        _ => Ok(data),
    }
}
