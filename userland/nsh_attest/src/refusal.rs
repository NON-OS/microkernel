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

//! Why a host was refused. Each refusal is one check of
//! docs/design/nsh-v1-attest.md, and the first one met ends the session
//! with `ABORT(POLICY)`. Its name, from `Refusal::name`, is what the user is
//! shown: `POLICY (pcr9)`.

/// How a document or a TPM structure failed to parse.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormatFault {
    /// The bytes end inside a field.
    Short,
    /// Bytes are left after the last field.
    Trailing,
    /// A length is over its cap.
    Oversize,
    /// The magic is not `NONOSATT`.
    Magic,
    /// A field holds a value outside its range.
    Value,
}

/// How the quote's signed structure failed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuoteFault {
    /// `TPMS_ATTEST` does not parse, or has bytes after `pcrDigest`.
    Malformed,
    /// It does not start with `TPM_GENERATED_VALUE`.
    NotTpmGenerated,
    /// Its type is not `TPM_ST_ATTEST_QUOTE`.
    NotAQuote,
}

/// How the TCG event log failed to parse.
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
}
