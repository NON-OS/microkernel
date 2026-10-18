// NONOS Operating System (AGPL-3.0-or-later)
//! The firmware images and addresses the station command proofs use.


pub(super) static SO_GF: &[u8] = include_bytes!("../../../../nonos-bootloader/firmware/intel/iwlwifi-so-a0-gf-a0-86.ucode");

pub(super) static SO_HR: &[u8] = include_bytes!("../../../../nonos-bootloader/firmware/intel/iwlwifi-so-a0-hr-b0-84.ucode");

pub(super) const ADDR: [u8; 6] = [0x02, 0x11, 0x22, 0x33, 0x44, 0x55];
pub(super) const BSSID: [u8; 6] = [0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F];

// `n` zero bytes after `head`.
pub(super) fn image(head: &[u8], n: usize) -> Vec<u8> {
    let mut v = head.to_vec();
    v.resize(n, 0);
    v
}

pub(super) fn le32(v: &[u8], at: usize) -> u32 {
    u32::from_le_bytes(v[at..at + 4].try_into().unwrap())
}

// The firmware file with the command version table entry for `(group, cmd)`
// rewritten to `version`.
pub(super) fn with_version(blob: &[u8], group: u8, cmd: u8, version: u8) -> Vec<u8> {
    let mut b = blob.to_vec();
    let mut off = 88;
    while off + 8 <= b.len() {
        let ty = le32(&b, off);
        let len = le32(&b, off + 4) as usize;
        if ty == 48 {
            for e in (off + 8..off + 8 + len - len % 4).step_by(4) {
                if b[e] == cmd && b[e + 1] == group {
                    b[e + 2] = version;
                    return b;
                }
            }
        }
        off += 8 + ((len + 3) & !3);
    }
    panic!("no entry for {group:#x}/{cmd:#x}");
}
