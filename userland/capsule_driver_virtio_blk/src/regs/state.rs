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

use nonos_virtio::Mmio;

use super::io::RegIo;

/// The legacy register window. Every access stays inside it: a memory
/// window checks each offset against its mapped length (a read outside
/// answers all ones, a write outside is dropped), and a port grant is
/// checked by the broker against the BAR it covers. No accessor is unsafe
/// to call.
#[derive(Clone, Copy)]
pub struct Regs {
    pub(super) io: RegIo,
}

impl Regs {
    pub const fn mmio(window: Mmio) -> Self {
        Self { io: RegIo::Mmio(window) }
    }

    pub const fn pio(grant_id: u64) -> Self {
        Self { io: RegIo::Pio(grant_id) }
    }
}
