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

/* HTML's valid e-mail address: atext and dots, '@', then dot-separated
 * labels of letters, digits and inner hyphens, none longer than 63. */
pub(super) fn email(v: &str) -> bool {
    let Some((local, domain)) = v.split_once('@') else { return false };
    let atext = |c: char| c.is_ascii_alphanumeric() || ".!#$%&'*+/=?^_`{|}~-".contains(c);
    let label = |l: &str| {
        !l.is_empty()
            && l.len() <= 63
            && !l.starts_with('-')
            && !l.ends_with('-')
            && l.chars().all(|c| c.is_ascii_alphanumeric() || c == '-')
    };
    !local.is_empty() && local.chars().all(atext) && domain.split('.').all(label)
}

/* An absolute URL: a scheme (a letter, then letters, digits, '+', '-' or
 * '.') and a colon, and no whitespace. */
pub(super) fn url(v: &str) -> bool {
    let Some((scheme, _)) = v.split_once(':') else { return false };
    let mut c = scheme.chars();
    c.next().is_some_and(|f| f.is_ascii_alphabetic())
        && c.all(|x| x.is_ascii_alphanumeric() || "+-.".contains(x))
        && !v.chars().any(char::is_whitespace)
}
