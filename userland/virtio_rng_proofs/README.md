# virtio_rng_proofs

Host proofs for the virtio-rng driver's legacy bring-up
(`capsule_driver_virtio_rng`). The crate includes the shipping constants,
queue and init source with `#[path]` and runs them against a register window
in memory (`nonos_devmodel`), without a boot and without a device.

## What it proves

12 `#[test]` functions: a brought-up device ends with every status bit the
specification requires, a device with no queue is refused and told so, and
no feature bit is claimed (`status_tests`); the queue layout and the request
ring, each completion read from its own used element (`queue_tests`,
`ring_tests`).

The modern transport the driver uses for a modern-only device is proved in
`userland/virtio_transport_proofs`.

## What it does not prove

Entropy quality, the broker calls, and the serving loop.

## Run

```sh
cd userland/virtio_rng_proofs && cargo test --release
```

See [drivers](../../docs/handbook/drivers.md) and
[proofs](../../docs/handbook/verification/proofs.md).
