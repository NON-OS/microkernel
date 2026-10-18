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

//! The gen3 bring-up run end to end against the modeled device, with the
//! bundled so-a0-gf-a0-86 image: the start sequence, memory laid out across
//! broker-sized regions, the kick, ALIVE, the platform-NVM step, the
//! post-ALIVE commands in Linux's order, and a passive scan that hands up the
//! beacon the device received. Then each way it can fail: the NIC never
//! ready, no ALIVE, a dead ALIVE status, a command with no answer, a
//! firmware error mid-sequence. What only silicon can show (that the ROM
//! accepts this memory, that the firmware answers like this) is left to a
//! boot; the order, the bounds and the parsing are proven here.

mod alive;
mod answers;
mod errors;
mod fails;
mod iface;
mod order;
mod rig;
