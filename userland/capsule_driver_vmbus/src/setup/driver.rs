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

//! The live state the serve loop holds. The contact's grant is held for
//! the driver's life: the host keeps the pages it names.

use crate::connect::Bus;
use crate::hv::Hv;
use crate::offers::Offers;

pub struct Driver {
    pub hv: Hv,
    pub bus: Bus,
    pub offers: Offers,
}
