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

/*
 * A described device's DMA window and the three interface stops the broker
 * writes when its capsule lets go. The real files are included by path; the
 * stops run against a register file in host memory, which never completes a
 * reset, so the bounded wait's giving up is proven too.
 */

#[cfg(test)]
#[path = "../../../../src/hardware/broker/described/stop_regs.rs"]
mod stop_regs;
#[cfg(test)]
mod stop_tests;
#[cfg(test)]
mod tests;
#[path = "../../../../src/hardware/broker/described/window.rs"]
pub mod window;
