// NONOS Operating System (AGPL-3.0-or-later)
//! The vendor requests, held to the shape Linux v6.12 r8152.c sends: a
//! dword-aligned wValue, four data bytes, a block of 0100h or 0000h in
//! wIndex's high byte, and byte enables whose two nibbles agree. Anything
//! else is a STALL, so a malformed access fails the proof that made it.

use nonos_usbnet::mock::Call;

use super::Chip;

const STALL: i32 = -32;
const EPROTO: i32 = -71;

impl Chip {
    /// The answer to `call` when it is a vendor register request.
    pub fn vendor(&mut self, call: &Call) -> Option<Result<Vec<u8>, i32>> {
        let (s, out) = match call {
            Call::In(s, _) if s.request_type == 0xc0 && s.request == 0x05 => (*s, None),
            Call::Out(s, d) if s.request_type == 0x40 && s.request == 0x05 => (*s, Some(d)),
            _ => return None,
        };
        self.transfers += 1;
        if self.transfers <= self.flaky || self.fail_after.is_some_and(|n| self.transfers > n) {
            return Some(Err(EPROTO));
        }
        let (block, lanes) = (s.index & 0xff00, s.index & 0x00ff);
        let shaped = s.value & 3 == 0 && (block == 0x0100 || block == 0);
        Some(match (call, out) {
            (Call::In(_, 4), None) if shaped && [0, 0x33, 0xcc].contains(&lanes) => {
                Ok(self.read_dword(block, s.value, lanes).to_le_bytes().to_vec())
            }
            (_, Some(d)) if shaped && d.len() == 4 && lanes != 0 && lanes >> 4 == lanes & 0xf => {
                let data = u32::from_le_bytes([d[0], d[1], d[2], d[3]]);
                // The bytes outside the enabled lanes are zero.
                let mut kept = 0u32;
                for lane in 0..4 {
                    if lanes & (1 << lane) != 0 {
                        kept |= 0xff << (8 * lane);
                    }
                }
                assert_eq!(data & !kept, 0, "data outside the enabled lanes at {:#06x}", s.value);
                self.write_dword(block, s.value, lanes & 0xf, data);
                Ok(vec![])
            }
            _ => Err(STALL),
        })
    }
}
