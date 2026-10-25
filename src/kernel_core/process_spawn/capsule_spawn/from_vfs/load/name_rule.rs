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

//! Which names a capsule loaded at run time may declare, pure.
//! A reply inbox `endpoint.<x>.reply` stands for `<x>`. The kernel's own
//! inboxes and `proc.` are nobody's to load. A name the image reserves, or
//! one under it, is its spawn site's namespace's alone, and a family
//! member's is `systems.nonos.<member>`'s. A publisher's capsule declares
//! only names under its own namespace. kernel_proofs::name_rule.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum NameRefusal {
    Reserved,
    OutsideNamespace,
}

fn stem(name: &str, reply: bool) -> Result<&str, NameRefusal> {
    let stem = match reply {
        false if name.starts_with("endpoint.") || name.starts_with("proc.") => "",
        false => name,
        true => {
            name.strip_prefix("endpoint.").map_or("", |s| s.strip_suffix(".reply").unwrap_or(s))
        }
    };
    let kernel_inbox = stem.is_empty() || stem.bytes().all(|b| b.is_ascii_digit());
    if kernel_inbox {
        Err(NameRefusal::Reserved)
    } else {
        Ok(stem)
    }
}

fn under(name: &str, owner: &str) -> bool {
    name == owner || name.strip_prefix(owner).is_some_and(|rest| rest.starts_with('.'))
}

/// Whether a loaded capsule signed under `namespace` may declare `name`.
pub(crate) fn admit_loaded(
    name: &str,
    reply: bool,
    namespace: &str,
    publisher: bool,
    baked: &[&[(&str, &str)]],
    families: &[&str],
) -> Result<(), NameRefusal> {
    let stem = stem(name, reply)?;
    let owner = baked.iter().flat_map(|set| set.iter()).find(|(n, _)| under(stem, n));
    let own = |ns: &str| !publisher && !ns.is_empty() && namespace == ns;
    if let Some(&(_, ns)) = owner {
        return if own(ns) { Ok(()) } else { Err(NameRefusal::Reserved) };
    }
    if families.iter().any(|f| under(stem, f)) {
        let member = namespace.strip_prefix("systems.nonos.") == Some(stem);
        return if !publisher && member { Ok(()) } else { Err(NameRefusal::Reserved) };
    }
    if publisher && !under(stem, namespace) {
        return Err(NameRefusal::OutsideNamespace);
    }
    Ok(())
}
