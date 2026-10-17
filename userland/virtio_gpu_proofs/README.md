# virtio_gpu_proofs

Host proofs for the virtio-gpu driver (`capsule_driver_virtio_gpu`). The crate
includes the shipping driver source with `#[path]`. Windows in host memory
(`nonos_devmodel`) stand in for the common, notify and device capability
regions, a queue region in host memory stands in for the DMA mapping, and a
part on its own thread consumes the ring the way a device does. It links
`nonos_virtio` for the doorbell bound, and a `nonos_libc` shim.

## What it proves

49 `#[test]` functions, over what the virtio specification fixes and over the
driver on the display wire:

- **The bring-up, legacy and modern:**
  - the status handshake;
  - the reset a bring-up starts from, waited for and given up on;
  - the features the driver accepts against what was offered;
  - where it tells the part its rings are;
  - the device configuration offsets, the queue size limit and the doorbell
    bound.
- **The control queue:** every command goes out as a two-descriptor chain,
  the answer is read from the descriptor the part named, and the wire bytes
  of each 2D command match the specification's layouts.
  the ring on the driver's own thread through the queue wait's yield. It
  keeps resources with their own pixels and backing and what each scanout
  shows, and can be cued to stop answering, for a while or for good, or to
  kill the driver, at any command. It proves:
  - the takeover of every enabled scanout;
  - CAPS, MODES, STATE and the host's display events;
  - flushes;
  - a mode set wrong at every command always shows a whole picture, old or
    new, and the host never reads pages the driver gave back;
  - a surface the host keeps waits in limbo and goes when the host answers;
    once a transfer landed, and is finished on later wakes;
  - the shared wire sweep, run on the driver's own server.

## What it does not prove

Discovery, the broker, the IPC wait loop itself, the virgl probe, pixels on a
screen, and anything the model's reading of the 2D commands gets wrong (it is
recalled, not checked against the specification).

## Run

```sh
cd userland/virtio_gpu_proofs && cargo test --release
```

See [drivers](../../docs/handbook/drivers.md) and
[proofs](../../docs/handbook/verification/proofs.md).
