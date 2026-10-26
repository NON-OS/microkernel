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

//! A kernel function as the pins read it: from `fn name(` through its
//! closing brace, each line's `//` comment dropped and every run of
//! whitespace made one space; and the only function of that name.

/// `name`'s whole text in `src`, flattened.
pub fn whole(src: &str, name: &str) -> String {
    let code: Vec<&str> = src.lines().map(|l| l.split("//").next().unwrap_or("")).collect();
    let flat = code.join(" ").split_whitespace().collect::<Vec<_>>().join(" ");
    let head = format!("fn {name}(");
    assert_eq!(flat.matches(&head).count(), 1, "`{name}` has a twin, or is gone");
    let at = flat.find(&head).unwrap_or_default();
    let open = at + flat[at..].find('{').expect("a body");
    let mut depth = 0i32;
    for (i, c) in flat[open..].char_indices() {
        depth += i32::from(c == '{') - i32::from(c == '}');
        if depth == 0 {
            return flat[at..=open + i].to_string();
        }
    }
    panic!("`{name}` never closes")
}

/// The test fails unless `name` in `src` reads as `pin`, joined by spaces.
pub fn pinned(src: &str, name: &str, pin: &[&str]) {
    assert_eq!(whole(src, name), pin.join(" "), "`{name}` is not as the model reads it");
}
