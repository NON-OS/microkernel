# demo_screen_proofs

The 0.9.3 demo's screens, drawn on the host through the apps' own paint code
before any boot. About (`capsule_about/src/about`) and the terminal
(`capsule_terminal/src`) are mounted whole by `#[path]`. `libc_shim` answers
their kernel calls from one fixed machine: 9 processes, a measured boot, the
display mode and driver a test picks, and nothing else running. The libc
files that make no call (the procstat header, the worker handoff, the
deadline) are mounted unchanged. `policy_shim` stands in for an absent policy
store.

`tests/screens` opens each app at the size app_skeleton's `fit_rule` gives
it on each mode the recording may use (1024x768, 1280x720, 1280x800,
1920x1080), under virtio-gpu and bga. It frames the window as the runner
does, with the titlebar at the display's quarters, and checks that the
window sits wholly on the screen between the menubar and the dock. It then
paints every About section, and the terminal as it opens and after `help`.
The lib's own unit-test build is off: under `cfg(test)` the terminal's files
compile their host-test variants instead of what ships.

```sh
RUST_TEST_THREADS=1 cargo test --release --config profile.release.overflow-checks=true
DEMO_SHOTS=/some/dir cargo test --release     # also writes every frame as PNG
cargo clippy --release --lib --tests -- -D warnings
```

What a picture cannot show is the device: how the frame reaches the screen
through bga or virtio-gpu is proved in bga_proofs, virtio_gpu_proofs and
gfx_pool_proofs.
