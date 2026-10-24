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

//! The receipt's closing line: what the listed capsules can reach and whose
//! proof admitted them. Programs only, so the firmware entry, which reaches
//! nothing and was admitted by no one, cannot dilute either count.

use crate::decode::{reaches_network, Authority};
use crate::entries::Entry;

pub fn summary(entries: &[&Entry]) -> String {
    let networked = entries.iter().filter(|e| reaches_network(e.caps)).count();
    let vendor = entries.iter().filter(|e| e.authority == Authority::Vendor).count();
    let publisher = entries.iter().filter(|e| e.authority == Authority::Publisher).count();
    let developer = entries.len() - vendor - publisher;
    format!(
        "\n{} of {} capsules can reach the network. {} were proved by the vendor \
         key, {} by a key enrolled on that machine, and {} carry a publisher \
         signature with no proof behind it.\n",
        networked,
        entries.len(),
        vendor,
        developer,
        publisher,
    )
}
