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

//! How the TCG event log failed to parse.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogFault {
    /// The bytes end inside an event, or the document carries no log.
    Short,
    /// The log is over `MAX_LOG` bytes.
    TooLarge,
    /// More than `MAX_EVENTS` events.
    TooManyEvents,
    /// An event body over `MAX_EVENT_BODY`.
    EventTooLarge,
    /// The first event is not the crypto-agile Spec ID header, or the header
    /// is not exactly the shape the TCG PC Client specification gives.
    Header,
    /// A digest under an undeclared bank, a bank twice, or too many banks.
    Bank,
    /// A PCR 4 event of a type other than no-action, separator, action or
    /// boot-services application: one the client cannot account for.
    Pcr4Event,
    /// A PCR 4 separator or action whose digest is not its body's SHA-256,
    /// so it could hide a measurement under a harmless label.
    Pcr4Digest,
    /// A PCR index past the 24 a PC TPM has.
    PcrIndex,
    /// A startup-locality event that is misplaced, repeated or malformed.
    Locality,
    /// A PCR 14 event that is not a capsule's: another type, a body other
    /// than 65 bytes, a digest not its body's SHA-256, or an unknown kind.
    Capsule,
}
