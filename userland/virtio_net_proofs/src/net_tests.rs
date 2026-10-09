// NONOS Operating System (AGPL-3.0-or-later)
use crate::constants::{
    MAX_ETHERNET_FRAME, MIN_ETHERNET_FRAME, RING_SLOTS, RX_BUFFER_LEN, RX_DESC_COUNT,
    TX_BUFFER_LEN, VIRTIO_NET_HDR_LEN, VIRTIO_NET_HDR_LEN_V1, VQ_AVAIL_OFFSET, VQ_REGION_SIZE,
    VQ_USED_OFFSET,
};
use crate::protocol::{
    decode_request, encode_response_header, Request, HDR_LEN, MAX_TX_PAYLOAD_BYTES,
};
use crate::queue::{RxQueue, TxQueue};
use crate::rx::take_one;

extern crate alloc;
use alloc::boxed::Box;
use alloc::vec::Vec;

// The RX used ring is written by the device: the descriptor id and the used
// length of every received frame are hostile values. The proofs below build a
// real RxQueue over host memory, write used-ring entries the way a malicious
// device would, and run the real take_one. The property is isolation: the
// returned frame slice always lies inside the payload area of the slot the
// descriptor id selects, never in another slot, another header, or outside
// the buffer area entirely.

const BUF_TOTAL: usize = RX_BUFFER_LEN as usize * RX_DESC_COUNT as usize;
const PAYLOAD_CAP: usize = RX_BUFFER_LEN as usize - VIRTIO_NET_HDR_LEN;

#[repr(align(4096))]
struct Aligned<const N: usize>([u8; N]);

// All access to the fake DMA memory goes through two raw pointers taken once
// at construction, exactly like device memory: taking a fresh mutable borrow
// between accesses would invalidate the queue's pointers under the aliasing
// model, and Miri checks that (cargo miri test runs these harnesses).
struct Harness {
    region_ptr: *mut u8,
    bufs_ptr: *mut u8,
}

impl Drop for Harness {
    fn drop(&mut self) {
        unsafe {
            drop(Box::from_raw(self.region_ptr as *mut Aligned<VQ_REGION_SIZE>));
            drop(Box::from_raw(self.bufs_ptr as *mut Aligned<BUF_TOTAL>));
        }
    }
}

impl Harness {
    fn new() -> Self {
        let region_ptr = Box::into_raw(Box::new(Aligned([0u8; VQ_REGION_SIZE]))) as *mut u8;
        let bufs_ptr = Box::into_raw(Box::new(Aligned([0u8; BUF_TOTAL]))) as *mut u8;
        Self { region_ptr, bufs_ptr }
    }

    fn rx(&mut self) -> RxQueue {
        RxQueue::new(self.region_ptr as u64, 0, self.bufs_ptr as u64, 0, RX_DESC_COUNT)
    }

    fn write_region(&mut self, off: usize, bytes: &[u8]) {
        for (i, b) in bytes.iter().enumerate() {
            unsafe { self.region_ptr.add(off + i).write(*b) };
        }
    }

    fn read_region(&self, off: usize) -> u8 {
        unsafe { self.region_ptr.add(off).read() }
    }

    fn write_buf(&mut self, off: usize, byte: u8) {
        unsafe { self.bufs_ptr.add(off).write(byte) };
    }

    fn set_used_idx(&mut self, idx: u16) {
        self.write_region(VQ_USED_OFFSET + 2, &idx.to_le_bytes());
    }

    fn set_used_elem(&mut self, pos: u16, desc_id: u32, used_len: u32) {
        let off = VQ_USED_OFFSET + 4 + 8 * (pos as usize);
        self.write_region(off, &desc_id.to_le_bytes());
        self.write_region(off + 4, &used_len.to_le_bytes());
    }

    fn assert_frame_in_slot(&self, bytes: &[u8], desc_id: u32) {
        let base = self.bufs_ptr as usize;
        assert!(desc_id < u32::from(RX_DESC_COUNT), "a frame for an id no slot has");
        let slot = desc_id as usize;
        let p = bytes.as_ptr() as usize;
        assert_eq!(p, base + slot * RX_BUFFER_LEN as usize + VIRTIO_NET_HDR_LEN);
        assert!(bytes.len() <= PAYLOAD_CAP, "frame longer than a slot payload");
        assert!(
            p + bytes.len() <= base + (slot + 1) * RX_BUFFER_LEN as usize,
            "frame escapes its slot"
        );
    }
}

// Miri runs the same harnesses with fewer rounds; the full counts are for
// the native fuzz.
fn rounds(full: u64) -> u64 {
    if cfg!(miri) { 300 } else { full }
}

fn xorshift(state: &mut u64) -> u64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state
}

// Half the ids name a primed slot and half are whatever the device wrote, so
// both the frame path and the drop path run on every seed range.
fn hostile_id(s: &mut u64) -> u32 {
    let raw = xorshift(s);
    if raw & 1 == 0 {
        (raw >> 1) as u32 % u32::from(RX_DESC_COUNT)
    } else {
        (raw >> 1) as u32
    }
}

#[test]
fn hostile_used_entries_never_yield_a_frame_outside_its_slot() {
    let mut h = Harness::new();
    for seed in 1..rounds(200_000) {
        let mut s = seed;
        let desc_id = hostile_id(&mut s);
        let used_len = xorshift(&mut s) as u32;
        let last_used = xorshift(&mut s) as u16;

        let mut rx = h.rx();
        rx.last_used = last_used;
        h.set_used_idx(last_used.wrapping_add(1));
        // Used-ring positions index modulo the physical ring, which legacy
        // virtio keeps at its QueueNumMax size, not the smaller number of
        // primed buffers. The driver reads at this position; a harness that
        // wrote modulo the buffer count would test a ring no device has.
        h.set_used_elem(last_used % RING_SLOTS, desc_id, used_len);

        let frame = unsafe { take_one(&mut rx) }.expect("one pending entry yields a frame");
        assert_eq!(rx.last_used, last_used.wrapping_add(1));
        if desc_id >= u32::from(RX_DESC_COUNT) {
            assert!(frame.bytes.is_empty(), "an id past the slots must yield no payload");
            assert_eq!(rx.pending_refill, None, "an id past the slots must not go back to the device");
            continue;
        }
        if used_len as usize <= VIRTIO_NET_HDR_LEN {
            assert!(frame.bytes.is_empty(), "a headerless entry must yield no payload");
        } else {
            h.assert_frame_in_slot(frame.bytes, desc_id);
            let expect = core::cmp::min(used_len as usize - VIRTIO_NET_HDR_LEN, PAYLOAD_CAP);
            assert_eq!(frame.bytes.len(), expect);
        }
        assert_eq!(rx.pending_refill, Some(desc_id as u16));
    }
}

#[test]
fn an_idle_ring_yields_no_frame() {
    let mut h = Harness::new();
    let mut rx = h.rx();
    rx.last_used = 7;
    h.set_used_idx(7);
    assert!(unsafe { take_one(&mut rx) }.is_none());
}

#[test]
fn an_oversized_used_len_is_clamped_to_the_slot_payload() {
    let mut h = Harness::new();
    // The device claims a u32::MAX-byte frame in slot 3; the driver must clamp
    // to the slot's payload capacity and point at the bytes after the header.
    let slot = 3usize;
    let base = slot * RX_BUFFER_LEN as usize + VIRTIO_NET_HDR_LEN;
    for i in 0..PAYLOAD_CAP {
        h.write_buf(base + i, (i % 251) as u8);
    }
    let mut rx = h.rx();
    h.set_used_idx(1);
    h.set_used_elem(0, slot as u32, u32::MAX);
    let frame = unsafe { take_one(&mut rx) }.expect("a pending entry yields a frame");
    h.assert_frame_in_slot(frame.bytes, slot as u32);
    assert_eq!(frame.bytes.len(), PAYLOAD_CAP);
    for (i, b) in frame.bytes.iter().enumerate() {
        assert_eq!(*b, (i % 251) as u8, "frame must expose the slot payload bytes");
    }
}

#[test]
fn a_wild_descriptor_id_is_dropped_and_never_posted_back() {
    let mut h = Harness::new();
    for wild in [u32::from(RX_DESC_COUNT), u32::from(RX_DESC_COUNT) + 3, 0xFFFF, u32::MAX] {
        let mut rx = h.rx();
        h.set_used_idx(1);
        h.set_used_elem(0, wild, 64);
        let frame = unsafe { take_one(&mut rx) }.expect("a pending entry is consumed");
        assert!(frame.bytes.is_empty(), "id {wild:#x} names no slot and must yield nothing");
        assert_eq!(rx.last_used, 1, "the entry is consumed, so the ring moves on");
        assert_eq!(rx.pending_refill, None);
        rx.refill_consumed();
        let avail_idx = u16::from_le_bytes([
            h.read_region(VQ_AVAIL_OFFSET + 2),
            h.read_region(VQ_AVAIL_OFFSET + 3),
        ]);
        assert_eq!(avail_idx, 0, "id {wild:#x} must not be handed back to the device");
    }
}

#[test]
fn the_last_primed_slot_is_still_served() {
    let mut h = Harness::new();
    let mut rx = h.rx();
    let last = u32::from(RX_DESC_COUNT) - 1;
    h.set_used_idx(1);
    h.set_used_elem(0, last, 100);
    let frame = unsafe { take_one(&mut rx) }.expect("a pending entry yields a frame");
    h.assert_frame_in_slot(frame.bytes, last);
    assert_eq!(rx.pending_refill, Some(last as u16));
}

#[test]
fn the_returned_slot_is_refilled_into_the_avail_ring() {
    let mut h = Harness::new();
    let mut rx = h.rx();
    h.set_used_idx(1);
    h.set_used_elem(0, 5, 100);
    let _ = unsafe { take_one(&mut rx) };
    assert_eq!(rx.pending_refill, Some(5));
    // The slot is handed back only after the caller has copied the frame out,
    // by the explicit refill_consumed the rx_packet handler makes; a refill
    // inside take_one would give the slot to the device while the frame
    // borrow is still alive.
    rx.refill_consumed();
    assert_eq!(rx.pending_refill, None);
    let avail_idx = u16::from_le_bytes([
        h.read_region(VQ_AVAIL_OFFSET + 2),
        h.read_region(VQ_AVAIL_OFFSET + 3),
    ]);
    assert_eq!(avail_idx, 1, "refill must publish one avail entry");
    let entry_off = VQ_AVAIL_OFFSET + 4;
    let entry = u16::from_le_bytes([h.read_region(entry_off), h.read_region(entry_off + 1)]);
    assert_eq!(entry, 5, "the refilled entry must name the drained slot");
}

// The constant relations that keep the TX copy in bounds: the tx_packet
// handler rejects any frame above MAX_TX_PAYLOAD_BYTES, send pads short
// frames to the Ethernet minimum, and both extremes must fit behind the
// virtio-net header in a TX slot. If someone widens the MTU or shrinks the
// buffers, this fails before the driver ships.

#[test]
#[allow(clippy::assertions_on_constants)] // guarding constant relations is the point
fn the_tx_length_gate_keeps_every_padded_frame_inside_a_slot() {
    assert_eq!(MAX_TX_PAYLOAD_BYTES as usize, MAX_ETHERNET_FRAME);
    assert!(VIRTIO_NET_HDR_LEN + MAX_ETHERNET_FRAME <= TX_BUFFER_LEN as usize);
    assert!(VIRTIO_NET_HDR_LEN + MIN_ETHERNET_FRAME <= TX_BUFFER_LEN as usize);
    assert!(MIN_ETHERNET_FRAME <= MAX_ETHERNET_FRAME);
    assert!(VIRTIO_NET_HDR_LEN < RX_BUFFER_LEN as usize);
}

// The wire header, as in the other driver proofs.

#[test]
fn decode_never_panics_and_reads_fields_from_their_offsets() {
    const MAGIC: u32 = 0x4E4E_4554; // the wire tag from protocol/header.rs
    for seed in 1..rounds(100_000) {
        let mut s = seed;
        let blen = (xorshift(&mut s) % 40) as usize;
        let mut buf: Vec<u8> = (0..blen).map(|_| (xorshift(&mut s) & 0xff) as u8).collect();
        if buf.len() >= 6 && seed % 2 == 0 {
            buf[0..4].copy_from_slice(&MAGIC.to_le_bytes());
            buf[4..6].copy_from_slice(&1u16.to_le_bytes());
        }
        if let Some(req) = decode_request(&buf) {
            assert!(buf.len() >= HDR_LEN);
            assert_eq!(req.op, u16::from_le_bytes([buf[6], buf[7]]));
            assert_eq!(req.flags, u16::from_le_bytes([buf[8], buf[9]]));
            assert_eq!(req.request_id, u32::from_le_bytes([buf[12], buf[13], buf[14], buf[15]]));
            assert_eq!(req.payload_len, u32::from_le_bytes([buf[16], buf[17], buf[18], buf[19]]));
        }
    }
}

#[test]
fn encoded_response_headers_decode_back_to_the_request_fields() {
    for seed in 1..rounds(20_000) {
        let mut s = seed;
        let request = Request {
            op: (xorshift(&mut s) & 0xffff) as u16,
            flags: (xorshift(&mut s) & 0xffff) as u16,
            request_id: (xorshift(&mut s) & 0xffff_ffff) as u32,
            payload_len: 0,
        };
        let payload_len = (xorshift(&mut s) & 0xffff_ffff) as u32;
        let mut out = [0u8; HDR_LEN];
        encode_response_header(&mut out, &request, payload_len);
        let back = decode_request(&out).expect("a response header carries the wire tag");
        assert_eq!(back.op, request.op);
        assert_eq!(back.flags, request.flags);
        assert_eq!(back.request_id, request.request_id);
        assert_eq!(back.payload_len, payload_len);
    }
}

// The header ahead of each frame depends on the transport: 10 bytes on the
// legacy transport, 12 under VERSION_1, where struct virtio_net_hdr always
// carries num_buffers. A queue must cut each frame after exactly its own
// transport's header, or a modern NIC hands the stack every frame with two
// header bytes in front and every Ethernet header two bytes off.

impl Harness {
    fn assert_frame_in_slot_after(&self, bytes: &[u8], desc_id: u32, hdr: usize) {
        let base = self.bufs_ptr as usize;
        assert!(desc_id < u32::from(RX_DESC_COUNT), "a frame for an id no slot has");
        let slot = desc_id as usize;
        let p = bytes.as_ptr() as usize;
        assert_eq!(p, base + slot * RX_BUFFER_LEN as usize + hdr);
        assert!(bytes.len() <= RX_BUFFER_LEN as usize - hdr, "frame longer than a slot payload");
        assert!(
            p + bytes.len() <= base + (slot + 1) * RX_BUFFER_LEN as usize,
            "frame escapes its slot"
        );
    }
}

#[test]
fn each_transport_cuts_the_frame_after_its_own_header() {
    assert_eq!(RxQueue::new(0, 0, 0, 0, 1).hdr_len, VIRTIO_NET_HDR_LEN, "legacy by default");
    for hdr in [VIRTIO_NET_HDR_LEN, VIRTIO_NET_HDR_LEN_V1] {
        let mut h = Harness::new();
        let slot = 2usize;
        let start = slot * RX_BUFFER_LEN as usize;
        for i in 0..80 {
            h.write_buf(start + i, i as u8);
        }
        let mut rx = h.rx().with_hdr_len(hdr);
        h.set_used_idx(1);
        h.set_used_elem(0, slot as u32, 60 + hdr as u32);
        let frame = unsafe { take_one(&mut rx) }.expect("a pending entry yields a frame");
        h.assert_frame_in_slot_after(frame.bytes, slot as u32, hdr);
        assert_eq!(frame.bytes.len(), 60, "the used length less this transport's header");
        assert_eq!(frame.bytes[0], hdr as u8, "the first byte after the header");
    }
}

#[test]
fn hostile_used_entries_on_a_version_1_queue_never_leave_their_slot() {
    let mut h = Harness::new();
    let hdr = VIRTIO_NET_HDR_LEN_V1;
    for seed in 1..rounds(200_000) {
        let mut s = seed;
        let desc_id = hostile_id(&mut s);
        let used_len = xorshift(&mut s) as u32;
        let last_used = xorshift(&mut s) as u16;

        let mut rx = h.rx().with_hdr_len(hdr);
        rx.last_used = last_used;
        h.set_used_idx(last_used.wrapping_add(1));
        h.set_used_elem(last_used % RING_SLOTS, desc_id, used_len);

        let frame = unsafe { take_one(&mut rx) }.expect("one pending entry yields a frame");
        if desc_id >= u32::from(RX_DESC_COUNT) {
            assert!(frame.bytes.is_empty(), "an id past the slots must yield no payload");
            assert_eq!(rx.pending_refill, None, "an id past the slots must not go back to the device");
            continue;
        }
        if used_len as usize <= hdr {
            assert!(frame.bytes.is_empty(), "a headerless entry must yield no payload");
        } else {
            h.assert_frame_in_slot_after(frame.bytes, desc_id, hdr);
            let expect = core::cmp::min(used_len as usize - hdr, RX_BUFFER_LEN as usize - hdr);
            assert_eq!(frame.bytes.len(), expect);
        }
        assert_eq!(rx.pending_refill, Some(desc_id as u16));
    }
}

#[test]
fn an_entry_no_longer_than_the_version_1_header_yields_no_payload() {
    let mut h = Harness::new();
    for used_len in 0..=VIRTIO_NET_HDR_LEN_V1 as u32 {
        let mut rx = h.rx().with_hdr_len(VIRTIO_NET_HDR_LEN_V1);
        h.set_used_idx(1);
        h.set_used_elem(0, 4, used_len);
        let frame = unsafe { take_one(&mut rx) }.expect("a pending entry");
        assert!(frame.bytes.is_empty(), "used_len {used_len} carries no frame");
    }
}

#[test]
fn a_header_length_past_the_slot_is_held_to_the_slot() {
    let mut h = Harness::new();
    for used_len in [0u32, 64, RX_BUFFER_LEN, RX_BUFFER_LEN + 1, u32::MAX] {
        let mut rx = h.rx().with_hdr_len(usize::MAX);
        h.set_used_idx(1);
        h.set_used_elem(0, 9, used_len);
        let frame = unsafe { take_one(&mut rx) }.expect("a pending entry");
        assert!(frame.bytes.is_empty(), "nothing past a slot-sized header is a frame");
    }
}

#[test]
#[allow(clippy::assertions_on_constants)] // guarding constant relations is the point
fn the_tx_length_gate_holds_for_the_version_1_header_too() {
    assert_eq!(VIRTIO_NET_HDR_LEN, 10);
    assert_eq!(VIRTIO_NET_HDR_LEN_V1, 12);
    assert!(VIRTIO_NET_HDR_LEN_V1 + MAX_ETHERNET_FRAME <= TX_BUFFER_LEN as usize);
    assert!(VIRTIO_NET_HDR_LEN_V1 + MIN_ETHERNET_FRAME <= TX_BUFFER_LEN as usize);
    assert!(VIRTIO_NET_HDR_LEN_V1 < RX_BUFFER_LEN as usize);
    let tx = TxQueue::new(0, 0, 0, 0, 1);
    assert_eq!(tx.hdr_len, VIRTIO_NET_HDR_LEN, "legacy by default");
    assert_eq!(tx.with_hdr_len(VIRTIO_NET_HDR_LEN_V1).hdr_len, VIRTIO_NET_HDR_LEN_V1);
}
