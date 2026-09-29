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

/* The scheme of an absolute address: a name before a ':' that comes ahead
 * of any '/', '?' or '#', unless a port number follows it (host:8080). */
pub fn scheme(s: &str, stop: usize) -> Option<&str> {
    let colon = s[..stop].find(':')?;
    let (name, rest) = (&s[..colon], &s[colon + 1..]);
    let ok = |c: char| c.is_ascii_alphanumeric() || matches!(c, '+' | '-' | '.');
    if !name.starts_with(|c: char| c.is_ascii_alphabetic()) || !name.chars().all(ok) {
        return None;
    }
    let digits = rest.bytes().take_while(u8::is_ascii_digit).count();
    let port = digits > 0 && matches!(rest.as_bytes().get(digits), None | Some(b'/' | b'?' | b'#'));
    (!port).then_some(name)
}
