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

//! An error answer's words: the error's type and, when it has one, the type
//! of the fault under it. An agent's error names its own kind and the fault
//! under that, as a relay the core could not reach does: `errorAgent BROKER
//! NETWORK`, where the kind alone said only that the agent failed.

use alloc::format;
use alloc::string::String;

use super::json::Value;

/// The members an agent error's kind keeps its own fault in.
const AGENT_FAULTS: [&str; 6] = ["brokerErr", "connErr", "smpErr", "cmdErr", "xftpErr", "ntfErr"];

pub fn error(v: &Value) -> String {
    let e = v.get("chatError");
    let kind = e.get("type").str().unwrap_or("error");
    let detail = e.at(&["errorType", "type"]).str().or(e.at(&["storeError", "type"]).str());
    match (detail, agent(e.get("agentError"))) {
        (Some(d), _) => format!("{kind} {d}"),
        (None, Some(a)) => format!("{kind} {a}"),
        (None, None) => String::from(kind),
    }
}

/// An agent error's kind, and the type of its fault when it names one.
fn agent(a: &Value) -> Option<String> {
    let kind = a.get("type").str()?;
    let fault = AGENT_FAULTS.iter().find_map(|m| a.at(&[m, "type"]).str());
    Some(match fault {
        Some(f) => format!("{kind} {f}"),
        None => String::from(kind),
    })
}
