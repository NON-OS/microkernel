# virtio_blk_proofs

Host-runnable proofs for the virtio-blk driver's untrusted-input parsers. The
real driver source is included through `#[path]` and run on the host.

## No shims, and no device memory

The crate depends only on `nonos_virtio`, because the driver state holds the
modern transport's register regions.

The request parsers take the driver state by reference, so the proofs build a
real `Driver` value: the true struct from the driver source, holding a real
`Queue` whose pointers are null and a `Regs` made with the real constructor.
Nothing is replaced or reimplemented. Because the queue pointers are null,
the proofs also establish that parsing never touches device memory: a
dereference would crash the tests and fail the Kani harnesses.

## Read and write bounds

A block request carries an attacker-controlled LBA and sector count, and for
writes the data itself. The proofs establish that both parsers never panic,
that an accepted request has a sector count within range and `lba + count`
neither overflows nor exceeds the device capacity, and that the framing is
exact: a read must declare precisely a header's payload, and a write is
accepted only when the body length and the declared payload length both equal
the header plus the data for the requested sector count. A byte more or less
is a framing error, so no stale queue data can ever be sent to the device in
place of missing payload. Kani harnesses prove the bounds and the exact
framing over every body within their bounds, every capacity, and every
declared payload length.

This completes the block-driver set: the same isolation property is proven
for AHCI in `driver_proofs` and for NVMe in `nvme_proofs`.

## The medium rule

`server/acl/rule.rs` is included as it ships: every op but the health check
answers only the kernel's client (sender pid 0) or a sender the kernel says
holds `StoreWrite`. A Kani harness proves that over every op, sender and
answer.

## Modern capacity read

`capacity_tests` runs the modern-transport capacity read and refuses a device
region too short to hold the capacity.

## Field readers and the wire header

The bounds-checked little-endian readers behind the parsers return `None`
rather than reading out of bounds, including when the offset plus the width
would overflow a `usize`. Header decoding is total and reads every field from
its wire offset. Kani proves both over all inputs within their bounds.

## Run

```sh
cd userland/virtio_blk_proofs
cargo test --release
cargo kani                # bounds, framing, readers, header, medium rule (requires Kani)
```

See [drivers](../../docs/handbook/drivers.md) and
[proofs](../../docs/handbook/verification/proofs.md).
