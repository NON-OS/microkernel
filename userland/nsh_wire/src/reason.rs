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

//! The u16 reason codes carried by `ABORT`, `AUTH_REFUSED` and
//! `ESCALATE_RESULT`. An unknown code is refused, not shown as a number.

use crate::WireError;

macro_rules! reasons {
    ($($name:ident = $code:expr,)+) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Reason {
            $($name,)+
        }

        impl Reason {
            pub const fn code(self) -> u16 {
                match self {
                    $(Reason::$name => $code,)+
                }
            }

            pub const fn from_code(code: u16) -> Result<Reason, WireError> {
                match code {
                    $($code => Ok(Reason::$name),)+
                    _ => Err(WireError::Value),
                }
            }
        }
    };
}

reasons! {
    Ok = 0,
    Protocol = 1,
    Version = 2,
    Policy = 3,
    Limit = 4,
    Busy = 5,
    GrantSig = 16,
    GrantTime = 17,
    GrantScope = 18,
    GrantRevoked = 19,
    RevocationStale = 20,
    RoleUnknown = 21,
    CapsCeiling = 22,
    CapUnknown = 23,
    UserSig = 24,
    EscParent = 32,
    EscApproval = 33,
    EscTooLong = 34,
    EscActive = 35,
}
