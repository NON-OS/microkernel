# xhci_proofs

Host-runnable proofs for the xHCI driver's TRB layer, its controller
bring-up, and its event ring. The real driver source is included through
`#[path]` and run on the host.

## TRB field algebra

Everything the driver tells the controller, and everything the controller
answers, crosses the rings as 16-byte TRBs. The device writes event TRBs, so
the extraction of completion code, slot id, type, cycle, and pointer must
read exactly the specification fields; a wrong shift silently addresses the
wrong device slot or misreads a transfer result. The proofs establish that
every getter reads its spec bits, that every setter writes exactly its field
and leaves every other bit untouched, and that getters invert setters, for
every TRB value. Kani proves the algebra over all inputs.

## Control-transfer encodings

The setup, data, and status stage builders are checked against the xHCI
specification section 6.4.1 and the USB GET_DESCRIPTOR layout: the setup
stage carries `bmRequestType 0x80`, `bRequest 0x06`, the descriptor type and
index in `wValue`, the length in `wLength`, an 8-byte transfer length,
immediate data, and the IN transfer type; the data stage carries the buffer
address split across the low words, the 17-bit length, and the IN direction;
the status stage interrupts on completion. The cycle bit, the ring ownership
handshake, follows the caller's argument in every stage. A Kani harness
proves the data stage faithful for every address, length, and cycle.

## Wire header

Header decoding is total, rejects short or mistagged buffers, reads every
field from its wire offset in little-endian order, and an encoded response
header decodes back to the request's fields. A Kani harness proves totality
and field faithfulness.

## Event ring

The event ring is the one structure the controller writes and the driver
reads, so every field of an event TRB is the controller's: the cycle bit,
the type, the completion code, the slot and endpoint ids, the residual length
and the 64-bit pointer. `src/event_ring/` runs the driver's own `EventRing`
and every function that reads it (the interrupt drain, the command and
transfer completion waits, the interrupt-IN poll, the bulk transfer and
Enable Slot) over host memory. The shim's `mk_dma_map` hands the driver's
`DmaPool` page-aligned host memory at a bus address of its own, above 4 GiB,
so a driver that gave the controller a virtual address, or dropped the high
half of a bus address, would fail here.

The other side is a producer written from xHCI 1.2 section 4.9.4. It finds
the segment through the ERSTBA and ERSTSZ the driver programmed, posts events
with its own producer cycle state, flips that state at the end of the
segment, refuses to write over an event the driver has not handed back
through ERDP, and fills the rest of the segment's page with TRBs that look
valid under either cycle state, so a read past the segment shows.

- Dequeue (`dequeue_tests`): a TRB whose cycle bit does not match is not
  read. From every starting dequeue index, three laps in batches of 1, 7 and
  63 read every event once, in order, never past the segment, with the cycle
  state flipped at each wrap; after a full lap nothing is read until the
  controller writes again.
- Hand-back (`erdp_tests`): programming leaves ERDP at the segment base with
  EHB clear. Every consumer then writes back the next unread TRB, inside the
  segment, with DESI zero and EHB written as one, which is how EHB is
  cleared.
- Matching (`match_tests`): a command completion naming another command, or
  a transfer event at the command's address, does not complete the command.
  An Enable Slot answer of slot 0 is refused, and one past MaxSlots is
  refused by the slot table before anything is indexed. A transfer event
  completes a control, bulk or interrupt-IN transfer only when its pointer,
  slot id and endpoint id all name the issued TRB; anything else is parked
  for its owner and never handed to a later transfer. Pointer bits 3:0 are
  reserved and ignored.
- Residual (`residual_tests`): the length handed up is the request less the
  residual clamped to the request, so it never exceeds the request or the
  buffer, and a residual past the request hands up nothing.
- Boundaries (`boundary_tests`): slot 0 and 255, endpoint 0 and 31, a
  residual of u32::MAX read as 24 bits, a null pointer, a pointer of all
  ones, and pointers 8 bytes into the issued TRB and into each neighbour,
  each through every wait it reaches.
- Fuzz (`fuzz_tests`): 200,000 xorshift rounds of up to four random events,
  their steering fields biased to the edges above, handed to a random
  consumer, with half-written TRBs left at the enqueue slot. After every
  round nothing has panicked, the dequeue pointer and ERDP lie in the
  segment, no TRB from past the segment was read, every length is within its
  request and its buffer, and every slot id handed out fits the slot table.

Each rule was checked by breaking it in the driver source (clearing read
TRBs, wrapping late, not flipping the cycle, dropping EHB or DESI, ignoring
the pointer, slot or endpoint, comparing reserved pointer bits, handing up
an unclamped residual) and seeing these tests fail.

What these do not show is timing against a live controller, the interrupt
line, or a part's errata; those need a boot.

## Run

```sh
cd userland/xhci_proofs
cargo test --release
cargo kani                # all-input TRB algebra (requires Kani)
```

See [drivers](../../docs/handbook/drivers.md) and
[proofs](../../docs/handbook/verification/proofs.md).
