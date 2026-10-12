# virtio_net_proofs

Host-runnable proofs for the virtio-net driver. The real driver source is
included through `#[path]` and run on the host: the receive and transmit
queues over fake DMA memory, the legacy queue programming over a model
register file, the transport choice over a broker in memory, and the IPC
dispatch and handlers end to end. What stands in: the legacy register file
(`regs`), the uptime clock (`clock`), the reply call (`server::error`) and the
driver struct (`setup`), whose bring-up and teardown call the broker.

## The receive path, driven by a hostile device

Every used-ring entry is the device's word: a descriptor id and a used
length. For every id, length and ring position (two hundred thousand seeded
entries per header length, plus every length from 0 to 4096 and the
extremes), `take_one` hands on at most a 1514-byte Ethernet frame, copied
from exactly the buffer the id names after exactly this transport's header,
posts that buffer back wiped, and never posts back an id that names no
buffer. A longer frame is dropped and its buffer recycled. This replaced a
test that asserted a 2038-byte frame was handed on: such a frame overran the
1552-byte reply and panicked the capsule.

`OP_RX_PACKET` answers every length with a reply that fits its buffer, and
wipes the frame from the buffer once sent. `OP_RX_BATCH` never passes
`BATCH_MAX_BYTES`, whatever lengths the device reports, and a lost batch is
answered again only to its caller.

## The transmit path

A frame goes into a free buffer behind a zeroed header, padded to 60 bytes,
and is offered; with every buffer the device's the answer is `E_AGAIN` at
once. Only a buffer posted and not yet returned comes back, whatever ids the
device makes up, and at most a ring's worth of entries is read per call. A
ring full for `TX_STALL_MS` with the link up is `Stalled`, and the serve loop
is told to reset the device; a completion that arrives late, before then,
ends the count; a link that is down never counts. `OP_TX_PACKET` takes 14 to
1514 bytes and a length that agrees with its header.

## Queues, transport and setup

The legacy device is refused, and marked FAILED, for any QueueNumMax but
256 before it is given a ring address; the modern device is told exactly 256
and refused below it. A transitional function that presents the virtio 1.0
structures is driven modern, and the modern negotiation takes
ACCESS_PLATFORM whenever offered and never the device's MAC. The legacy reset
is waited for by the clock; a DMA grant is used only when it covers the
request, page aligned, non-zero and not wrapping. Orders the host cannot run
are checked in the source text: Bus Master only after the reset, teardown
resetting the device before releasing it, and no DMA unmapped by the driver.
`rules_tests` holds the code rules over the whole driver: no file past 75
lines, no blank runs, a SAFETY comment on every unsafe block.

## The station address and the modern MAC read

`station_tests` holds that neither transport takes the device's MAC feature,
that link status is still taken, and that the driver draws its own address.
`mac_tests` runs the modern-transport MAC read.

## Wire header

Header decoding is total, rejects short or mistagged buffers, reads every
field from its wire offset in little-endian order, and an encoded response
header decodes back to the request's fields. A Kani harness proves totality
and field faithfulness.

## Run

```sh
cd userland/virtio_net_proofs
cargo test --release
cargo kani                # wire header decode (requires Kani)
```

See [drivers](../../docs/handbook/drivers.md) and
[proofs](../../docs/handbook/verification/proofs.md).
