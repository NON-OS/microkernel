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

//! The message a failed fetch is shown with.

use alloc::string::String;

use crate::browser::fetch::types::Fetch;
use crate::browser::tls13;

// The reason arrives as a byte on the wire; the sentence is a rendering of it.
pub(super) fn reason(job: &Fetch) -> String {
    let base = match job.error {
        Some(err) => err,
        None => "error",
    };
    match job.tls_alert {
        Some(description) => alloc::format!("{base}: {}", tls13::alert_name(description)),
        None => String::from(base),
    }
}
