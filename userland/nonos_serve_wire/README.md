# nonos_serve_wire

The frames between the capsule a box's SERVE_ONE names (the SimpleX window,
`app.simplex`) and the Linux personality that serves the box's one loopback
port. Both sides import this crate, so the numbers cannot drift: the
header, the four ops, the statuses and the bounds are in `src/ops.rs`, and
`src/lib.rs` has the table of what each op carries. The design is
the SERVE_ONE design.

`cargo test` holds the codec: a `len` that is not the body, a body past
64 KiB, a bad magic or op, and a question its op may not carry are refused.
