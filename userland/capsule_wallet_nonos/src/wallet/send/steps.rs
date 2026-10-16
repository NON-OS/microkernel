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
//! A public payment from this account: ETH, NOX or USDC, on the network

//! The payment's steps and the assets it moves, as the screens name them.

/// Which screen of the payment is up.
pub const STAGE_FORM: u8 = 0;
pub const STAGE_REVIEW: u8 = 1;
pub const STAGE_SENT: u8 = 2;

/// The assets a payment can move, in the order the form offers them.
pub const ASSET_ETH: u8 = 0;
pub const ASSET_NOX: u8 = 1;
pub const ASSET_USDC: u8 = 2;
