# nonos-app

The `App` builder. `App::new(title)` starts at 640 by 480 on a dark background;
`size`, `background` and `window` adjust it. `show` creates a surface through
`nonos_window`, fills it, presents it and logs the title. `run(root)` opens a
desktop window through `nonos_desktop`, paints the root control and then loops
on input events, handing pointer and button events to the control; it exits
with 1 when the window cannot be opened. See [the app model](../../../docs/handbook/desktop/app-model.md).
