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

/*
 * The one place a Shield screen hands work to the shield service. This
 * build of the wallet does not yet speak the service's request format, so
 * a confirmed request is refused here with a sentence saying so, rather
 * than shown as sent. When the format lands, it lands in this file.
 */

use crate::wallet::state::State;

const NOT_YET: &str = "The shield service answered, but this build of the wallet cannot \
     send it requests yet. Nothing was deposited, sent or withdrawn.";

pub fn submit(state: &State) -> Result<(), &'static str> {
    if let Some(text) = super::absent::banner(state) {
        return Err(text);
    }
    Err(NOT_YET)
}
