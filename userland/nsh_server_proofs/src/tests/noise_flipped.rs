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

//! The fuzz target's oracle run here on fixed inputs, so every test run
//! holds the server to it: noise at each stage, every kind byte with bodies
//! of each length near its rules' edges, and good frames with one bit
//! flipped or cut short.

use super::noise::*;
use crate::kit::noise::drive;
use crate::kit::raw_run::RawRun;
use crate::kit::roles_build::grant::GrantB;
use crate::kit::roles_build::keys::K;
use nsh_wire::Kind;

#[test]
fn a_good_user_auth_flipped_or_cut_admits_nobody() {
    let r = RawRun::proved();
    let ua = [&[Kind::UserAuth.byte()][..], &r.raw.user_auth(&GrantB::default(), K::User)].concat();
    for at in (1..ua.len()).step_by(997).chain([1, 33, 2048, ua.len() - 1]) {
        let mut flipped = ua.clone();
        flipped[at] ^= 0x10;
        drive(&framed(1, &[flipped]));
        drive(&framed(1, &[ua[..at].to_vec()]));
    }
}
