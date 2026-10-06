# virtio_gpu_proofs

Host proofs for the virtio-gpu driver (`capsule_driver_virtio_gpu`). The crate
includes the shipping driver source with `#[path]`. Windows in host memory
(`nonos_devmodel`) stand in for the common, notify and device capability
regions, a queue region in host memory stands in for the DMA mapping, and a
part on its own thread consumes the ring the way a device does. It links
`nonos_virtio` for the doorbell bound, and a `nonos_libc` shim.

## What it proves

38 `#[test]` functions over two things the virtio specification fixes:

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
- the answer to a refused request header (`request_refusal_tests`).

## What it does not prove

The discovery, broker and IPC server halves, the virgl probe, and pixels on a
screen.

## Run

```sh
cd userland/virtio_gpu_proofs && cargo test --release
```

See [drivers](../../docs/handbook/drivers.md) and
[proofs](../../docs/handbook/verification/proofs.md).
