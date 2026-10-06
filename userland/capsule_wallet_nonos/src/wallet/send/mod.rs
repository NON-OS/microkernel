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
//! picked now. Three steps, each its own screen: the form, a review of
//! exactly what will be signed, and what the network said once it went.
//! Nothing is signed before the review, and the review is built from fresh
//! reads of the nonce, the fee and the gas the node says it takes.

mod confirm;
mod draft;
pub mod exact;
pub mod fees;
pub mod gas;
pub mod outcome;
pub mod sent;
pub mod stake;
pub mod text;

pub use confirm::{follow, held_back, sign, sign_draft};
pub use draft::{finish, held, plan, Draft, Plan};
pub use text::{amount_text, asset_name, decimals, fee_text};

/// Which screen of the payment is up.
pub const STAGE_FORM: u8 = 0;
pub const STAGE_REVIEW: u8 = 1;
pub const STAGE_SENT: u8 = 2;

/// The assets a payment can move, in the order the form offers them.
pub const ASSET_ETH: u8 = 0;
pub const ASSET_NOX: u8 = 1;
pub const ASSET_USDC: u8 = 2;
