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

//! The device's replies swept: every byte of an answer and the length
//! the used ring claims for it are the device's word. Each case is a fresh
//! queue whose next command a scripted device (`device`) answers with a
//! valid reply mutated (`reply`), and the driver's verdict is held to what
//! the virtio-gpu rules say it must be (`oracle`): accepted exactly when
//! well formed, with what it says, and never a reset for a bad body.

mod device;
mod dtd;
mod edid_blob;
mod mutate;
mod oracle;
mod oracle_capset;
mod reply;

mod capset_tests;
mod display_tests;
mod edid_tests;
mod panel_tests;
mod seed_tests;
