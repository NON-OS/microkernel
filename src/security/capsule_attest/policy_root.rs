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
 * A static, read through black_box, so the root stays one contiguous run in
 * .rodata where the build receipt finds it. A constant copy was folded into
 * four instruction immediates and never appeared as 32 bytes. A root file of
 * any other length now fails the build instead of refusing every capsule.
 */
static ROOT: [u8; 32] =
    *include_bytes!("../../../nonos-data/trust/policy/zk_capsule_policy_root.bin");

pub(super) fn root() -> Option<[u8; 32]> {
    Some(*core::hint::black_box(&ROOT))
}
