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

//! The host half of `explain`: why a host was refused before the user's
//! credential left this machine.

use crate::{AttestRefusal, EnrolRefusal, HostRefusal};

pub(super) fn host(h: &HostRefusal) -> &'static str {
    match h {
        HostRefusal::HostSig(_) => {
            "its signature over the handshake does not verify, so it \
                                    does not hold the host keys it presented"
        }
        HostRefusal::Attest(a) => attest(a),
        HostRefusal::Challenge => {
            "its attestation answers another challenge than this \
                                   handshake's, so it may be replayed"
        }
        HostRefusal::Enrol(e) => enrol(e),
        HostRefusal::FirmwarePcrs => {
            "PCRs 0, 1, 2 and 7 are not the ones it was enrolled with: \
                                      its firmware or boot settings changed; enrol it again"
        }
    }
}

fn attest(a: &AttestRefusal) -> &'static str {
    match a {
        AttestRefusal::Parse => "its attestation document does not parse as NONOSATT",
        AttestRefusal::Version => "its attestation document is not NONOSATT version 5",
        AttestRefusal::Challenge => {
            "its attestation answers another challenge than this \
                                     handshake's, so it may be replayed"
        }
        AttestRefusal::Quote => "its TPM quote does not verify, or does not cover this handshake",
        AttestRefusal::ReleasePcrs => {
            "PCRs 8 and 9 name no release in the --releases file: it \
                                       booted another build; add that build's release line \
                                       (nsh-probe writes one) only if you trust that build"
        }
        AttestRefusal::Log => "its boot log does not parse as a TCG event log",
        AttestRefusal::Replay => {
            "its boot log does not replay to the PCRs its TPM quoted, so the \
                                  log does not say what booted"
        }
        AttestRefusal::BootApplication => {
            "PCR 4 holds a boot application that is neither the \
                                           release's loader nor one its enrolment lists; enrol \
                                           it again if that boot manager is yours"
        }
        AttestRefusal::Loader => {
            "the release's loader is not among the applications PCR 4 \
                                  holds: it booted another loader than the --releases line names"
        }
        AttestRefusal::PathOnly => {
            "PCR 14 holds capsules admitted on their path alone, as a \
                                    development image's are; a release host's are admitted by \
                                    the vendor's proof"
        }
        AttestRefusal::NotVendor => {
            "PCR 14 holds no capsule, or one admitted by a person's root \
                                     or a local build rather than the vendor's proof"
        }
        AttestRefusal::Dma => "its IOMMU does not confine every device",
        AttestRefusal::Registry => {
            "its capsule registry is incomplete, or its nsh.front or \
                                    network entries are not the ones the --releases line names"
        }
        AttestRefusal::DeviceProof => "its device proof failed",
        AttestRefusal::Unavailable => {
            "this client was built with no attestation checker \
                                       (build it with the nonos feature)"
        }
    }
}

fn enrol(e: &EnrolRefusal) -> &'static str {
    match e {
        EnrolRefusal::Authority => "its enrolment names no authority given with --authority",
        EnrolRefusal::Sig(_) => "its enrolment's authority signature does not verify",
        EnrolRefusal::Fleet => "it is enrolled in another fleet than --fleet",
        EnrolRefusal::Ak => {
            "its enrolment names another TPM attestation key than its quote's: \
                             the machine or its TPM changed; enrol it again"
        }
        EnrolRefusal::HostKeys => "its enrolment names other host keys than the ones it presented",
        EnrolRefusal::HostName => "it is enrolled under another name than --host",
        EnrolRefusal::Expired => {
            "its enrolment has expired; check this machine's clock, or \
                                  enrol the host again"
        }
        EnrolRefusal::Revoked => "its enrolment is in a list given with --revoked-enrol",
    }
}
