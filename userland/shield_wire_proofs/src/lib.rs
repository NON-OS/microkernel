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

//! The shield's wire, wallet job to service and back, on the host.

extern crate alloc;

/// The service's reply builders, as it ships them.
#[path = "../../capsule_shield/src/reply.rs"]
pub mod service_reply;

/// The wallet's history types, as it ships them.
#[path = "../../capsule_wallet_nonos/src/wallet/state/shield_log.rs"]
pub mod shield_log;

/// The wallet's module shape for its readers.
pub mod wallet {
    pub mod state {
        pub use crate::shield_log;
    }
}

/// The wallet's readers, as it ships them.
#[path = "../../capsule_wallet_nonos/src/wallet/shield/reply.rs"]
pub mod wallet_reply;

/// How the wallet names the spends the service keeps, as it ships it.
#[path = "../../capsule_wallet_nonos/src/wallet/shield/kept_names.rs"]
pub mod kept_names;

pub mod pool;

#[cfg(test)]
mod tests;
