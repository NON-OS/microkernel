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

//! `--flag value` pairs: each flag one of those known and given once, each
//! value present and not itself a flag.

pub struct Given(Vec<(String, String)>);

impl Given {
    pub fn read(args: &[String], known: &[&str]) -> Result<Self, String> {
        let mut given: Vec<(String, String)> = Vec::new();
        let mut it = args.iter();
        while let Some(flag) = it.next() {
            let name = known.iter().find(|k| *k == flag).ok_or(format!("unknown flag `{flag}`"))?;
            let value = it.next().filter(|v| !v.is_empty() && !v.starts_with("--"));
            let value = value.ok_or(format!("`{name}` needs a value"))?;
            if given.iter().any(|(n, _)| n == name) {
                return Err(format!("`{name}` given twice"));
            }
            given.push((name.to_string(), value.clone()));
        }
        Ok(Self(given))
    }
    pub fn get(&self, name: &str) -> Option<&str> {
        self.0.iter().find(|(f, _)| f == name).map(|(_, v)| v.as_str())
    }
}
