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

//! A Qwen tier's install asked to download its model over a direct
//! connection. The store asks it with the release word `@direct`, which no
//! market release is called, only after the person chose it for that tier:
//! a download over the Nym mixnet or the Anyone network of gigabytes can
//! take hours, and direct is faster but names this machine to the mirror.
//! It never changes which network anything else leaves by. Asking for an
//! install takes AppInstall, so the choice is as trusted as the install.
//! Pure, so model_fetch_proofs holds it.

/// The release word that asks for a direct download of a tier's model.
pub(super) const DIRECT: &str = "@direct";

/// The release to ask the market for, and whether a direct download was
/// asked: only for a Qwen tier, and only with the default release.
pub(super) fn split<'a>(listing: &str, release: &'a str) -> (&'a str, bool) {
    match release == DIRECT && listing.starts_with("linux.qwen-") {
        true => ("", true),
        false => (release, false),
    }
}
