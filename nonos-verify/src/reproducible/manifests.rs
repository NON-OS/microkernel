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

//! Reproducibility across machines. Each builder runs `nix build` on its own,
//! with no shared cache, and hands over the nonos-build.json it wrote: every
//! reproducible artifact by sha256, and the configuration it resolved. The
//! builds agree when every manifest names the same files with the same
//! hashes. Enrollment and signing come after the flake and never reach these
//! files, so nothing here is expected to differ.

use serde_json::{json, Map, Value};

pub(super) fn compare(paths: &[&str]) -> std::io::Result<(Value, bool)> {
    let mut seen: Vec<(String, Value)> = Vec::new();
    for p in paths {
        let text = std::fs::read_to_string(p)?;
        let parsed: Value = serde_json::from_str(&text).map_err(|e| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, format!("{p}: {e}"))
        })?;
        seen.push((p.to_string(), parsed));
    }
    let Some((first_path, first)) = seen.first() else {
        return Ok((json!({ "error": "no manifests" }), false));
    };
    let mut verdicts = Map::new();
    let mut all_ok = seen.len() >= 2;
    for (path, other) in &seen[1..] {
        let diff = differences(&first["artifacts"], &other["artifacts"]);
        let same_config = first["config"] == other["config"];
        all_ok &= diff.is_empty() && same_config;
        verdicts.insert(
            path.clone(),
            json!({ "against": first_path, "config_identical": same_config, "differs": diff }),
        );
    }
    let count = first["artifacts"].as_object().map(|a| a.len()).unwrap_or(0);
    verdicts.insert("artifacts".into(), json!(count));
    Ok((Value::Object(verdicts), all_ok))
}

fn differences(a: &Value, b: &Value) -> Vec<String> {
    let (Some(a), Some(b)) = (a.as_object(), b.as_object()) else {
        return vec!["artifacts missing".into()];
    };
    let mut out: Vec<String> =
        a.iter().filter(|(k, v)| b.get(*k) != Some(*v)).map(|(k, _)| k.clone()).collect();
    out.extend(b.keys().filter(|k| !a.contains_key(*k)).cloned());
    out.sort();
    out
}
