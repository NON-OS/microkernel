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

//! The wallet's last word, drawn above the status line on every screen.
//!
//! What a press came to (an account opened, an address copied, a payment
//! sent or refused) is the wallet's status, and the status line only names
//! the network and how the wallet is kept. The notice row says the status
//! itself, so nothing the wallet says goes unseen. Set once a paint.

use spin::Mutex;

static NOTICE: Mutex<&'static str> = Mutex::new("");

/// Take the wallet's status for this paint.
pub fn set(text: &'static str) {
    *NOTICE.lock() = text;
}

/// The status this paint shows, empty for none.
pub fn now() -> &'static str {
    *NOTICE.lock()
}
