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

//! What one look at a waiting connect decides.

const ESTABLISHED: u8 = 3;
/*
 * The peer sent FIN right after the handshake. The three-way still completed,
 * so the connection is usable (buffered data is readable, the local half can
 * still send); a fast-closing server would otherwise be seen only in this
 * state and never as ESTABLISHED.
 */
const CLOSE_WAIT: u8 = 4;
const CLOSED: u8 = 0xFF;

#[derive(Debug, PartialEq, Eq)]
pub enum Verdict {
    Waiting,
    Up,
    Failed,
}

/// Decide from net.tcp's answer for the connection (its state, or the error
/// the query returned) and the clock. Every state other than the ones named
/// is still a handshake in progress, until the deadline.
pub fn decide(state: Result<u8, u16>, now: i64, deadline: i64) -> Verdict {
    match state {
        Ok(ESTABLISHED) | Ok(CLOSE_WAIT) => Verdict::Up,
        Ok(CLOSED) | Err(_) => Verdict::Failed,
        Ok(_) if now.wrapping_sub(deadline) > 0 => Verdict::Failed,
        Ok(_) => Verdict::Waiting,
    }
}
