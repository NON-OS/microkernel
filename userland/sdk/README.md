# sdk

An application SDK on the native runtime (`userland/nonos_runtime`), plus a
std-shaped library on the libc. Each crate is its own package with its own
lock.

| Crate | Package | What it is |
|---|---|---|
| `nonos_sdk` | `nonos-sdk` | the entry macro `sdk_main!` and the capability groups |
| `nonos_prelude` | `nonos-prelude` | one import for an app: `App`, `Window`, `Canvas`, `Color`, `Rect`, `Widget`, `Control`, `exit`, `log`, `yield_now` |
| `nonos_app` | `nonos-app` | the `App` builder: a window, its size and background, and the input loop |
| `nonos_window` | `nonos-window` | a window over a shared surface |
| `nonos_ui` | `nonos-ui` | `Canvas`, `Color`, `Rect`, `Widget`, `Control` |
| `nonos_appkit` | `nonos-appkit` | `Button`, `Label`, `Panel`, `Theme` |
| `nonos_desktop` | `nonos-desktop` | the client side of the compositor, window manager and input router protocols |
| `nonos_font` | `nonos-font` | an 8 by 8 bitmap font |
| `nonos_std` | `nonos-std` | a `no_std` library shaped like `std`, built on `nonos_libc`, not on the runtime |
| `examples/hello_nonos`, `examples/appkit_demo` | | two apps that open a window |

No `Capsule.mk` builds any of these, so no SDK app is signed, enrolled or in an
image. The one capsule that uses a crate from here is `capsule_gui_proof`, on
`nonos_std`, and it has no `Capsule.mk` either.

See [the libc and std page](../../docs/handbook/userland/libc-and-std.md) and [the app model](../../docs/handbook/desktop/app-model.md).
