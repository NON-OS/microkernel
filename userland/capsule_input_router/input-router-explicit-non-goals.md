# capsule_input_router: Explicit non-goals today

Part of [capsule_input_router](README.md).

## Explicit non-goals today

- No IME or dead-key composition.
- No keyboard macro expansion.
- No global hotkey table. `capsule_desktop_shell` owns global shortcuts.
- No input recording for replay.

## Verification

- `userland/desktop_proofs/` includes `state/press.rs` and runs one press
  through the router's press grab, the window manager and the compositor scene
  together.
- `make nonos-mk-host-trust-verify` verifies the baked
  `input_router.manifest.bin` against the trust anchor.
