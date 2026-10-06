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

use alloc::format;
use alloc::string::String;

use crate::browser::css::walk::Counters;

use super::numerals::{alpha, roman};

/* counter(name, style), or with `sep` counters(name, sep, style): the
 * innermost instance, or every instance outermost first joined by the
 * separator. A counter no element created reads 0. */
pub(super) fn counters(k: &Counters, name: &str, sep: Option<&str>, style: &str, out: &mut String) {
    let style = style.trim().to_ascii_lowercase();
    match sep {
        Some(sep) => {
            let mut first = true;
            for v in k.values(name) {
                if !first {
                    out.push_str(sep);
                }
                first = false;
                out.push_str(&format_one(v, &style));
            }
            if first {
                out.push_str(&format_one(0, &style));
            }
        }
        None => out.push_str(&format_one(k.values(name).last().unwrap_or(0), &style)),
    }
}

/* One value in a list style: decimal (the default for names this does
 * not know), decimal-leading-zero, alphabetic, roman, the bullets, or
 * none. */
fn format_one(v: i32, style: &str) -> String {
    match style {
        "none" => String::new(),
        "disc" => String::from("\u{2022}"),
        "circle" => String::from("\u{25e6}"),
        "square" => String::from("\u{25aa}"),
        "decimal-leading-zero" if (0..10).contains(&v) => format!("0{v}"),
        "lower-alpha" | "lower-latin" if v > 0 => alpha(v, b'a'),
        "upper-alpha" | "upper-latin" if v > 0 => alpha(v, b'A'),
        "lower-roman" if (1..4000).contains(&v) => roman(v).to_ascii_lowercase(),
        "upper-roman" if (1..4000).contains(&v) => roman(v),
        _ => format!("{v}"),
    }
}
