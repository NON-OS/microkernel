# virtio_gpu_proofs

Host proofs for the virtio-gpu driver (`capsule_driver_virtio_gpu`). The crate
includes the shipping driver source with `#[path]`. Windows in host memory
(`nonos_devmodel`) stand in for the common, notify and device capability
regions, a queue region in host memory stands in for the DMA mapping, and a
part on its own thread consumes the ring the way a device does. It links
`nonos_virtio` for the doorbell bound, and a `nonos_libc` shim.

## What it proves

55 `#[test]` functions over what the virtio specification fixes and what a
dead owner leaves:

- the bring-up, legacy and modern: the status handshake, the features the
  driver accepts against what was offered, where it tells the part its rings
  are, the device configuration offsets, the queue size limit and the
  doorbell bound (`legacy_tests`, `modern_tests`, `modern_queue_tests`,
  `modern_refusal_tests`, `config_tests`, `queue_limit_tests`,
  `notify_bound_tests`);
- the control queue: every command goes out as a two-descriptor chain, the
  answer is read from the descriptor the part named, and the wire bytes of
  each 2D command, scanout ones included, match the specification's layouts
  (`queue_tests`, `reply_tests`, `reply_refusal_tests`, `wire_tests`,
  `wire_scanout_tests`);
- the answer to a refused request header (`request_refusal_tests`);
- the reset a bring-up starts from, waited for and given up on
  (`reset_tests`);
- a dead or silent owner, the driver itself killed, and a restarted
  compositor (G-73, `deaths/`). A host model answers the ring on the driver's
  own thread through the queue wait's yield. It keeps resources with their
  pixels and backing and what each scanout shows. It is cued to stop
  answering, for a while or for good, or to kill the driver, at every
  command of the blank and of the handover. The proofs:
  - nothing of the owner's stays shown, or in a host copy, once the command
    that takes it off has landed;
  - the primary is zero once the driver got that far;
  - a host back again has the next wakes finish the blank;
  - LOST is answered once;
  - a new owner gets a zeroed primary;
  - an id the driver did not make in this life is refused before the host
    hears of it;
  - a blank that cannot be made is given back whole.

## What it does not prove

Discovery, the broker, the IPC wait loop itself, the virgl probe, pixels on a
screen, and anything the model's reading of the 2D commands gets wrong (it
is recalled, not checked against the specification).

## Run

```sh
cd userland/virtio_gpu_proofs && cargo test --release
```

See [drivers](../../docs/handbook/drivers.md) and
[proofs](../../docs/handbook/verification/proofs.md).
