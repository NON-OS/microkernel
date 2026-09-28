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

/*
 * What a private payment needs before it can be reviewed: a nox1 address
 * made of the bech32 alphabet, a positive amount with at most 18 decimals,
 * and either a note that has waited long enough or the typed override.
 */

use super::consts::OVERRIDE;
use crate::wallet::state::shield_ui::ShieldUi;

const BECH32: &[u8] = b"qpzry9x8gf2tvdw0s3jn54khce6mua7l";
/* Short enough to reject a prefix typed by mistake, long before any key. */
const MIN_ADDR: usize = 12;

pub fn address_ok(to: &str) -> bool {
    let Some(rest) = to.strip_prefix("nox1") else { return false };
    to.len() >= MIN_ADDR && rest.bytes().all(|b| BECH32.contains(&b))
}

pub fn amount_ok(text: &str) -> bool {
    let mut parts = text.splitn(2, '.');
    let whole = parts.next().unwrap_or("");
    let frac = parts.next().unwrap_or("");
    let digits = |s: &str| s.bytes().all(|b| b.is_ascii_digit());
    if (whole.is_empty() && frac.is_empty()) || !digits(whole) || !digits(frac) || frac.len() > 18 {
        return false;
    }
    whole.bytes().chain(frac.bytes()).any(|b| b != b'0')
}

pub fn wait_ok(ui: &ShieldUi) -> bool {
    super::meter::waited(ui) || ui.override_text == OVERRIDE
}

pub fn send_ready(ui: &ShieldUi) -> bool {
    address_ok(&ui.to) && amount_ok(&ui.amount) && wait_ok(ui)
}
