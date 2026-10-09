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
//! How the payment screens write amounts.

use alloc::format;
use alloc::string::String;

use super::{ASSET_ETH, ASSET_NOX};
use crate::wallet::num::Amount;

pub fn asset_name(asset: u8) -> &'static str {
    match asset {
        ASSET_ETH => "ETH",
        ASSET_NOX => "NOX",
        _ => "USDC",
    }
}

/// The precision each asset is counted in: USDC counts to six places.
pub fn decimals(asset: u8) -> u32 {
    match asset {
        ASSET_ETH => crate::wallet::units::ETH_DECIMALS,
        ASSET_NOX => crate::wallet::nox::NOX_DECIMALS,
        _ => 6,
    }
}

/// The figure as it was typed, point and all.
pub fn amount_text(a: &Amount) -> String {
    if !a.typed_anything() && a.is_zero() {
        return String::new();
    }
    let digits = format!("{}", a.raw());
    let places = a.places() as usize;
    if !a.point_started() {
        return digits;
    }
    let padded = format!("{:0>width$}", digits, width = places + 1);
    let (whole, frac) = padded.split_at(padded.len() - places);
    format!("{whole}.{frac}")
}

/// A fee in wei as ETH, every digit of it: a maximum is never shown less.
pub fn fee_text(wei: u128) -> String {
    format!("{} ETH", super::exact::exact_text(wei, 18))
}
