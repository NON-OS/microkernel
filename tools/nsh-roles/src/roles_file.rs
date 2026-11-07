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

//! A fleet's roles, as an operator writes them: one role a line,
//!
//! ```text
//! name digest|text approvals max-session-hours ceiling,... escalation,...|-
//! ```
//!
//! `#` starts a comment. With no roles file the table carries the three
//! shipped roles exactly as `nsh_roles::shipped` defines them.

use crate::error::{value, Error};

pub struct Role {
    pub name: String,
    pub digest: bool,
    pub approvals: u8,
    pub max_session_ms: u64,
    pub ceiling: Vec<String>,
    pub escalation: Vec<String>,
}

fn list(s: &str) -> Vec<String> {
    match s {
        "-" => Vec::new(),
        _ => s.split(',').map(str::to_string).collect(),
    }
}

pub fn parse(text: &str) -> Result<Vec<Role>, Error> {
    let lines = text.lines().map(|l| l.split('#').next().unwrap_or("").trim());
    lines.filter(|l| !l.is_empty()).map(role).collect()
}

fn role(line: &str) -> Result<Role, Error> {
    let f: Vec<&str> = line.split_whitespace().collect();
    let [name, digest, approvals, hours, ceiling, escalation] = f[..] else {
        return Err(value(&format!("role line: {line}")));
    };
    let digest = match digest {
        "digest" => true,
        "text" => false,
        _ => return Err(value("role record mode: digest or text")),
    };
    let approvals = approvals.parse().map_err(|_| value("approvals"))?;
    let hours: u64 = hours.parse().map_err(|_| value("max-session-hours"))?;
    let max_session_ms = hours.checked_mul(3_600_000).ok_or_else(|| value("max-session-hours"))?;
    let (ceiling, escalation) = (list(ceiling), list(escalation));
    Ok(Role { name: name.to_string(), digest, approvals, max_session_ms, ceiling, escalation })
}
