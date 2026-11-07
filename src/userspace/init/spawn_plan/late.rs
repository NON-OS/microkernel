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

//! What the ACPI capsule publishes can come after init stopped waiting for
//! its first pass: a host it powers with _PS0, or one it publishes once an
//! EC that started late answers. So the stages that need the ACPI stage
//! (`order::after_acpi`) are walked again, in order, whenever the first pass
//! comes or the device table grows; each capsule is spawned once a boot
//! (`once`), so a walk spawns only what is newly there. The first-pass
//! wait is no cut-off for spawning. Pure: `kernel_proofs` mounts it.

/// What a walk is decided on.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) struct Seen {
    pub(super) pass_done: bool,
    pub(super) devices: usize,
}

/// Walk again: the first pass has come, or the table holds more devices,
/// since the last walk. A device leaving is the supervisor's, not a spawn.
pub(super) fn due(last: Seen, now: Seen) -> bool {
    (now.pass_done && !last.pass_done) || now.devices > last.devices
}
