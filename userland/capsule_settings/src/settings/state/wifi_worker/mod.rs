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

//! The Wi-Fi scan and join, made on a worker thread.
//!
//! Each is one call to the driver that answers only when the radio work is
//! done (up to 15 s for a scan, 30 s for a join), and the answer is the reply
//! itself, so the call has to wait. A worker waits for it while the window
//! keeps painting and taking keys; a tick picks the answer up (`wifi_step`).
//! One worker at a time: the driver is busy with it, so nothing else is asked
//! of the driver until it answers (`Pending::is_out`).
//!
//! The kernel admits a caller to the Wi-Fi drivers when its process owns the
//! Settings endpoint, and judges a thread by its process (4e4c172a), so the
//! worker is admitted where the window thread is.
//!
//! The passphrase of a join travels in the request, in a buffer wiped when
//! the request is dropped, whichever thread drops it and however the join
//! went. Remembering the network (sealing it with the TPM key) happens on the
//! worker too, while it still holds the passphrase; the answer carries only
//! the outcome, never the passphrase. The answer lands in a static, so a
//! worker that finishes after Settings closed writes into memory still there.

mod ask;
mod seat;
mod secret;
mod work;

pub(crate) use ask::REMEMBERED;
pub(super) use ask::{Answer, Ask};
pub(super) use seat::{poll, send, Sent};
pub(super) use secret::Secret;
pub(super) use work::perform;
