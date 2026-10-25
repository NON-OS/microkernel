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
// The verb transport (verb.rs, ring_recover.rs), included into `controller`.

#[path = "../../../capsule_driver_hda/src/controller/immediate.rs"]
pub mod immediate;
#[path = "../../../capsule_driver_hda/src/controller/ring_health.rs"]
pub mod ring_health;
#[path = "../../../capsule_driver_hda/src/controller/ring_recover.rs"]
pub mod ring_recover;
#[path = "../../../capsule_driver_hda/src/controller/ring_reset.rs"]
pub mod ring_reset;
#[path = "../../../capsule_driver_hda/src/controller/ring_steps.rs"]
mod ring_steps;
#[path = "../../../capsule_driver_hda/src/controller/rirb.rs"]
mod rirb;
#[path = "../../../capsule_driver_hda/src/controller/verb.rs"]
pub mod verb;
#[path = "../../../capsule_driver_hda/src/controller/verb_send.rs"]
mod verb_send;
