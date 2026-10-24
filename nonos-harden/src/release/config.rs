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

//! The resolved configuration the record carries (`config`): whether the
//! profile is a release, its name, its kernel features and its rollback
//! index. Read as untrusted, like the rest of the record.

use serde_json::{Map, Value};

type Config = (bool, String, Vec<String>, Option<u32>);

pub fn config(cfg: &Map<String, Value>) -> Result<Config, String> {
    let release = cfg.get("release").and_then(Value::as_bool);
    let release = release.ok_or("`config.release` is not true or false")?;
    let profile = cfg.get("profile").and_then(Value::as_str).unwrap_or("(unnamed)");
    let features = match cfg.get("features") {
        None => Vec::new(),
        Some(Value::Array(a)) => {
            let names = a.iter().map(|f| f.as_str().map(String::from)).collect::<Option<_>>();
            names.ok_or("`config.features` holds something other than names")?
        }
        Some(_) => return Err("`config.features` is not a list".into()),
    };
    let rollback_index = match cfg.get("rollback_index") {
        None => None,
        Some(n) => Some(
            n.as_u64()
                .and_then(|n| u32::try_from(n).ok())
                .ok_or("`config.rollback_index` is not a 32-bit count")?,
        ),
    };
    Ok((release, profile.escape_debug().to_string(), features, rollback_index))
}
