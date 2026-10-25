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

//! Live NOX token and staking reads against the pinned mainnet deployment.

mod apr_bps;
mod calldata_addr;
pub mod constants;
mod format_apr;
mod format_nox;
mod nox_text;
mod q32_to_u128;
mod stakeable;
mod staking;
mod status;

pub use apr_bps::apr_bps;
pub use calldata_addr::calldata_addr;
pub use format_apr::format_apr;
pub use format_nox::format_nox;
pub use nox_text::nox_text;
pub use q32_to_u128::q32_to_u128;
pub use stakeable::{held_wei, NOX_DECIMALS};
pub use staking::{lock_days, LOCK_TERMS, SEL_GET_STAKE_INFO};
pub use status::NoxStatus;
