# SDK examples

- `hello_nonos`: opens a window with `App::new("Hello").window().show()` and
  declares `caps: [WINDOW]`.
- `appkit_demo`: a 480 by 320 window with a panel, a label and a button from
  `nonos_appkit`, also `caps: [WINDOW]`.

Neither has a `Capsule.mk`, so neither is signed or carried by an image. See
[the libc and std page](../../../docs/handbook/userland/libc-and-std.md).
