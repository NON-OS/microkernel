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

//! Each answer to a request, in words.

use super::asked::Asked;

impl Asked {
    pub fn label(self) -> &'static [u8] {
        match self {
            Asked::Queued => b"install requested",
            // The exec gate has the last word, after this window has asked.
            Asked::Opening => b"asked to start it; the system checks it first",
            // EBUSY: the queue is full, or holds this very request already.
            Asked::Busy => b"the system is busy with it or with others; ask again soon",
            Asked::Refused(errno) => super::refused::refused(errno),
            Asked::NotInstallable => b"nothing is selected",
            Asked::InImage => b"part of this system: start it from the dock",
            Asked::NotReady => b"this machine cannot install it yet: see its gates",
            Asked::Cannot => b"asking again cannot change this",
            Asked::Removing => b"uninstall requested",
            Asked::NotInstalled => b"it is not installed, so there is nothing to uninstall",
            Asked::Moving => b"it is still moving; uninstall it once it stops",
            Asked::InstallFirst => b"it is not installed: install it, then open it",
            Asked::QueuedDirect => {
                b"install requested, downloading direct: the mirror sees this machine's address"
            }
            Asked::NoDirect => b"no direct download to offer for this one",
            Asked::InWallet => b"confirm the purchase in your wallet",
            Asked::Order(state, plain) => super::order_line::order_line(state, plain),
            Asked::WalletWaiting => {
                b"another purchase waits in your wallet: confirm it or let it lapse"
            }
            Asked::NoMeter => b"the meter did not answer, so the wallet was not asked; try again",
        }
    }
}
