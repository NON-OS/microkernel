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

//! Whether an entry's PriceModel, which the store shows, agrees with its
//! Commerce block, which the meter charges (plan/25, B6). Commerce is
//! authoritative; a listing whose shown price says otherwise is refused, so
//! nobody is shown one price and charged another.
//!
//! - Free commerce: a Free price of zero.
//! - Voucher: unit_price shown as the amount; OneTime with no period for the
//!   install unit, UsageMetered for any other unit.
//! - Escrow: OneTime, unit_price shown, no period.
//! - A Subscription agrees with no commerce block in v1.

use nonos_nox_config::UNIT_INSTALL;

use crate::types::{MarketplaceEntry, PriceKind, SettleMode};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PriceVerdict {
    /// A schema 2 entry, which carries no commerce block.
    NotV3,
    Agrees,
    Contradicts,
}

pub fn price_verdict(entry: &MarketplaceEntry) -> PriceVerdict {
    let Some(c) = entry.commerce.as_ref() else {
        return PriceVerdict::NotV3;
    };
    let p = &entry.price;
    let agrees = match c.settle_mode {
        SettleMode::Free => p.kind == PriceKind::Free && p.amount_atomic == 0,
        SettleMode::Voucher => {
            let kind = match c.unit == UNIT_INSTALL {
                true => PriceKind::OneTime,
                false => PriceKind::UsageMetered,
            };
            p.kind == kind
                && p.amount_atomic == c.unit_price
                && (kind != PriceKind::OneTime || p.period_seconds == 0)
        }
        SettleMode::Escrow => {
            p.kind == PriceKind::OneTime && p.amount_atomic == c.unit_price && p.period_seconds == 0
        }
    };
    if agrees {
        PriceVerdict::Agrees
    } else {
        PriceVerdict::Contradicts
    }
}
