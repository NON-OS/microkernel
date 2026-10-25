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

//! A certificate, field by field, for the operator to read before it is
//! handed to the machine.

use nsh_wire::Enrol;

use crate::hex::of;

pub fn describe(e: &Enrol<'_>) -> String {
    let mut out = String::new();
    let mut line = |k: &str, v: String| out.push_str(&format!("{k:<12} {v}\n"));
    line("fleet", of(e.fleet_id));
    line("host", String::from_utf8_lossy(e.host).into_owned());
    line("ak", of(e.ak_public));
    line("host keys", of(e.host_keys));
    for (i, p) in [0, 1, 2, 7].iter().zip(e.pcrs) {
        line(&format!("pcr {i}"), of(p));
    }
    for a in e.boot_applications() {
        line("boot app", of(a));
    }
    line("not after", format!("{} ms", e.not_after_ms));
    line("authority", of(e.authority));
    out
}

/// An unsigned body, read as a certificate would be, for the operator to
/// read before it is signed.
pub fn describe_body(body: &[u8]) -> Result<String, crate::Error> {
    let shaped = [body, &[0u8; nsh_wire::keys::HYBRID_SIG_LEN][..]].concat();
    let e = Enrol::parse(&shaped).map_err(|_| crate::Error::Value("certificate body"))?;
    Ok(describe(&e))
}
