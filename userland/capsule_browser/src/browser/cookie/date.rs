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

//! The Expires date, read the forgiving way RFC 6265 section 5.1.1 asks.

/*
 * Servers write every date format there has ever been: the RFC 1123 one,
 * the RFC 850 one with a two-digit year and dashes, asctime's. The RFC's
 * algorithm takes tokens split on any delimiter and keeps the first that
 * looks like a time, a day, a month and a year, in whatever order they
 * come, so one reader covers all of them.
 */
/// An Expires value as Unix seconds, or `None` if it is not a date.
pub fn cookie_date(text: &str) -> Option<i64> {
    let (mut time, mut day, mut month, mut year) = (None, None, None, None);
    for token in text.split(is_delimiter).filter(|t| !t.is_empty()) {
        if time.is_none() {
            if let Some(t) = hms(token) {
                time = Some(t);
                continue;
            }
        }
        if day.is_none() {
            if let Some(d) = digits(token, 1, 2) {
                day = Some(d as u32);
                continue;
            }
        }
        if month.is_none() {
            if let Some(m) = month_of(token) {
                month = Some(m);
                continue;
            }
        }
        if year.is_none() {
            if let Some(y) = digits(token, 2, 4) {
                year = Some(y);
            }
        }
    }
    let (h, m, s) = time?;
    let mut year = year? as i64;
    if (70..=99).contains(&year) {
        year += 1900;
    } else if year <= 69 {
        year += 2000;
    }
    if year < 1601 {
        return None;
    }
    super::civil::unix(year, month?, day?, h, m, s)
}

/* %x09 / %x20-2F / %x3B-40 / %x5B-60 / %x7B-7E, RFC 6265 section 5.1.1. */
fn is_delimiter(c: char) -> bool {
    matches!(c, '\t' | ' '..='/' | ';'..='@' | '['..='`' | '{'..='~')
}

/// One to `max` leading digits of `token`, at least `min` of them, with
/// anything after them that is not a digit ignored as the RFC allows.
fn digits(token: &str, min: usize, max: usize) -> Option<u64> {
    let n = token.bytes().take_while(u8::is_ascii_digit).count();
    if n < min || n > max {
        return None;
    }
    token[..n].parse().ok()
}

fn hms(token: &str) -> Option<(u32, u32, u32)> {
    let mut parts = token.split(':');
    let h = digits(parts.next()?, 1, 2)? as u32;
    let m = digits(parts.next()?, 1, 2)? as u32;
    let s = digits(parts.next()?, 1, 2)? as u32;
    if parts.next().is_some() {
        return None;
    }
    Some((h, m, s))
}

fn month_of(token: &str) -> Option<u32> {
    const NAMES: [&str; 12] =
        ["jan", "feb", "mar", "apr", "may", "jun", "jul", "aug", "sep", "oct", "nov", "dec"];
    let head = token.get(..3)?;
    NAMES.iter().position(|n| head.eq_ignore_ascii_case(n)).map(|i| i as u32 + 1)
}
