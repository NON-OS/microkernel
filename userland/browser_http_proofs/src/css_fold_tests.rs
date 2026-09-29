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

//! Stylesheet arrivals: one relayout per page load, and a sheet past the
//! page cap cut after a whole rule instead of dropped.

use crate::browser::apply_css::css_cut::cut;
use crate::browser::apply_css::css_fold::{fold, sheet_done, MAX_PAGE_CSS};

/* A page load as the capsule runs it: css_pump takes the next URL off the
queue as it starts that fetch; when the fetch ends, a sheet's @imports
join the queue, then apply_css asks sheet_done. `fail` marks the sheets
whose fetch fails; `imports` adds that many imports from the first. */
fn relayouts(sheets: usize, fail: &[usize], imports: usize) -> (usize, String) {
    let mut queue: Vec<usize> = (0..sheets).collect();
    let (mut page_css, mut count, mut next) = (String::new(), 0, sheets);
    while !queue.is_empty() {
        let n = queue.remove(0);
        if n == 0 {
            queue.extend(next..next + imports);
            next += imports;
        }
        let text = format!(".s{n}{{color:red}}");
        let got = (!fail.contains(&n)).then_some(text.as_str());
        count += usize::from(sheet_done(&mut page_css, &queue, got));
    }
    (count, page_css)
}

#[test]
fn a_page_lays_out_once_whatever_its_sheets_do() {
    for sheets in 1..=16 {
        assert_eq!(relayouts(sheets, &[], 0).0, 1, "{sheets} sheets");
        assert_eq!(relayouts(sheets, &[sheets - 1], 0).0, 1, "last of {sheets} fails");
        assert_eq!(relayouts(sheets, &(0..sheets).collect::<Vec<_>>(), 0).0, 1, "all fail");
    }
    let (count, css) = relayouts(3, &[1], 4);
    assert_eq!(count, 1, "imports extend the hold");
    assert_eq!(css.matches("color").count(), 6, "every fetched sheet folded in");
}

#[test]
fn a_large_sheet_is_kept_and_an_oversized_one_is_cut_after_a_rule() {
    let rule = ".a{b:c}\n";
    let big = rule.repeat(3_560_000 / rule.len());
    let mut page = String::new();
    assert_eq!(fold(&mut page, &big), big.len(), "a 3.56 MB framework sheet fits");
    let huge = rule.repeat(MAX_PAGE_CSS / rule.len());
    let kept = fold(&mut page, &huge);
    assert!(kept > 0 && kept < huge.len() && huge[..kept].ends_with('}'));
    assert!(page.len() <= MAX_PAGE_CSS);
}

#[test]
fn the_cut_ignores_braces_in_comments_and_strings_and_nested_blocks() {
    let css = "a{x:y}/* } */b{c:\"}\"}@media x{c{d:e}}f{g:h}";
    assert_eq!(cut(css, 7), 6);
    assert_eq!(cut(css, 30), 21, "after b, not inside the comment or the string");
    assert_eq!(cut(css, 40), 37, "after the whole @media block");
    assert_eq!(cut("a{b:c", 5), 0, "no rule ends in the limit");
}
