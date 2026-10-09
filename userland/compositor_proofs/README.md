# compositor_proofs

Host-runnable proofs for the compositor, on the real source included
through `#[path]`: the damage accumulator, the scene table and its submit,
raise and remove steps, the raise rule, the software blitter and the
request protocol (`../compositor/src/state/`, `sw_blitter/`, `protocol/`).

| Tests | What they hold |
|---|---|
| `damage_tests` | damage coverage is never lost across a merge |
| `blitter_tests`, `upscale_tests` | the blitter never writes outside the destination, and the 2x upscale fills what it should |
| `parse_tests` | every frame through the real header decode: served, or refused with a status and the request it answers |
| `raise_tests` | only the window manager's pid may raise a layer (`state/raise_rule.rs`) |
| `stacking_tests` | layers draw in raise order inside a band and never leave their band |
| `repaint_tests` | after every move, raise, open and close, the screen kept current through the reported damage equals the scene composed from scratch |

`frame_model.rs` is the small paint model the repaint tests compare
against. What a boot exercises and these do not: the attach cache, the
present paths and the IPC loop.

Run: `cargo test` in this directory. The compositor is described in
[Compositor](../../docs/handbook/desktop/compositor.md).
