// NONOS Operating System (AGPL-3.0-or-later)

//! The 22000 family's context information, `struct iwl_context_info`
//! (Linux v6.12 iwl-context-info.h), 1792 bytes, little-endian: `version`
//! (mac_id 0, version 2, size in dwords 4), `control` (flags 8), reserved 16,
//! `rbd_cfg` (free ring 24, used ring 32, status word 40), `hcmd_cfg`
//! (command ring 48, its size 56), reserved and the dump, early-debug and
//! PNVM blocks (all left zero, as Linux leaves them), and the DRAM map at
//! 192: 64 UMAC, 64 LMAC and 64 paging addresses.

/// `sizeof(struct iwl_context_info)`.
pub const CTXT_INFO_V1_SIZE: usize = 1792;
const MAX_DRAM_ENTRY: usize = 64;
const OFF_CONTROL: usize = 8;
const OFF_FREE_RBD: usize = 24;
const OFF_USED_RBD: usize = 32;
const OFF_STATUS_WR_PTR: usize = 40;
const OFF_CMD_QUEUE: usize = 48;
const OFF_CMD_QUEUE_SIZE: usize = 56;
const OFF_UMAC_IMG: usize = 192;

/// What the ROM is told: the adapter, the rings and the image maps.
pub struct CtxtInfoV1<'a> {
    /// CSR_HW_REV's low 16 bits.
    pub mac_id: u16,
    pub control_flags: u32,
    pub free_rbd: u64,
    pub used_rbd: u64,
    pub status_wr_ptr: u64,
    pub cmd_queue: u64,
    /// `TFD_QUEUE_CB_SIZE` of the command ring: log2 of its entries less 3.
    pub cmd_queue_size: u8,
    pub umac: &'a [u64],
    pub lmac: &'a [u64],
    pub virt: &'a [u64],
}

impl CtxtInfoV1<'_> {
    /// Write the structure over the first [`CTXT_INFO_V1_SIZE`] bytes of
    /// `buf`; `false` when `buf` is short or an image map holds more than 64
    /// chunks (which would drop firmware rather than fault).
    pub fn write(&self, buf: &mut [u8]) -> bool {
        let maps = [self.umac, self.lmac, self.virt];
        if buf.len() < CTXT_INFO_V1_SIZE || maps.iter().any(|m| m.len() > MAX_DRAM_ENTRY) {
            return false;
        }
        buf[..CTXT_INFO_V1_SIZE].fill(0);
        let mut put = |off: usize, b: &[u8]| buf[off..off + b.len()].copy_from_slice(b);
        put(0, &self.mac_id.to_le_bytes());
        put(4, &((CTXT_INFO_V1_SIZE / 4) as u16).to_le_bytes());
        put(OFF_CONTROL, &self.control_flags.to_le_bytes());
        put(OFF_FREE_RBD, &self.free_rbd.to_le_bytes());
        put(OFF_USED_RBD, &self.used_rbd.to_le_bytes());
        put(OFF_STATUS_WR_PTR, &self.status_wr_ptr.to_le_bytes());
        put(OFF_CMD_QUEUE, &self.cmd_queue.to_le_bytes());
        put(OFF_CMD_QUEUE_SIZE, &[self.cmd_queue_size]);
        for (k, map) in maps.iter().enumerate() {
            for (i, a) in map.iter().enumerate() {
                put(OFF_UMAC_IMG + (k * MAX_DRAM_ENTRY + i) * 8, &a.to_le_bytes());
            }
        }
        true
    }
}
