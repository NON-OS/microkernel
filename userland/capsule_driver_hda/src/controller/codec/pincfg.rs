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

//! The pin configuration default (HDA 1.0a section 7.3.3.31): what the
//! board maker says is wired to a pin. Firmware fills it in for the machine,
//! so it is the only place the driver learns which pin is the speaker.

/// Port connectivity, bits 31:30.
pub const CONN_JACK: u8 = 0;
pub const CONN_NONE: u8 = 1;
pub const CONN_BOTH: u8 = 3;

/// Default device, bits 23:20.
pub const DEV_LINE_OUT: u8 = 0x0;
pub const DEV_SPEAKER: u8 = 0x1;
pub const DEV_HP_OUT: u8 = 0x2;

/// Misc bit 0 (bit 8 of the word): the jack has no presence detect, whatever
/// the pin capabilities say.
const MISC_NO_PRESENCE: u32 = 1 << 8;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum OutKind {
    Speaker,
    Headphone,
    LineOut,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PinConfig(pub u32);

impl PinConfig {
    pub const fn connectivity(self) -> u8 {
        (self.0 >> 30) as u8
    }

    pub const fn device(self) -> u8 {
        ((self.0 >> 20) & 0xf) as u8
    }

    /// Default association, bits 7:4, and sequence, bits 3:0: the order
    /// Linux sorts outputs in (`snd_hda_parse_pin_defcfg`).
    pub const fn order(self) -> u8 {
        (self.0 & 0xff) as u8
    }

    pub const fn presence_overridden(self) -> bool {
        self.0 & MISC_NO_PRESENCE != 0
    }

    /// The output this pin is wired as, or none: a pin marked as not
    /// connected is skipped whatever its device field says, as Linux does.
    pub const fn out_kind(self) -> Option<OutKind> {
        if self.connectivity() == CONN_NONE {
            return None;
        }
        match self.device() {
            DEV_SPEAKER => Some(OutKind::Speaker),
            DEV_HP_OUT => Some(OutKind::Headphone),
            DEV_LINE_OUT => Some(OutKind::LineOut),
            _ => None,
        }
    }

    /// A jack the user plugs into, as opposed to a fixed internal device.
    pub const fn is_jack(self) -> bool {
        let c = self.connectivity();
        c == CONN_JACK || c == CONN_BOTH
    }
}
