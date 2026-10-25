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

//! How the service answers: at once, refused with why, or as a job on the
//! worker.

use shield_wire::*;

use crate::{jobs, ops};

pub(crate) fn answer(status: i32, values: &Values) -> (i32, String) {
    (status, String::from(values.text()))
}

pub(crate) fn refused(why: &str) -> (i32, String) {
    let mut v = Values::new();
    v.put("why", why);
    (STATUS_REFUSED, String::from(v.text()))
}

pub(crate) fn now(result: ops::Answer) -> (i32, String) {
    match result {
        Ok(v) => answer(STATUS_OK, &v),
        Err(why) => refused(&why),
    }
}

/// Hand `work` to the worker under `name`.
pub(crate) fn job(
    name: &'static str,
    work: impl FnOnce() -> ops::Answer + Send + 'static,
) -> (i32, String) {
    if jobs::start(name, work) {
        (STATUS_STARTED, String::new())
    } else {
        let mut v = Values::new();
        v.put("job", jobs::running().unwrap_or("another"));
        (STATUS_BUSY, String::from(v.text()))
    }
}

pub(crate) fn owned(fields: &[&str]) -> Vec<String> {
    fields.iter().map(|s| String::from(*s)).collect()
}

pub(crate) fn borrowed(fields: &[String]) -> Vec<&str> {
    fields.iter().map(String::as_str).collect()
}
