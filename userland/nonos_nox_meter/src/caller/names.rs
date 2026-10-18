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

//! The registry names behind each role (plan/21, plan/29).

/// The wallet's endpoints: its first window and the later ones.
pub const WALLET: [&[u8]; 3] = [b"app.nonos_wallet", b"app.nonos_wallet.1", b"app.nonos_wallet.2"];

/// Who may start and follow an escrow order (plan/21, v1.3 E14): the
/// wallet and the store. ORDER_REFUND is the wallet's alone.
pub const BUYERS: [&[u8]; 4] =
    [b"app.store", b"app.nonos_wallet", b"app.nonos_wallet.1", b"app.nonos_wallet.2"];

/// capsule_nox_settle, the only reader of signed vouchers.
pub const SETTLE: [&[u8]; 1] = [b"nox_settle"];

/// Who may ask for a charge: the installer and the app store, for paid
/// installs after the person agreed, and the wallet. Nothing else: Linux
/// guests do not pay through capsule_linux, which hosts untrusted code.
pub const PAYERS: [&[u8]; 5] =
    [b"installer", b"app.store", b"app.nonos_wallet", b"app.nonos_wallet.1", b"app.nonos_wallet.2"];

/// Who may hand an order its K (ORDER_DELIVERED, plan/21): the wallet, and
/// the installer that fetched the release.
pub const DELIVERERS: [&[u8]; 4] =
    [b"installer", b"app.nonos_wallet", b"app.nonos_wallet.1", b"app.nonos_wallet.2"];

/// The install role alone takes an order's plaintext (op 18, plan/29).
pub const INSTALL: [&[u8]; 1] = [b"app.linux.install"];

/// The store alone posts a purchase's names (op 19/0, plan/27) and asks
/// whether a listing's channel takes one more install (op 21, FUNDED).
pub const STORE: [&[u8]; 1] = [b"app.store"];
