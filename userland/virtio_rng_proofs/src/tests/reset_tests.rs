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

//! Stopping the device: zero written to the status, and read back, on
//! either transport, before the claim is given back.

use nonos_devmodel::FakeBar;
use nonos_virtio::common::DEVICE_STATUS;

use super::model::{device, regs, view};
use crate::constants::LEG_STATUS;
use crate::transport::{Modern, Transport};

#[test]
fn a_legacy_reset_writes_zero_and_reads_it_back() {
    let bar = device(8);
    bar.present8(LEG_STATUS, 0x0F);
    assert!(Transport::Legacy(regs(&bar)).reset());
    assert_eq!(bar.wrote8(LEG_STATUS), 0);
}

#[test]
fn a_modern_reset_writes_zero_to_the_common_configuration() {
    let (common, notify) = (FakeBar::new(0x40), FakeBar::new(0x40));
    common.present8(DEVICE_STATUS, 0x0F);
    let modern = Modern { common: view(&common), notify: view(&notify), doorbell: 0 };
    assert!(Transport::Modern(modern).reset());
    assert_eq!(common.wrote8(DEVICE_STATUS), 0);
}
