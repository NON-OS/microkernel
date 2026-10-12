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

//! The HID host on one ATT bearer: discovery, then configuration, then
//! the reports. It sends nothing itself; it keeps one request outstanding
//! at a time, as a client must (Vol 3, Part F, 3.3.2).

use super::phase::Phase;
use super::types::{Decl, Source};
use crate::report::{Keyboard, Mouse};
use alloc::vec::Vec;

pub struct Host {
    pub(super) phase: Phase,
    /// HID service handle ranges.
    pub(super) services: Vec<(u16, u16)>,
    /// The current service's declarations, while they are being read.
    pub(super) decls: Vec<Decl>,
    pub(super) sources: Vec<Source>,
    /// Protocol Mode value handles, to put in boot mode.
    pub(super) modes: Vec<u16>,
    pub(super) keyboard: Keyboard,
    pub(super) mouse: Mouse,
    /// The opcode of the request awaiting its response.
    pub(super) pending: Option<u8>,
    pub(super) sent_ms: u64,
    pub(super) requests: u32,
}

impl Host {
    pub fn phase(&self) -> Phase {
        self.phase
    }

    pub fn sources(&self) -> &[Source] {
        &self.sources
    }

    pub(super) fn expired(&self, now_ms: u64) -> bool {
        self.pending.is_some() && now_ms.saturating_sub(self.sent_ms) >= super::TIMEOUT_MS
    }
}
