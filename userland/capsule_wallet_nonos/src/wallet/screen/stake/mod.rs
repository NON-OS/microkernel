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
//! 07  STAKE NOX, on Ethereum mainnet: lock NOX into the staking contract
//! for a share of its emission, or close a position. Staking is two
//! transactions, the allowance and then the stake, and the button says
//! which one it signs next. Sepolia has no staking contract, so there the
//! screen says so and signs nothing.

pub mod click;
mod confirm;
mod facts;
mod form;
mod key;
mod review;
mod show;
mod text;
mod why;

pub use show::show;
pub use text::nox_amount;
