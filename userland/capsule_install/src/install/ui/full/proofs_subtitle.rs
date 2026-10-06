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

/*
 * The proofs screen's line names what the kernel actually checked. A
 * development kernel checks the kernel and every capsule on its Merkle path
 * alone, so saying a STARK proof was checked there would claim a proof that
 * never ran. Kept free of the installer's types so the host proofs can run
 * it as it is.
 */

pub fn proofs_subtitle(reported: bool, proof: bool, path_only: bool) -> &'static str {
    match (reported, proof, path_only) {
        (false, _, _) => "The kernel did not report what this boot checked.",
        (true, true, true) => "What this development boot checked: Merkle paths, no STARK proof.",
        (true, true, false) => "Every STARK proof this boot checked, and what each one proved.",
        (true, false, _) => "What this boot checked. No STARK proof is on record.",
    }
}
