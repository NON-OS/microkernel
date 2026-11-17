# Building an app with the toolkit

A screen drawn entirely with the design system, followed through the real `capsule_clock`: its window, its controls, hover and press, the theme and the display scale. [Writing an app](../writing-an-app.md) covers the capsule itself, its manifest and its build; this page covers what it draws.

## What you build

The Clock is four tabs, Clock, Stopwatch, Timer and Set, over a tab strip, with a row of buttons under each. It is 23 files and depends on three crates: `nonos_libc`, `nonos_app_skeleton` and `nonos_toolkit` (`userland/capsule_clock/Cargo.toml`). It writes no colour and no pixel size of its own.

| Piece | Where | What it does |
|---|---|---|
| Entry | `src/main.rs` | `run(clock::Clock::new)` hands the app to the skeleton |
| App | `src/clock/app.rs` | the skeleton's `App` trait: manifest, events, paint, tick |
| Manifest | `src/clock/manifest.rs` | the window: title, logical size, input |
| Controls | `src/clock/controls.rs` | every button's logical rect and the one hit test |
| Hover | `src/clock/hover.rs` | the `Pointer`, what is under it and how each control looks |
| Paint | `src/clock/paint*.rs` | widgets drawn into the controls' rects |

## 1. The window, in logical pixels

The manifest asks for 360 by 440 (`userland/capsule_clock/src/clock/manifest.rs:19-38`). Those are [logical pixels](sizes.md): the skeleton multiplies them by the [display scale](scaling.md) before it opens the window, so the Clock gets 720 by 880 of content on a 2x panel without saying anything about it (`userland/app_skeleton/src/runner/fit_display.rs:58-60`). It also asks for pointer moves and button presses, which hover needs.

## 2. Controls: one rect, one hit test

Each tab's buttons are constant logical rects, the Stopwatch's two at `(40, 300)` and `(190, 300)`, 130 by 46 (`userland/capsule_clock/src/clock/controls.rs:38-55`). `keys(tab)` turns them into screen rects through `scale().px`, and `target_at` answers what is at a point, the tab strip first and then the keys (`userland/capsule_clock/src/clock/controls.rs:57-76`). The painter draws into `keys()` and the click handler asks `target_at`, which asks `keys()`: one function, so a click always lands on the control drawn there.

## 3. Drawing with widgets

`paint` clears the window to the theme's `bg` role, draws the tab in force, then the strip (`userland/capsule_clock/src/clock/paint.rs:41-50`). The Stopwatch's buttons are two widgets:

```rust
let k = keys(Tab::Stopwatch);
Button::new(label).kind(Kind::Primary).state(look(state, Target::Key(0))).draw(fb, k[0]);
Button::new("Reset").state(look(state, Target::Key(1))).draw(fb, k[1]);
```

(`userland/capsule_clock/src/clock/paint_stopwatch.rs:34-43`). The strip is `Tabs::new(&LABELS, index)` drawn into `bar(width)`, a rect whose top and height go through `scale().px` (`userland/capsule_clock/src/clock/tabs.rs:45-52`; `userland/capsule_clock/src/clock/paint_tabbar.rs:24-30`). Colour comes only from `roles()`, inside the widgets or, for the readout, as `roles().text`.

## 4. Hover and press

The Clock keeps a toolkit `Pointer` and the control under it in its state (`userland/capsule_clock/src/clock/state.rs:45-47`). `on_pointer` passes each move, release and leave to the pointer, then `rehover` names the control under it again and asks for a repaint only if that changed (`userland/capsule_clock/src/clock/hover.rs:28-47`). `look(t)` gives a control its `State`, asking the pointer only for the control under it (`userland/capsule_clock/src/clock/hover.rs:50-56`). A press is recorded in `on_event` before the click is handled (`userland/capsule_clock/src/clock/app.rs:39-53`).

## 5. Theme and scale, for free

Nothing in the Clock reads the theme setting or the display. The skeleton's loop follows the theme the person picks ([Themes](themes.md)) and sets the scale from the display ([Display scale](scaling.md)); the widgets and `scale().px` pick both up on the next paint.

## 6. Proving it

`apps_proofs` mounts the Clock's own source and runs it at 1x, 1.25x, 1.5x and 2x. It checks that the tab bar and every button are their 1x rects through `scale().px`, that a click or a hover names the control drawn there, and that one tab's buttons never overlap (`userland/apps_proofs/tests/clock_controls_scale.rs:17-26`; `:86-87`, `the_controls_scale_and_still_name_what_is_drawn_at_every_scale`). It is its own test binary, so moving the process-wide scale races no other test.

## What the Clock does not show

- Keyboard focus. The Clock has none; Settings keeps a focus cursor per page ([Widgets](widgets.md#hover-press-and-focus)).
- A scrolled pane. A list longer than its window draws through `Scrolled` and hit-tests at the pointer's y plus the scroll.
- `Controls`, which packs the pointer and the hit boxes into one value. The Clock keeps the two apart, which reads more plainly at this size.
