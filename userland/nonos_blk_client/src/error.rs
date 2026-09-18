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

//! What went wrong between this capsule and the driver. The driver's own
//! status comes through as the number it sent, so a refusal at the device
//! is told apart from a reply that never arrived.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlkError {
    /// No driver of that kind is registered on this boot.
    NoService,
    /// The kernel refused the call; the value is its errno.
    Transport(i64),
    ShortReply(usize),
    BadHeader,
    IdMismatch,
    BadLength,
    /// The driver answered with a non-zero status.
    Status(i32),
    /// The request was not a whole number of sectors, or too large.
    Inval,
}

impl BlkError {
    /// The value the disk writer carries in its `SinkError`: the driver's
    /// status when there is one, a negative errno otherwise.
    pub fn code(self) -> i32 {
        match self {
            BlkError::Status(s) => s,
            BlkError::Transport(e) => e as i32,
            BlkError::NoService => -19,
            BlkError::Inval => -22,
            BlkError::ShortReply(_) | BlkError::BadHeader | BlkError::IdMismatch => -71,
            BlkError::BadLength => -90,
        }
    }
}
