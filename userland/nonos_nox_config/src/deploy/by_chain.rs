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

//! Each chain's table. The local test chain's is compiled in only by a
//! `devnet` build (P5), which only a development image is.

use super::{mainnet, sepolia, Deployment};
use crate::chain::Chain;

pub const fn deployment(chain: Chain) -> &'static Deployment {
    match chain {
        Chain::Mainnet => &mainnet::MAINNET,
        Chain::Sepolia => &sepolia::SEPOLIA,
        #[cfg(feature = "devnet")]
        Chain::Devnet => &super::devnet::DEVNET,
    }
}
