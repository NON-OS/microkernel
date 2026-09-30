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
use super::layout::{ANSWERS_LEN, ANSWERS_V1_LEN, MAGIC_V1, MAGIC_V2, NAME_AT, TIER_AT};
use super::rules::{name_ok, tier_ok};

/* Why a record was refused, so a log can say which part was wrong. */
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Refused {
    /* Neither version's length. */
    Length,
    /* A length that fits, without that version's magic: zeros included. */
    Magic,
    /* A time zone setup does not offer. */
    Timezone,
    /* A name setup's name step would not take, or bytes past its end. */
    Name,
    /* A tier that is no tier's name, or bytes past its end. */
    Tier,
}

impl Refused {
    pub fn name(self) -> &'static str {
        match self {
            Refused::Length => "length",
            Refused::Magic => "magic",
            Refused::Timezone => "time zone",
            Refused::Name => "name",
            Refused::Tier => "qwen tier",
        }
    }
}

/* The answers in `raw`, of either version, or why they were refused. */
pub fn check(raw: &[u8]) -> Result<Answers, Refused> {
    let v2 = match raw.len() {
        ANSWERS_V1_LEN if raw[..4] == MAGIC_V1 => false,
        ANSWERS_LEN if raw[..4] == MAGIC_V2 => true,
        ANSWERS_V1_LEN | ANSWERS_LEN => return Err(Refused::Magic),
        _ => return Err(Refused::Length),
    };
    let timezone = raw[5] as i8;
    if !(-12..=14).contains(&timezone) {
        return Err(Refused::Timezone);
    }
    let (username, qwen_tier) = match v2 {
        false => (Kept::EMPTY, Kept::EMPTY),
        true => (
            Kept::take(&raw[NAME_AT..TIER_AT], name_ok).ok_or(Refused::Name)?,
            Kept::take(&raw[TIER_AT..], tier_ok).ok_or(Refused::Tier)?,
        ),
    };
    Ok(Answers { keyboard_layout: raw[4], timezone, wallpaper: raw[6], username, qwen_tier })
}
