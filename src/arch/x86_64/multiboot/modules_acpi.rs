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

#[derive(Debug, Clone)]
pub struct AcpiRsdp {
    pub signature: [u8; 8],
    pub checksum: u8,
    pub oem_id: [u8; 6],
    pub revision: u8,
    pub rsdt_address: u32,
    pub length: Option<u32>,
    pub xsdt_address: Option<u64>,
    pub extended_checksum: Option<u8>,
    pub reserved: Option<[u8; 3]>,
}

impl AcpiRsdp {
    pub fn is_acpi2(&self) -> bool {
        self.revision >= 2
    }

    /* The XSDT address exists from revision 2; below that the bytes are
    not covered by any checksum the RSDP carries. */
    pub fn table_address(&self) -> u64 {
        if self.is_acpi2() {
            if let Some(xsdt) = self.xsdt_address {
                if xsdt != 0 {
                    return xsdt;
                }
            }
        }
        self.rsdt_address as u64
    }

    pub fn verify_checksum(&self) -> bool {
        let mut sum: u8 = 0;
        for &b in &self.signature {
            sum = sum.wrapping_add(b);
        }
        sum = sum.wrapping_add(self.checksum);
        for &b in &self.oem_id {
            sum = sum.wrapping_add(b);
        }
        sum = sum.wrapping_add(self.revision);
        for &b in &self.rsdt_address.to_le_bytes() {
            sum = sum.wrapping_add(b);
        }
        sum == 0
    }

    /* An ACPI 2.0 RSDP passes only with all its extended fields present, a
    length of exactly 36, and those 36 bytes summing to zero, reserved bytes
    included. The extended checksum covers the length the table declares, and
    only 36 bytes are kept, so a longer declaration could hide unchecked bytes.
    Below revision 2 there is no extended checksum to check. */
    pub fn verify_extended_checksum(&self) -> bool {
        if !self.is_acpi2() {
            return true;
        }
        let (Some(len), Some(xsdt), Some(ext), Some(reserved)) =
            (self.length, self.xsdt_address, self.extended_checksum, self.reserved)
        else {
            return false;
        };
        if len != 36 {
            return false;
        }
        let mut sum: u8 = 0;
        for &b in &self.signature {
            sum = sum.wrapping_add(b);
        }
        sum = sum.wrapping_add(self.checksum);
        for &b in &self.oem_id {
            sum = sum.wrapping_add(b);
        }
        sum = sum.wrapping_add(self.revision);
        for &b in &self.rsdt_address.to_le_bytes() {
            sum = sum.wrapping_add(b);
        }
        for &b in &len.to_le_bytes() {
            sum = sum.wrapping_add(b);
        }
        for &b in &xsdt.to_le_bytes() {
            sum = sum.wrapping_add(b);
        }
        sum = sum.wrapping_add(ext);
        for &b in &reserved {
            sum = sum.wrapping_add(b);
        }
        sum == 0
    }
}
