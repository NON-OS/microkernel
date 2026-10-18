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

//! Choosing how the user takes part in pairing, from both sides' IO
//! capabilities (Vol 3, Part H, 2.3.2) and MITM flags, for LE Secure
//! Connections with no out-of-band data (Vol 3, Part H, 2.3.5.1). If
//! neither side asks for MITM protection the method is Just Works.

use super::iocap::IoCap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Method {
    JustWorks,
    NumericComparison,
    /// The initiator shows the passkey and the responder's user types it.
    PasskeyInitiatorDisplays,
    /// The responder shows the passkey and the initiator's user types it.
    PasskeyResponderDisplays,
    /// Both users type the same passkey.
    PasskeyBothInput,
}

pub fn select(init: IoCap, init_mitm: bool, resp: IoCap, resp_mitm: bool) -> Method {
    use IoCap::*;
    use Method::*;
    if !init_mitm && !resp_mitm {
        return JustWorks;
    }
    match (init, resp) {
        (NoInputNoOutput, _) | (_, NoInputNoOutput) => JustWorks,
        (DisplayOnly, DisplayOnly | DisplayYesNo) | (DisplayYesNo, DisplayOnly) => JustWorks,
        (DisplayOnly | DisplayYesNo, KeyboardOnly) | (DisplayOnly, KeyboardDisplay) => {
            PasskeyInitiatorDisplays
        }
        (KeyboardDisplay, KeyboardOnly) => PasskeyInitiatorDisplays,
        (KeyboardOnly, DisplayOnly | DisplayYesNo | KeyboardDisplay) => PasskeyResponderDisplays,
        (KeyboardDisplay, DisplayOnly) => PasskeyResponderDisplays,
        (KeyboardOnly, KeyboardOnly) => PasskeyBothInput,
        (DisplayYesNo | KeyboardDisplay, DisplayYesNo | KeyboardDisplay) => NumericComparison,
    }
}
