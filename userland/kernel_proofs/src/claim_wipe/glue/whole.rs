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


//! A kernel function pinned whole, its code as `flat` gives it (L80): from
//! its head to its closing brace, and the only function of its name in the
//! file. So a body changed anywhere, moved about inside its own text, copied
//! into a string, or shut out by an attribute for a twin built in its place
//! is seen; and a constant read from source is the only item of its name.

/// The functions named `name` in `text`, whatever their visibility.
pub(super) fn fns(text: &str, name: &str) -> usize {
    let ident = |c: char| c.is_alphanumeric() || c == '_';
    let named = |i: usize| {
        let rest = text[i + 3..].trim_start();
        let rest = rest.strip_prefix("r#").unwrap_or(rest);
        rest.strip_prefix(name).is_some_and(|r| !r.starts_with(ident))
    };
    text.match_indices("fn ").filter(|&(i, _)| !text[..i].ends_with(ident) && named(i)).count()
}

/// Whether `text` holds the function `pin` spells, through its closing
/// brace, and no other function of its name.
pub(super) fn pinned(text: &str, name: &str, pin: &[&str]) -> Result<(), String> {
    let pin = pin.join(" ");
    let head = pin.find(&format!("fn {name}(")).ok_or("the pin has no head")?;
    let head = &pin[..head + name.len() + 4];
    let body = &pin[pin.find('{').ok_or("the pin has no body")?..];
    let mut depth = 0i32;
    let shut = body.char_indices().filter(|&(_, c)| {
        depth += i32::from(c == '{') - i32::from(c == '}');
        depth == 0
    });
    if !shut.map(|(i, _)| i + 1).eq([body.len()]) {
        return Err(format!("the pin of `{name}` is not one whole function"));
    }
    if fns(text, name) != 1 {
        return Err(format!("`{name}` has a twin, or is gone"));
    }
    let at = text.find(head).ok_or(format!("missing `{head}`"))?;
    text[at..].starts_with(&pin).then_some(()).ok_or(format!("`{name}` is not as pinned"))
}

/// `pinned`, or the test fails saying why.
pub(super) fn whole(text: &str, name: &str, pin: &[&str]) {
    pinned(text, name, pin).unwrap_or_else(|why| panic!("{why}"));
}

/// The number the constant `name` is set to, the only item of that name in
/// `text`, which names it `uses` times in all.
pub(super) fn num<T: core::str::FromStr>(text: &str, name: &str, uses: usize) -> T {
    let ident = |c: char| c.is_alphanumeric() || c == '_';
    let alone = |&(i, _): &(usize, &str)| {
        !text[..i].ends_with(ident) && !text[i + name.len()..].starts_with(ident)
    };
    assert_eq!(text.match_indices(name).filter(alone).count(), uses, "`{name}` named again");
    let def = format!("const {name}: ");
    let v = &text[text.find(&def).unwrap_or_else(|| panic!("no `{def}`"))..];
    let v = &v[v.find("= ").expect("a value") + 2..v.find(';').expect("an end")];
    v.parse().unwrap_or_else(|_| panic!("{name} not a number"))
}
