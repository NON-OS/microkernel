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

//! The recovery words or key a request carries: wiped when dropped, whether
//! the job ran or was refused.

use zeroize::Zeroize;

use crate::answers::borrowed;
use crate::ops;

/* Recovery words and keys: zeroed when dropped, whether the job ran or was refused. */
struct Secret(Vec<String>);

impl Drop for Secret {
    fn drop(&mut self) {
        // Each string's bytes, spare capacity too, through zeroize's volatile writes.
        self.0.iter_mut().for_each(Zeroize::zeroize);
    }
}

pub(crate) fn opening(
    fields: Vec<String>,
    open: fn(&[&str]) -> ops::Answer,
) -> impl FnOnce() -> ops::Answer + Send + 'static {
    let secret = Secret(fields);
    move || open(&borrowed(&secret.0))
}
