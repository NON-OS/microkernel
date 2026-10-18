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

//! How many disks a driver should be serving and is not.

use crate::driver::Driver;

/// Controllers with no service behind them. An NVMe controller is one
/// disk, so each one past those served is one missing. The SATA driver
/// serves a disk from whichever of the machine's AHCI controllers holds
/// one, and a virtio-blk device can be the medium this boot came from, so
/// for those only a driver serving nothing is a missing disk. A card
/// reader serves no controller kind, so it is never counted missing.
pub(super) fn unserved(driver: Driver, on_bus: usize, served: u8) -> u8 {
    let missing = match driver {
        Driver::Nvme => on_bus.saturating_sub(served as usize),
        Driver::Ahci | Driver::VirtioBlk | Driver::Rtsx => usize::from(served == 0 && on_bus > 0),
    };
    missing.min(u8::MAX as usize) as u8
}
