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

//! A certificate's validity window, read out for a person to be told about.

/*
 * cert_valid_now answers yes or no and is the only thing that decides.
 * This reads the same two fields with the same parsers, so a refusal can say
 * which side of the window the clock fell on. Nothing accepts or refuses a
 * certificate on what this returns.
 */
/// The notBefore and notAfter of `cert`, packed as YYYYMMDDhhmmss.
pub fn cert_window(cert: &[u8]) -> Option<(u64, u64)> {
    let val = super::cert_valid_now::validity(cert)?;
    let (t1, v1, e1) = super::der_tlv::der_tlv(val, 0)?;
    let (t2, v2, e2) = super::der_tlv::der_tlv(val, e1)?;
    let from = super::cert_time_value::cert_time_value(t1, &val[v1..e1])?;
    let until = super::cert_time_value::cert_time_value(t2, &val[v2..e2])?;
    Some((from, until))
}
