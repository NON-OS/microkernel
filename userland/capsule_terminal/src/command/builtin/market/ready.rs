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

//! INSTALL_READY for the listing's default release, its bond read on
//! Sepolia, the one chain the contracts are on (v1.4.23).

use alloc::format;
use alloc::string::String;

use nonos_market_proto::{
    parse_readiness, ready_body, BondCode, Readiness, GATES, OP_INSTALL_READY,
};
use nonos_nox_config::SEPOLIA;

use super::call::call;
use super::failure::Failure;
use crate::command::output::Output;

pub(super) fn readiness(id: &[u8]) -> Result<Option<Readiness>, Failure> {
    let body = ready_body(id, b"", SEPOLIA).ok_or(Failure::Malformed)?;
    call(OP_INSTALL_READY, &body).map(|b| parse_readiness(&b))
}

/// The verdict, then each gate with its own mark (`market info`), and why
/// a failed bond failed (v1.4.35): "not checked" only while it is unread.
pub(super) fn gates(out: &mut Output<'_>, r: &Readiness) {
    let verdict = match r.install_ready {
        true => "ready to install on this machine",
        false => "this machine will not install it yet",
    };
    out.writeln(format!("  install    {verdict}").as_bytes());
    let last = GATES.len() - 1;
    for (i, (label, pass)) in GATES.iter().zip(r.gates.iter()).enumerate() {
        let mark = match (*pass, i == last && r.bond == BondCode::Unread) {
            (true, _) => "pass",
            (false, true) => "not checked",
            (false, false) => "fail",
        };
        let label = String::from_utf8_lossy(label);
        out.writeln(format!("    {label:<22} {mark}").as_bytes());
    }
    if let Some(why) = (!r.gates[last]).then(|| r.bond.sentence()).flatten() {
        out.writeln(format!("    {}", String::from_utf8_lossy(why)).as_bytes());
    }
}
