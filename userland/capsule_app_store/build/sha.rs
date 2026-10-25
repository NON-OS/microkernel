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

//! The short commit the About text names, from the environment or git.

use std::env;
use std::process::Command;

pub(crate) fn resolve_sha() -> String {
    if let Ok(sha) = env::var("NONOS_BUILD_SHA") {
        if !sha.trim().is_empty() {
            return sha.trim().chars().take(12).collect();
        }
    }
    if let Ok(sha) = env::var("GITHUB_SHA") {
        if !sha.trim().is_empty() {
            return sha.trim().chars().take(12).collect();
        }
    }
    if let Some(sha) = Command::new("git")
        .args(["rev-parse", "--short=12", "HEAD"])
        .output()
        .ok()
        .and_then(|o| if o.status.success() { String::from_utf8(o.stdout).ok() } else { None })
        .map(|s| s.trim().to_string())
    {
        if !sha.is_empty() {
            return sha;
        }
    }
    "unknown".into()
}
