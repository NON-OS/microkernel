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

/// The refusals, one per check, in the order the checks run.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Refusal {
    /// Step 1: the document does not parse.
    Format(crate::FormatFault),
    /// Step 1: a version other than 5, versions 3 and 4 included.
    Version(u32),
    /// Step 2: the document answers another challenge.
    Challenge,
    /// Step 3: the signed structure is not a TPM quote.
    Quote(crate::QuoteFault),
    /// Step 3: `extraData` is not the client's own `qualifying_data`.
    Binding,
    /// Step 3: `qualifiedSigner` is not a SHA-256 name.
    Signer,
    /// Step 3: the AK's signature does not verify, or is not ECDSA SHA-256.
    Signature,
    /// Step 3: the quote does not select exactly the PCRs.
    Selection,
    /// Step 3: the document's PCR values are not the ones the quote signed.
    PcrDigest,
    /// Step 4: the certificate does not parse.
    EnrolFormat,
    /// Step 4: the certificate names an authority the policy does not hold.
    EnrolAuthority,
    /// Step 4: either half of the authority's signature fails.
    EnrolSignature,
    EnrolFleet,
    EnrolHost,
    EnrolAk,
    EnrolHostKeys,
    EnrolExpired,
    EnrolRevoked,
    /// Step 5: no accepted release has this PCR 9, or it is zero.
    Pcr9,
    Pcr8,
    /// Step 5: PCR 14 records no capsule, or one not the vendor root's.
    Pcr14(crate::CapsuleFault),
    /// Step 5: the TCG event log does not parse.
    Log(crate::LogFault),
    /// Step 5: replaying the log does not give the quoted value of this PCR.
    Replay(u8),
    /// Step 5: the last application in PCR 4 is not the release's loader.
    Loader,
    /// Step 5: PCR 4 holds an application the policy does not list.
    BootApplication,
    /// Step 5: a firmware PCR is not the value enrolled.
    Firmware(u8),
    /// Step 6: the IOMMU does not enforce, or a mapping goes around it.
    Dma,
    /// Step 7: the entries do not fold to the signed root.
    Registry,
    RegistryIncomplete,
    CapsuleCount,
    /// Step 7: an entry the vendor's root did not admit.
    NotVendor,
    /// Step 7: no entry holds the release's `nsh.front` measurement.
    Front,
}
