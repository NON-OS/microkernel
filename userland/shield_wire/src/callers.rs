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

//! Who may call the shield: the wallet window's service names alone. Every
//! op answers DENIED to any other sender, SIGN_FROM_FRESH included
//! (v1.4.51).

/// The wallet window's service names.
pub const WALLETS: [&[u8]; 3] = [b"app.nonos_wallet", b"app.nonos_wallet.1", b"app.nonos_wallet.2"];

/// Whether `sender` is a wallet window, given the pid owning each name.
/// Pid zero is no sender at all.
pub fn from_wallet(sender: u32, pid_of: impl Fn(&[u8]) -> Option<u32>) -> bool {
    sender != 0 && WALLETS.iter().any(|name| pid_of(name) == Some(sender))
}
