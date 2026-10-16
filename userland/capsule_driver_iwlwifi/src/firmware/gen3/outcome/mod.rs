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

//! How far a gen3 bring-up got, in the terms the Wi-Fi control family and the
//! boot console use. A failure names its step, carries a detail word (the
//! refused id, the ALIVE status, the command that went unanswered) and maps
//! to the stage code `nonos_wifi_client` reads, so a panel on a machine with
//! no serial port still says whether the card was refused, never powered,
//! got no DMA memory or ran a firmware that did not come up.

mod detail;
mod kinds;
mod stage;
mod subcodes;
mod text;

pub use kinds::*;
