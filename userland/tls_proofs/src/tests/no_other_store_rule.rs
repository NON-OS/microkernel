// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! What counts as a store into a flight's key field, and that the rule
//! sees each kind of store and no read.

/// A write to a field named `private` or `p256_private`: an assignment (not
/// a comparison), a fill, a copy into it, or an index into it.
pub(super) fn stores(line: &str) -> bool {
    ["private", "p256_private"].iter().any(|f| {
        line.match_indices(&format!(".{f}")).any(|(at, m)| {
            let next = line[at + m.len()..].trim_start();
            let ident = next.starts_with(|c: char| c.is_alphanumeric() || c == '_');
            !ident
                && ((next.starts_with('=') && !next.starts_with("=="))
                    || [".fill(", ".copy_from_slice(", "[", ".as_mut"]
                        .iter()
                        .any(|s| next.starts_with(s)))
        })
    })
}

#[test]
fn the_rule_sees_each_kind_of_store_and_no_read() {
    for bad in [
        "self.cf.private = [0; 32];",
        "*cf.private = [0; 32];",
        "f.p256_private = Box::new([0; 32]);",
        "cf.private.fill(0);",
        "cf.p256_private.copy_from_slice(&k);",
        "cf.private[0] = 0;",
        "cf.private.as_mut().fill(0);",
    ] {
        assert!(stores(bad), "{bad}");
    }
    for good in [
        "cf.wipe_private();",
        "if cf.private == other {",
        "let k = &cf.private;",
        "self.private_key = k;",
        "generate(&mut flight.private)",
    ] {
        assert!(!stores(good), "{good}");
    }
}
