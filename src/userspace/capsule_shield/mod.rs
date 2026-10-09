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
//! nonos.shield's kernel side: its signed embed bytes and its boot spawn.
//! It starts with the desktop and waits for the wallet window's calls. Its
//! heap grows when it first proves, to about 0.94 GB from the periodic cache
//! it ships, and stays mapped: the std layer's heap never unmaps. No limit
//! here bounds it; MMAP takes up to 1 GiB a call, the prover asks 256 MiB at
//! most, and the machine's free frames are the only ceiling.

mod embed;
mod spawn;
mod state;

pub use spawn::spawn_shield_capsule;
pub use state::shared_state;
