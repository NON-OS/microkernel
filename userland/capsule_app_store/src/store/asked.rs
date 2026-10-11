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

//! What the person is told after asking for something of a listing.

/// What the user is told after asking.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Asked {
    Queued,
    Opening,
    Busy,
    /// Refused outright, with the kernel's errno (`refused`).
    Refused(i64),
    NotInstallable,
    /// A NONOS capsule or community listing: part of this system's image,
    /// so there is nothing to install and the dock starts it.
    InImage,
    /// A Linux listing the market says this machine cannot install yet.
    NotReady,
    /// The last install stopped for a reason no retry changes.
    Cannot,
    /// An uninstall was queued.
    Removing,
    /// Only something installed can be uninstalled.
    NotInstalled,
    /// An install or uninstall of it is still moving.
    Moving,
    /// A package not installed has nothing to start.
    InstallFirst,
    /// A tier's install was queued, its model to come over a direct
    /// connection, as the person asked with `d`.
    QueuedDirect,
    /// `d` on a listing with no direct download to offer.
    NoDirect,
    /// A sold listing's names went to the wallet (op 19/0, the plan).
    InWallet,
    /// Another purchase still waits in the wallet (EBUSY).
    WalletWaiting,
    /// The meter, which carries the names, did not take them.
    NoMeter,
    /// The order's state and plain byte, from ORDER_STATUS (v1.4.42).
    Order(u8, bool),
}
