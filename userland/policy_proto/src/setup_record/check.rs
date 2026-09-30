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
 * Reading a kept record back, and naming what is wrong with one refused.
 */

use super::answers::Answers;
use super::kept::Kept;
use super::layout::{ANSWERS_LEN, ANSWERS_V1_LEN, ANSWERS_V2_LEN, APPS_AT, NAME_AT, TIER_AT};
use super::layout::{MAGIC_V1, MAGIC_V2, MAGIC_V3};
use super::record::Record;
use super::refused::Refused;
use super::rules::{name_ok, tier_ok};

/* The answers in `raw`, of any version, or why they were refused. */
pub fn check(raw: &[u8]) -> Result<Answers, Refused> {
    check_record(raw).map(|r| r.answers)
}

/* The whole record in `raw`: every app on in version 1 and 2. */
pub fn check_record(raw: &[u8]) -> Result<Record, Refused> {
    let version = version(raw)?;
    let timezone = raw[5] as i8;
    if !(-12..=14).contains(&timezone) {
        return Err(Refused::Timezone);
    }
    let (username, qwen_tier) = match version {
        1 => (Kept::EMPTY, Kept::EMPTY),
        _ => (
            Kept::take(&raw[NAME_AT..TIER_AT], name_ok).ok_or(Refused::Name)?,
            Kept::take(&raw[TIER_AT..ANSWERS_V2_LEN], tier_ok).ok_or(Refused::Tier)?,
        ),
    };
    let answers =
        Answers { keyboard_layout: raw[4], timezone, wallpaper: raw[6], username, qwen_tier };
    let apps_off = if version == 3 { raw[APPS_AT] } else { 0 };
    Ok(Record { answers, apps_off })
}

/* Each version has its own length and its own magic. */
fn version(raw: &[u8]) -> Result<u8, Refused> {
    match raw.len() {
        ANSWERS_V1_LEN if raw[..4] == MAGIC_V1 => Ok(1),
        ANSWERS_V2_LEN if raw[..4] == MAGIC_V2 => Ok(2),
        ANSWERS_LEN if raw[..4] == MAGIC_V3 => Ok(3),
        ANSWERS_V1_LEN | ANSWERS_V2_LEN | ANSWERS_LEN => Err(Refused::Magic),
        _ => Err(Refused::Length),
    }
}
