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

/*
 * Why a block request did not produce usable sector bytes. Transport carries
 * the raw syscall return of a store write, or a store read that timed out;
 * Status carries the errno a store read was refused with, so a device fault
 * is never mistaken for a lost reply.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlkError {
    NoService,
    Transport(i64),
    ShortReply(usize),
    BadLength,
    Status(i32),
    Inval,
    BadContainer,
    Exists,
}
