# input_proofs

Host proofs for the input decoders and the code just above them. Each
`#[path]` include pulls in shipping source: the PS/2 mouse packet and event
decode (`capsule_driver_ps2_input`), the I2C-HID report parse, sample and
gesture decode (`capsule_driver_i2c_hid`), the compositor's damage tracking,
and the input router's request parse and its whole routing: the `state` and
`route` trees of `capsule_input_router`, included under the `crate::` paths
they name each other by. The router's peers (window manager, compositor,
service lookup) answer from a desk the tests lay out (`src/clients/`), and
`nonos_libc` is the shim in `libc_shim/`, whose sends land in an outbox the
tests read back and whose pids can be marked dead or their inboxes full. It
also links `nonos_keymap`.

## What it proves

Tests against hostile and edge-case bytes, with no device:
PS/2 packets (`ps2_tests`), touchpad reports and the HID touchpad decode
(`touchpad_tests`, `hid_touchpad_tests`), gestures and tap-to-click
(`gesture_tests`, `gesture_click_tests`), the keyboard layout tables spot
checked against the national layouts (`layout_tests`), the router's request
parse and press frames (`router_parse_tests`, `press_frame_tests`), and
compositor damage (`damage_tests`). And the router driven whole, event by
event as the drivers post them (`router_route_tests`): keys to focus and each
release to where its press went, the reserved chord, a press grab that keeps a
release off the pressed window for it, and a grab whose holder died; and
focus-only input (`router_focus_tests`): a grab refused to a named capsule
without focus, and keys after a focused window closes going to the shell.
The trusted path (`trusted_rule_tests`, `router_trusted_tests`,
`router_hold_end_tests`, `secure_chord_tests`): only the compositor's notice
is believed; while the band is held nothing reaches a focused window or a
grab's holder; a held key is let go where it went down; a hold lapses or
ends with its holder; and Ctrl+Alt+Delete and Ctrl+Alt+Esc reach the shell
alone even over a grab. Gamepads (`router_pad_tests`, `router_pad_hold_tests`):
an unfocused subscriber sees no pad event, a release follows its press per
pad, and under a prompt the pad is the holder's.

## What it does not prove

Anything about the kernel input ring, or who may drain it.

## Run

```sh
cd userland/input_proofs && cargo test --release
```

See [drivers](../../docs/handbook/drivers.md),
[compositor](../../docs/handbook/desktop/compositor.md) and
[proofs](../../docs/handbook/verification/proofs.md).
