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
 * The typed parts of a private payment, checked before the service is
 * asked: a whole nox1 address and a positive decimal amount, which amount
 * an action reads, and what a paste brings. Pure, so the wallet proofs run
 * the same functions on the host.
 */

/* nox1 then base32 (a to z, 2 to 7) of the 1,253 address bytes: 2,009
 * letters. The checksum inside is checked by the service. */
const ADDR_LEN: usize = 2009;

pub fn address_ok(to: &str) -> bool {
    let Some(rest) = to.strip_prefix("nox1") else { return false };
    to.len() == ADDR_LEN
        && rest.bytes().all(|b| b.is_ascii_lowercase() || (b'2'..=b'7').contains(&b))
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

/* Whether an action reads the typed amount: on the send screen, and on a
 * review that a send opened. Anywhere else it reads the picked size; the
 * screen a review came from says nothing once another screen is up. */
pub fn reads_typed(screen: u8, from: u8, send: u8, review: u8) -> bool {
    screen == send || (screen == review && from == send)
}

/* The address a paste carries, if it carries one: trimmed, with a `nox:`
 * scheme taken off, as a link or a QR code writes it. */
pub fn pasted_address(text: &str) -> Option<&str> {
    let text = text.trim();
    let bare = match text.get(..4) {
        Some(head) if head.eq_ignore_ascii_case("nox:") => text.get(4..).unwrap_or(""),
        _ => text,
    };
    let bare = bare.trim_start_matches('/');
    bare.get(..4).filter(|h| h.eq_ignore_ascii_case("nox1")).map(|_| bare)
}

/* Where the shown tail of a field's text starts: the first character from
 * which the rest fits in `room`. `width` is the measure of a tail and
 * shrinks as the tail does, so a halving search asks it about a dozen
 * times for a 2,009-letter address, not once per letter dropped. */
pub fn tail_start(text: &str, room: i32, width: impl Fn(&str) -> i32) -> usize {
    if width(text) <= room {
        return 0;
    }
    let starts: alloc::vec::Vec<usize> = text.char_indices().map(|(i, _)| i).collect();
    let (mut lo, mut hi) = (0usize, starts.len());
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        let at = starts.get(mid).copied().unwrap_or(text.len());
        if width(text.get(at..).unwrap_or("")) <= room {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    starts.get(lo).copied().unwrap_or(text.len())
}
