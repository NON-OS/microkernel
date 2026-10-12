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
| `batch_tests` | a frame reaches a wire driver in one FLUSH of up to 15 rectangles, and a refused FLUSH's rectangles are all damaged again (`frame_pacer/drain_batch.rs`) |
| `backoff_tests` | on GOP, one display name asked a tick, 500 ms apart, the gap doubling each fruitless round up to 30 s (`display/backoff.rs`) |
| `poll_tests` | a wire driver's poll: a new pid re-primes, a new serial or a stale FLUSH asks SURFACE again, LOST repaints, nothing else acts (`display/poll_rule.rs`) |
| `head_tests` | a mirrored output's poll: a new serial or a stale FLUSH asks SURFACE again, LOST repaints, silence waits; up to three heads past output 0 (`display/head_rule.rs`) |
| `mirror_tests` | the screen fitted whole and centred onto another output, nearest pixel, the bars and everything outside the damage untouched (`sw_blitter/resample.rs`) |
| `hint_tests` | NCMP 0x0009's body, and only a service in the kernel's HINT_SOURCES (empty today) heard (`display/hint_rule.rs`) |
| `raise_tests` | only the window manager's pid may raise a layer (`state/raise_rule.rs`) |
| `stacking_tests` | layers draw in raise order inside a band and never leave their band |
| `trusted_tests`, `trusted_notice_tests` | only a trusted source places a layer in the trusted band, its holder is the owner of its topmost layer, a raise never lifts a client into it, and the router is told on every change and each second while held (`state/trusted_rule.rs`) |
| `trusted_capture_tests` | a screenshot never holds a pixel of the trusted band, even where a prompt covers a window, nor the cursor, and stops where the canvas or the shot ends (`frame_pacer/canvas_copy.rs`) |
| `repaint_tests` | after every move, raise, open and close, the screen kept current through the reported damage equals the scene composed from scratch |

`frame_model.rs` is the small paint model the repaint tests compare
against. What a boot exercises and these do not: the attach cache, the
present paths and the IPC loop.

Run: `cargo test` in this directory. The compositor is described in
[Compositor](../../docs/handbook/desktop/compositor.md).
