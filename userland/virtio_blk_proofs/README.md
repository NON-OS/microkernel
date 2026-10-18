# virtio_blk_proofs

Host-runnable proofs for the virtio-blk driver. The shipping source is
included through `#[path]` and run on the host, bring-up included; its kernel
calls resolve to `libc_shim`.

## The shim

`libc_shim` stands in for `nonos_libc`: an interrupt line a test scripts and
a clock that moves only when a wait takes time, so a wait bounded by the
clock and one bounded by a count of wakes come apart as on a machine; a
broker that grants and logs each request, with a hook run at that moment; a
device list; port I/O answered by a device model, which reaches the DMA
grants by their device addresses; MkExit unwinding the test's thread with
its code. The record types and the bring-up schedule are the libc's own; the
few flag values it cannot mount are held to the libc's text (`abi_tests`).

## What is held

- The request wait ends by the clock, never by a count of wakes: a line that
  keeps firing cannot hold a request past its budget, nor end a healthy one
  early. A request the device never answers stalls the queue by name; the
  next one is refused at once, and the server resets the device. The stall
  after three resets in a row, with under a minute served since the last,
  stops the device, gives it back and ends the driver within a bound: the
  disk reads as gone, not reset for good (`streak_tests`).
- The used element is taken only when it names the request in flight with the
  length that request has (on legacy, any that covers the status byte, as
  the specification's legacy notes allow: bhyve reports one); an index moved
  out of turn stalls the queue. A read's buffer is emptied before the device
  is given it.
- The chain, the available ring and the ring sizes as the specification lays
  them out, every power of two from 4 to 32768, across the index wrap.
- Bring-up against a legacy device on ports (`tests::legacy_dev`) that can
  start with a queue a dead predecessor left running: Bus Master goes on
  only once the device was reset and seen stopped, and never for a device
  that does not stop; MSI-X before INTx, and a function with neither refused
  by name; a refused bring-up and teardown reset the device and switch bus
  mastering off before the claim is given back, last. Nothing unmaps DMA.
- The legacy handshake takes FLUSH alone; the register accessors cannot
  reach outside their window.
- The request parsers: exact framing, every accepted request on the disk.
  Kani harnesses prove the bounds and the framing over every body within
  their bounds, every capacity and every declared payload length.
- The medium rule (`server/acl/rule.rs`): every op but the health check
  answers only the kernel's client or a sender holding `StoreWrite`.
- Orders the host cannot run (barriers, the modern handshake, the server
  loop's turn) and rules over the whole driver (no unsafe block without a SAFETY
  comment, no file over 75 lines) are held in the shipping text.

## Run

```sh
cd userland/virtio_blk_proofs
cargo test --offline
cargo kani                # bounds, framing, readers, header, medium rule
```

See [drivers](../../docs/handbook/drivers.md) and
[proofs](../../docs/handbook/verification/proofs.md).
