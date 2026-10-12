# virtio_gpu_proofs

Host proofs for the virtio-gpu driver (`capsule_driver_virtio_gpu`). The crate
includes the shipping driver source with `#[path]`. The driver reaches its
registers and its queue through bounds-checked views (`nonos_virtio::Mmio`),
so windows in host memory (`nonos_devmodel`) stand in for the common, notify
and device capability regions and for the queue's DMA grant, and a part on its
own thread consumes the ring the way a device does. A `nonos_libc` shim keeps
the server's replies and debug lines for the tests to read and counts each
release, showing the test the device at that moment; it has no `MkDmaUnmap`
and no `MkExit`, so no included file can free the surface's frames. A clock
counts each question a wait asks as a millisecond.

## What it proves

52 `#[test]` functions, and one check held when the crate builds:

- the bring-up on the shared transport: the status handshake, the features
  taken against those offered (ACCESS_PLATFORM whenever offered, never
  assumed), where the part is told its rings are, the ring cut to a power of
  two the region holds, refusals left FAILED, the doorbell bound
  (`modern_tests`, `modern_refusal_tests`, `modern_queue_tests`,
  `notify_bound_tests`), the device configuration offsets (`config_tests`),
  only the modern id matched (`discovery_tests`), and discovery reading at
  least 128 device records (`discover.rs`, at build);
- the control queue: every command a two-descriptor chain, the answer read
  from the descriptor the part named, an answer out of turn (another
  descriptor, or the used index moved by two) or no answer by the deadline
  ending in a device reset, and a part that wakes late never
  running a stale chain with the next command's bytes (`queue_tests`,
  `late_tests`, `queue_limit_tests`), the wire bytes of each command
  (`reply_tests`, `reply_refusal_tests`, `wire_tests`, `wire_scanout_tests`);
- the scanout mode, the device's own from 16x16 up, and the primary surface
  held to the broker's display grant (`mode_tests`);
- a stall once the surface is shared: a device that works after the reset
  recovered in place, the same resource on the same frames under the same
  surface handle, presenting again; a dead one leaving the driver up and
  `Down`, answering `E_DEVICE`, with nothing released (`recover_tests`);
  `quiesce` resetting the device before it is given back (`teardown_tests`);
- the server: the retired resource ops answered as unknown and a flush held
  decoded or answered (`request_refusal_tests`, `boundary_frame_tests`);
- in the shipping text, the device reset before its claim is released, no
  DMA unmapped by the driver, bus mastering only after the reset
  (`order_tests`), nothing in the server releasing the device or leaving, and
  a failed registration releasing it only after a reset
  (`recover_order_tests`).

## What it does not prove

The broker calls, the receive loop, the virgl probe, and pixels on a screen.

## Run

```sh
cd userland/virtio_gpu_proofs && cargo test --release
```

See [drivers](../../docs/handbook/drivers.md) and
[proofs](../../docs/handbook/verification/proofs.md).
