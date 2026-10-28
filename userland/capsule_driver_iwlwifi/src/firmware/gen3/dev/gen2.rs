// NONOS Operating System (AGPL-3.0-or-later)

//! A 22000-family receive buffer, read as Linux v6.12 pcie/rx.c
//! `iwl_pcie_rx_handle_rb` reads one before AX210: packets one after another,
//! each starting on a 64-byte boundary (`FH_RSCSR_FRAME_ALIGN`), until the
//! end marker, a packet shorter than its header, or one that would run past
//! the buffer. A received frame's descriptor is the shorter version 1 one
//! (`IWL_RX_DESC_SIZE_V1`, 48 bytes, mvm/rxmq.c), which is handed on in the
//! version 3 layout the frame readers take: the 20 common bytes, then the
//! version 1 fields where version 3 keeps them (rate 36, the energies and
//! the channel 40, the air time 44 and the TSF 48), then the frame at 64.

use alloc::vec::Vec;

use super::super::bringup::is_legacy;
use super::super::cmds::REPLY_RX_MPDU_CMD;
use super::super::packet::{parse, Packet, PACKET_HDR};
use super::super::plan::RB_SIZE;
use super::kinds::Flow;

const FRAME_ALIGN: usize = 64;
/// `IWL_RX_DESC_SIZE_V1` and `sizeof(struct iwl_rx_mpdu_desc)`.
pub const DESC_V1: usize = 48;
const DESC_V3: usize = 64;

/// Give every packet in `rb` to `f`, in order; `Done` when `f` said so.
pub fn packets(rb: &[u8], f: &mut dyn FnMut(&Packet<'_>) -> Flow) -> Flow {
    let mut off = 0;
    while off + PACKET_HDR < RB_SIZE {
        let Some(p) = rb.get(off..).and_then(parse) else { break };
        off += (PACKET_HDR + p.payload.len()).next_multiple_of(FRAME_ALIGN);
        if off > RB_SIZE {
            break;
        }
        let flow = match is_legacy(&p, REPLY_RX_MPDU_CMD) && !p.is_reply() {
            true => v3_of_v1(p.payload).map_or(Flow::Continue, |v| f(&Packet { payload: &v, ..p })),
            false => f(&p),
        };
        if flow == Flow::Done {
            return Flow::Done;
        }
    }
    Flow::Continue
}

/// A version 1 frame notification as a version 3 one, or `None` when it is
/// shorter than its descriptor.
pub fn v3_of_v1(v1: &[u8]) -> Option<Vec<u8>> {
    let frame = v1.get(DESC_V1..)?;
    let mut v3 = Vec::with_capacity(DESC_V3 + frame.len());
    v3.extend_from_slice(&v1[..20]);
    v3.extend_from_slice(&[0; 16]);
    v3.extend_from_slice(&v1[28..32]);
    v3.extend_from_slice(&v1[32..DESC_V1]);
    v3.extend_from_slice(&[0; 8]);
    v3.extend_from_slice(frame);
    Some(v3)
}
