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

//! `--name value` pairs. A name given twice is kept twice, for
//! `--loader`; every other name must be given once.

use nsh_enrol::Error;

pub struct Args(Vec<(String, String)>);

impl Args {
    pub fn parse(rest: &[String]) -> Result<Self, Error> {
        let mut out = Vec::new();
        let mut it = rest.iter();
        while let Some(k) = it.next() {
            let name =
                k.strip_prefix("--").ok_or_else(|| Error::Usage(format!("not a flag: {k}")))?;
            let v = it.next().ok_or_else(|| Error::Usage(format!("--{name} needs a value")))?;
            out.push((name.to_string(), v.clone()));
        }
        Ok(Args(out))
    }

    pub fn all(&self, name: &str) -> Vec<&str> {
        self.0.iter().filter(|(k, _)| k == name).map(|(_, v)| v.as_str()).collect()
    }

    pub fn one(&self, name: &str) -> Result<&str, Error> {
        match self.all(name)[..] {
            [v] => Ok(v),
            [] => Err(Error::Usage(format!("--{name} is required"))),
            _ => Err(Error::Usage(format!("--{name} given more than once"))),
        }
    }

    pub fn maybe(&self, name: &str) -> Result<Option<&str>, Error> {
        match self.all(name)[..] {
            [] => Ok(None),
            _ => self.one(name).map(Some),
        }
    }
}
