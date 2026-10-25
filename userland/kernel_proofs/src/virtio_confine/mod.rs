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

#[path = "../../../../src/hardware/broker/confine/virtio_rule.rs"]
pub mod virtio_rule;
#[path = "../../../../src/hardware/broker/confine/virtio_read.rs"]
pub mod virtio_read;
#[cfg(test)]
mod build;
#[cfg(test)]
mod read_tests;
#[cfg(test)]
mod tests;
