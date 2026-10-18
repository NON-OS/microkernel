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

//! What a brightness key came to, as the toast says it: the level the
//! focused output's panel is at now, that no panel here takes a brightness,
//! or that the change did not go through. Pure, so the words are proven on
//! the host (desktop_proofs brightness_tests). A held key repeats as more
//! presses, and each notice replaces the last.

/// What the compositor answered for a brightness key.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lit {
    /// The panel is at this percent.
    Level(u8),
    /// No panel here takes a brightness.
    NoPanel,
    /// The compositor or the display did not take it.
    Failed,
}

/// The compositor's E_NOENT: no panel, or a driver that serves none.
const E_NOENT: i32 = -2;

const LEVEL: &[u8] = b"Brightness ";
const NO_PANEL: &[u8] = b"No brightness control on this display";
const FAILED: &[u8] = b"Brightness did not change";

/// The answer `status` and `level`, or None when none came.
pub fn heard(answer: Option<(i32, u32)>) -> Lit {
    match answer {
        Some((0, level)) => Lit::Level(level.min(100) as u8),
        Some((E_NOENT, _)) => Lit::NoPanel,
        _ => Lit::Failed,
    }
}

/// The notice for `lit` in `out`, which holds a toast's worth; the bytes.
pub fn notice(lit: Lit, out: &mut [u8]) -> usize {
    let mut n = 0;
    let mut put = |b: &[u8], n: &mut usize| {
        let take = b.len().min(out.len() - *n);
        out[*n..*n + take].copy_from_slice(&b[..take]);
        *n += take;
    };
    match lit {
        Lit::Level(v) => {
            put(LEVEL, &mut n);
            let digits = [b'0' + v / 100, b'0' + v / 10 % 10, b'0' + v % 10];
            put(&digits[(v < 100) as usize + (v < 10) as usize..], &mut n);
            put(b"%", &mut n);
        }
        Lit::NoPanel => put(NO_PANEL, &mut n),
        Lit::Failed => put(FAILED, &mut n),
    }
    n
}

/// Whether a toast's `text` is a brightness notice, which the next one
/// replaces rather than stacks under.
pub fn is_brightness_notice(text: &[u8]) -> bool {
    text.starts_with(LEVEL) || text == NO_PANEL || text == FAILED
}
