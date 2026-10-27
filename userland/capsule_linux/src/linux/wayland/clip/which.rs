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

//! The two selections a guest has, and the objects and numbers of each.
//! Pure, so the host proofs hold it.

use super::{data, select, Object};

/// wl_data_device's selection, the clipboard, or the primary selection.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Which {
    Data,
    Primary,
}

impl Which {
    pub fn at(self) -> usize {
        self as usize
    }

    pub fn device(self) -> Object {
        [Object::DataDevice, Object::PrimaryDevice][self.at()]
    }

    pub fn offer(self) -> Object {
        [Object::DataOffer, Object::PrimaryOffer][self.at()]
    }

    pub fn source(self) -> Object {
        [Object::DataSource, Object::PrimarySource][self.at()]
    }

    /// The device's data_offer and selection events.
    pub fn device_events(self) -> (u16, u16) {
        match self {
            Which::Data => (data::DEVICE_DATA_OFFER, data::DEVICE_SELECTION),
            Which::Primary => (select::PRIMARY_DEVICE_DATA_OFFER, select::PRIMARY_DEVICE_SELECTION),
        }
    }

    /// The offer's offer event, which names one mime type.
    pub fn offer_event(self) -> u16 {
        [data::OFFER_OFFER, select::PRIMARY_OFFER_OFFER][self.at()]
    }

    /// The source's send and cancelled events.
    pub fn source_events(self) -> (u16, u16) {
        match self {
            Which::Data => (data::SOURCE_SEND, data::SOURCE_CANCELLED),
            Which::Primary => (select::PRIMARY_SOURCE_SEND, select::PRIMARY_SOURCE_CANCELLED),
        }
    }
}
