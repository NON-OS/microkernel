# nonos-desktop

The client side of the desktop protocols for SDK apps. `lookup_peers` finds
the `compositor`, `wm` and `input_router` services. The crate submits and
removes scene entries and commits damage on the compositor, opens, focuses and
closes windows on the window manager, subscribes to the input router and drains
input. See [the compositor](../../../docs/handbook/desktop/compositor.md) and
[the app model](../../../docs/handbook/desktop/app-model.md).
