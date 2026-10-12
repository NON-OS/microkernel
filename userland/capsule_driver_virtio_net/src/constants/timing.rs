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

/// How long a device may take to read back zero after a reset. QEMU is done
/// at once; real virtio hardware (vDPA, a SmartNIC) may take longer.
pub const RESET_MS: u64 = 1_000;

/// How long the transmit ring may stay full, with the link up and nothing
/// completed, before the device is taken to have stopped and is reset.
pub const TX_STALL_MS: u64 = 2_000;
