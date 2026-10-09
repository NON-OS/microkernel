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

/* Certificates, their chains, and the signature checks the pool performs. */

mod cert_at;
mod cert_count;
mod cert_dns_match;
mod cert_ext;
mod cert_ext_entry;
mod cert_is_ca;
mod cert_issuer;
mod cert_problem;
mod cert_sig_alg;
mod cert_signature;
mod cert_spki;
mod cert_tbs;
mod cert_time_value;
mod cert_valid_now;
mod cert_window;
mod cert_verify_msg;
mod chain_walk;
mod crypto_port;
mod crypto_status;
mod der_tlv;
mod ecdsa_sig_raw;
mod hash_sha384;
mod roots;
mod rtc_now;
mod spki_point;
mod verify_link;
mod verify_p256;
mod verify_p384;
mod verify_rsa;
