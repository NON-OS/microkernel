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

//! The restart policy a watched service is held to, from kernel source.

#[path = "../../../src/services/lifecycle/state/constants.rs"]
mod constants;
#[path = "../../../src/services/lifecycle/state/respawn.rs"]
mod respawn;
// pid and generation are read by liveness.rs, which needs the process table
// and is not included; the type is the kernel's own, restyled nowhere here.
#[allow(dead_code, clippy::new_without_default)]
#[path = "../../../src/services/lifecycle/state/types.rs"]
mod types;

pub use types::CapsuleState;
