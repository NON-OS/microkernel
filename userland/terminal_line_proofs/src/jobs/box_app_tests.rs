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

//! A box's `[BOX-APP]` line names the policy its receipt is asked for with:
//! never drawn, taken only from the keyed report, and read only when it is
//! the one such line and exactly 256 bytes of hex.

use super::box_app::{keep, last, policy};
use super::box_hold::{BoxHold, KEY_LEN, TAG};

const KEY: [u8; KEY_LEN] = [4; KEY_LEN];
const BOX: &[u8] = b"[BOX] v=1 preset=strict refused=0 started=yes exit=0";

fn p() -> [u8; 256] {
    core::array::from_fn(|i| (i * 7) as u8)
}

fn app(p: &[u8; 256]) -> Vec<u8> {
    let hex: String = p.iter().map(|b| format!("{b:02x}")).collect();
    format!("[BOX-APP] v=1 policy={hex}").into_bytes()
}

fn held(lines: &[&[u8]]) -> BoxHold {
    let mut hold = BoxHold::new(KEY);
    let text: Vec<u8> = lines.iter().flat_map(|l| [*l, b"\n"].concat()).collect();
    assert!(hold.take(&[&[TAG][..], &KEY, &text].concat()));
    hold
}

#[test]
fn the_app_line_is_read_and_never_drawn() {
    let hold = held(&[b"BOX REPORT v1", &app(&p()), BOX]);
    assert_eq!(policy(&hold), Some(p()));
    assert_eq!(hold.lines(), (vec![&b"BOX REPORT v1"[..]], Some(BOX)));
}

#[test]
fn a_guest_printed_app_line_is_never_the_reports() {
    let (line, mut hold) = ([app(&p()), b"\n".to_vec()].concat(), BoxHold::new(KEY));
    assert!(!hold.take(&line));
    assert!(!hold.take(&[&[TAG][..], &[0; KEY_LEN], &line].concat()));
    assert_eq!(policy(&hold), None);
}

#[test]
fn two_lines_or_a_bent_one_name_no_policy() {
    let a = app(&p());
    let (n, upper) = (a.len(), [&a[..a.len() - 1], b"A"].concat());
    let v2 = [&b"[BOX-APP] v=2"[..], &a[b"[BOX-APP] v=1".len()..]].concat();
    for bent in [&a[..n - 2], &[&a[..], b"00"].concat(), &upper, &v2] {
        assert_eq!(policy(&held(&[bent, BOX])), None);
    }
    assert_eq!(policy(&held(&[&a, &a, BOX])), None);
}

#[test]
fn the_last_whole_report_is_kept_and_replaced() {
    keep(&held(&[&app(&p()), BOX]));
    assert_eq!(last(), Some(p()));
    keep(&held(&[BOX]));
    assert_eq!(last(), None);
}
