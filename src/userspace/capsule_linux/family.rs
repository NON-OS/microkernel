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

//! A market listing names a Linux package as `linux.<tail>`. Alpine's are
//! bare, `linux.jq`; another distribution's sit under a namespace that is
//! its own, `linux.kali.jq` or `linux.blackarch.nmap`. The personality is
//! told the family in the prefix it reads, `deb:` or `pacman:`, and each
//! family installs into and runs from a tree of its own.

use alloc::string::String;

/// Listing namespace, and the prefix the personality knows the family by.
/// The catalogue refuses an Alpine name that begins with a namespace, so a
/// tail is never read as the wrong family.
const NAMESPACES: [(&str, &str); 2] = [("kali.", "deb:"), ("blackarch.", "pacman:")];

/// The name the personality is given for listing tail `tail`, or `None`
/// when a namespace names no package.
pub fn package_arg(tail: &str) -> Option<String> {
    for (space, prefix) in NAMESPACES {
        if let Some(pkg) = tail.strip_prefix(space) {
            return usable(pkg).then(|| alloc::format!("{prefix}{pkg}"));
        }
    }
    usable(tail).then(|| String::from(tail))
}

// A colon is the personality's family separator, so a tail carrying one
// could name a family its namespace does not. Listing ids have no colon;
// this does not rely on that.
fn usable(pkg: &str) -> bool {
    !pkg.is_empty() && !pkg.starts_with('.') && !pkg.contains(':')
}
