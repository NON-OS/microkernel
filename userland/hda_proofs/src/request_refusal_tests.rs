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

//! Every frame a client can put in driver.hda0's inbox, through the driver's
//! real header decode and, when it is refused, the reply the loop now sends:
//! the driver's own encoder under `refused`, with E_INVAL. Each frame either
//! decodes to the request it carries or draws one whole reply naming the op
//! and request id it carried (zeros when it is too short to carry them), so
//! its caller is not left waiting out its timeout.

mod boundary_tests;
mod frame;
mod random_tests;
