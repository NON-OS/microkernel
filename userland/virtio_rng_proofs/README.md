# virtio_rng_proofs

Host proofs for the virtio-rng driver (`capsule_driver_virtio_rng`). The crate
includes the shipping constants, register, queue, fill, init and transport
source with `#[path]` and runs them against a register window and DMA memory
in host memory (`nonos_devmodel`, viewed through `nonos_virtio::Mmio`),
without a boot and without a device. The clock is the one stand-in: each
question a wait asks counts as a millisecond.

## What it proves

29 `#[test]` functions, and three checks held when the crate builds:

- the legacy handshake: every status bit the specification requires, a
  device with no queue refused and told so, no feature bit claimed, the ring
  given as a page number, one past 32 bits refused (`status_tests`); the
  legacy QUEUE_NUM taken as offered when the ring region holds it and
  refused, never cut, when it does not, and (at build) the largest such ring
  ending where the legacy layout puts the used ring (`queue_tests`);
- the request ring: descriptor 0 at each request's own position, each
  completion read from its own element, wrapping (`ring_tests`); lengths held
  to the buffer and the caller, and the buffer emptied once handed on
  (`take_tests`); an element naming another descriptor, an index that moves
  by more than the request in flight, or with none, stalls the queue, and a
  completion that wrote nothing hands on zeros, never older bytes
  (`hostile_tests`);
- the fill: the doorbell on both transports, and a wait that ends when the
  clock says, not after a count (`fill_tests`); a request that timed out is
  never served and never posted over, and across an injected late
  completion no byte goes out twice (`late_tests`);
- the reset on both transports (`reset_tests`), and, in the shipping text,
  reset before the claim is released, no DMA unmapped by the driver, a
  stalled queue reset and brought up again (`teardown_tests`), bus mastering
  only after the reset, and the modern transport whenever it is presented
  (`order_tests`).

The modern transport's own steps are proved in
`userland/virtio_transport_proofs`.

## What it does not prove

Entropy quality, the broker calls, and the serving loop as it runs.

## Run

```sh
cd userland/virtio_rng_proofs && cargo test --release
```

See [drivers](../../docs/handbook/drivers.md) and
[proofs](../../docs/handbook/verification/proofs.md).
