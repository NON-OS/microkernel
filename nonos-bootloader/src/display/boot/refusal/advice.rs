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

//! A refusal in words a person can act on. The loader's refusal sites pass a
//! short reason; this names what it means and what to do about it. It reads
//! the reason only: it never decides, retries or softens a refusal.

/// What the screen says about one refusal.
pub struct Advice {
    pub title: &'static [u8],
    pub what: &'static [u8],
    pub remedy: &'static [u8],
}

/// The advice for `reason`, matched on how the reason starts.
pub fn advice(reason: &[u8]) -> Advice {
    let all = super::kernel::KERNEL.iter().chain(super::platform::PLATFORM.iter());
    let all = all.chain(super::policy::POLICY.iter());
    for (prefix, a) in all {
        if starts_with_ignoring_case(reason, prefix) {
            return Advice { title: a.title, what: a.what, remedy: a.remedy };
        }
    }
    Advice {
        title: b"Boot stopped",
        what: b"The loader stopped before running anything it could not verify. The reason it gave is below.",
        remedy: b"Take a photo of this screen, with the reason, and report it. Restarting tries the same checks again.",
    }
}

fn starts_with_ignoring_case(s: &[u8], prefix: &[u8]) -> bool {
    s.len() >= prefix.len() && s[..prefix.len()].eq_ignore_ascii_case(prefix)
}
