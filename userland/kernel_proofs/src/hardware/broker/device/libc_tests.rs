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

//! The record's BAR and device flags are libc's: the kernel sets them and a
//! driver reads them through libc, so a bit renumbered on one side alone
//! would have every driver misread the records it is given.

use super::flags::{BAR_FLAG_FIXED, BAR_FLAG_MEM64, BAR_FLAG_PREFETCH};
use super::flags::{DEVICE_FLAG_CLAIMED, DEVICE_FLAG_DEPS_UNMET, DEVICE_FLAG_DISABLED};
use super::flags::{DEVICE_FLAG_DMA_NONCOHERENT, DEVICE_FLAG_EXTERNAL};
use super::flags::{DEVICE_FLAG_IRQ_ACTIVE_LOW, DEVICE_FLAG_IRQ_EDGE};
use super::libc_types::{bar, device};

#[test]
fn every_bar_flag_is_libcs() {
    assert_eq!(BAR_FLAG_PREFETCH, bar::BAR_FLAG_PREFETCH);
    assert_eq!(BAR_FLAG_MEM64, bar::BAR_FLAG_MEM64);
    assert_eq!(BAR_FLAG_FIXED, bar::BAR_FLAG_FIXED);
}

#[test]
fn every_device_flag_is_libcs() {
    assert_eq!(DEVICE_FLAG_CLAIMED, device::DEVICE_FLAG_CLAIMED);
    assert_eq!(DEVICE_FLAG_DISABLED, device::DEVICE_FLAG_DISABLED);
    assert_eq!(DEVICE_FLAG_EXTERNAL, device::DEVICE_FLAG_EXTERNAL);
    assert_eq!(DEVICE_FLAG_DEPS_UNMET, device::DEVICE_FLAG_DEPS_UNMET);
    assert_eq!(DEVICE_FLAG_IRQ_EDGE, device::DEVICE_FLAG_IRQ_EDGE);
    assert_eq!(DEVICE_FLAG_IRQ_ACTIVE_LOW, device::DEVICE_FLAG_IRQ_ACTIVE_LOW);
    assert_eq!(DEVICE_FLAG_DMA_NONCOHERENT, device::DEVICE_FLAG_DMA_NONCOHERENT);
}
