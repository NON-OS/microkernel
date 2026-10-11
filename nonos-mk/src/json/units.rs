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

use nonos_marketplace_abi::SettleMode;
use nonos_nox_config::UNITS;

use crate::error::CliError;

pub(crate) fn unit_word(label: &str) -> Result<[u8; 32], CliError> {
    UNITS
        .iter()
        .find(|(l, _)| *l == label)
        .map(|(_, w)| *w)
        .ok_or_else(|| CliError::UnknownEnum { field: "commerce.unit", value: label.into() })
}

/// The label, or the word in hex when this build does not know it.
pub(crate) fn unit_label(word: &[u8; 32]) -> String {
    match UNITS.iter().find(|(_, w)| w == word) {
        Some((l, _)) => (*l).into(),
        None => hex::encode(word),
    }
}

pub(crate) fn settle_mode(s: &str) -> Result<SettleMode, CliError> {
    match s {
        "free" => Ok(SettleMode::Free),
        "voucher" => Ok(SettleMode::Voucher),
        "escrow" => Ok(SettleMode::Escrow),
        other => Err(CliError::UnknownEnum { field: "commerce.settle_mode", value: other.into() }),
    }
}

pub(crate) fn settle_mode_str(m: SettleMode) -> &'static str {
    match m {
        SettleMode::Free => "free",
        SettleMode::Voucher => "voucher",
        SettleMode::Escrow => "escrow",
    }
}
