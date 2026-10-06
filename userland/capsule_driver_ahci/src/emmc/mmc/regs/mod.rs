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

//! The card's registers as the driver reads them: OCR, CID, CSD (JEDEC eMMC
//! 5.1, 7.1 to 7.3) and the R2 response that carries the last two.

mod cid;
mod csd;
mod ocr;
mod r2;

pub use cid::Cid;
pub use csd::*;
pub use ocr::*;
pub use r2::*;
