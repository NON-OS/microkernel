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

//! The address the station transmits from: drawn once a run, before the
//! first bind, and handed to every device bound in it, as the PCI drivers
//! draw theirs once a boot (capsule_driver_e1000/src/init/station_address.rs);
//! never the one in the adapter's EEPROM, efuse or descriptors. A factory
//! address names one piece of silicon to every network it joins. net.core
//! asks for the address only when its bound port changes and keeps the
//! first, so a device bound again after a loss must take the same one.

use nonos_mac::{apply, MAC_LEN};

/// A locally administered unicast address. `from_random` is the only way
/// to one, so a driver handed a `Station` cannot be handed a factory
/// address in its place. No `Debug`: the address is never formatted.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Station([u8; MAC_LEN]);

impl Station {
    pub fn from_random(mut bytes: [u8; MAC_LEN]) -> Self {
        apply(&mut bytes);
        Self(bytes)
    }

    pub fn bytes(self) -> [u8; MAC_LEN] {
        self.0
    }
}

/// How the device came to take frames sent to the drawn address. None of
/// these falls back to the device's own address.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Taken {
    /// Written into the device's address register, or set by its class
    /// request, so a device that filters takes frames sent to it.
    Programmed,
    /// The device has no way to take it, or refused it; its packet filter
    /// was opened to every frame instead.
    Promiscuous,
    /// The device could not take the address and refused the open filter:
    /// frames to the drawn address arrive only if it does not filter.
    Unconfirmed,
}

/// A station address from the kernel's CryptoRandom. `None` without
/// entropy, and no device is then bound: the address it carries is not a
/// fallback.
pub fn draw() -> Option<Station> {
    let mut bytes = [0u8; MAC_LEN];
    let rc = nonos_libc::crypto_random(bytes.as_mut_ptr(), MAC_LEN);
    (rc >= 0 && rc as usize == MAC_LEN).then(|| Station::from_random(bytes))
}
