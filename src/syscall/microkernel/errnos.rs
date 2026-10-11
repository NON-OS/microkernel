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

//! Negative-errno values used by microkernel syscall handlers. The
//! sign convention is `-errno`; the syscall return value is `i64` so
//! a successful call returns a non-negative number and a failure
//! returns one of these constants.

pub const ERRNO_PERM: i64 = -1;
pub const ERRNO_NOENT: i64 = -2;
/* The volume or the disk under it could not do what was asked. */
pub const ERRNO_IO: i64 = -5;
pub const ERRNO_CHILD: i64 = -10;
/* No disk is chosen yet, the USB driver still looking; ask again. */
pub const ERRNO_AGAIN: i64 = -11;
pub const ERRNO_NOMEM: i64 = -12;
pub const ERRNO_ACCES: i64 = -13;
pub const ERRNO_FAULT: i64 = -14;
pub const ERRNO_BUSY: i64 = -16;
pub const ERRNO_EXIST: i64 = -17;
pub const ERRNO_NODEV: i64 = -19;
pub const ERRNO_INVAL: i64 = -22;
pub const ERRNO_NOTTY: i64 = -25;
/* A stream fed more bytes than the length named at its start. */
pub const ERRNO_FBIG: i64 = -27;
/* The volume has no room for the bytes still to come. */
pub const ERRNO_NOSPC: i64 = -28;
/* A DMA map whose device address a 32-bit descriptor cannot carry. */
pub const ERRNO_RANGE: i64 = -34;
pub const ERRNO_NOSYS: i64 = -38;
/* The file to import is not the one its digest pins. */
pub const ERRNO_BADMSG: i64 = -74;
pub const ERRNO_NOTSUP: i64 = -95;
/* The boot profile runs no network: Air-Gapped, Safe Mode or Recovery. */
pub const ERRNO_NETDOWN: i64 = -100;
pub const ERRNO_TIMEDOUT: i64 = -110;
/* The file was imported and verified before; nothing to feed. */
pub const ERRNO_ALREADY: i64 = -114;
/* A stream finished before every byte came; it is kept. */
pub const ERRNO_INPROGRESS: i64 = -115;
pub const ERRNO_STALE: i64 = -116;
