# Widgets

What a [widget](../../overview/glossary.md#widget) is, how an app draws one and feeds it input, and how the host proofs hold every widget to the same rules. The widgets themselves are listed in [Widget list](widget-list.md).

## Where they live

Every widget is in `nonos_toolkit::widgets`, one module each, re-exported from `userland/toolkit/src/widgets/mod.rs:57-86` with `Rect`, so an app takes everything it draws from one path. Beside the widgets sit the shared pieces:

| Piece | What it is | Where |
|---|---|---|
| `State` | what the app tells a widget about it | `userland/toolkit/src/widgets/state.rs:19-32` |
| `Surface` | what a widget draws on; `PaintBuffer` is one | `userland/toolkit/src/widgets/surface.rs:23-75` |
| `draw` | the typed painters every widget shares | `userland/toolkit/src/widgets/draw/painters.rs:27-59` |
| `text` | measure, place, truncate, tail, wrap | `userland/toolkit/src/widgets/text/mod.rs:32-40` |
| `rect` | inset, split, centre, contains, share | `userland/toolkit/src/widgets/rect.rs` |
| `Pointer` | hover and press from pointer input | `userland/toolkit/src/widgets/pointer.rs:23-75` |
| `Controls` | a surface's hit boxes, giving each control's `State` and the action a click fires | `userland/toolkit/src/widgets/controls/` |
| `Scrolled` | a `Surface` for a pane drawn at a scroll offset | `userland/toolkit/src/widgets/scrolled/paint.rs:24` |

## The contract

| Item | Shape | Rule |
|---|---|---|
| `new` | `pub fn new(..) -> Self` | takes only what the widget cannot be drawn without |
| builders | `.note(&str)`, `.kind(Kind)` and the like | a new option is a new method, so no caller breaks |
| `state` | `.state(State)` | hover, focus, press, selection, disabled |
| `draw` | `draw(&self, fb: &mut impl Surface, at: Rect)` | paints inside `at` and nowhere else |
| `hit` | `hit(&self, at: Rect, x: i32, y: i32) -> ..` | `bool`, or the part that was hit |

The Button is the pattern: `Button::new(label).kind(..).icon(..).state(s)` builds it (`userland/toolkit/src/widgets/button/build.rs:36-66`), and `draw` and `hit` both start from `self.parts(at)`, so a click lands on exactly what was drawn (`userland/toolkit/src/widgets/button/paint.rs:17-64`; `userland/toolkit/src/widgets/button/geometry.rs:53-71`, `Parts`). Every widget has one such geometry function and nothing else works out a position.

A rect, a point or a width crossing the boundary is in screen pixels. A size an app declares as a setting is logical, and the widget scales it; see [Display scale](scaling.md#in-an-app).

## State

`State` has five flags: `hovered`, `focused`, `pressed`, `selected` and `disabled` (`userland/toolkit/src/widgets/state.rs:21-32`). The app owns it and sets it from input. A widget never sees an input event, since that type lives in the app skeleton, which depends on the toolkit and not the other way round.

The states draw in one order everywhere. `disabled` takes the `disabled` role and ignores the rest. `selected` fills with `accent_wash`, `hovered` adds `hover_wash`, `pressed` takes the fill through `draw::pressed`, and `focused` adds a `focus_ring` outline two hairlines wide on top (`userland/toolkit/src/widgets/button/paint.rs:31-57`).

## Hover, press and focus

The skeleton subscribes every app to absolute pointer moves, button down and up, wheel and touch (`userland/app_skeleton/src/setup/input_mask.rs:27-29`). The app passes each one to a `Pointer` it keeps per window, through `moved`, `pressed`, `touched`, `released` and `left` (`userland/toolkit/src/widgets/pointer.rs:36-64`). `Pointer::state(hit)` then answers for any widget: hovered when the pointer is over it, not for a finger; pressed only when the press began on that same widget, so a press dragged off a button and released does not click it (`userland/toolkit/src/widgets/pointer.rs:70-74`).

`Controls<A>` packs that with the hit boxes. An app pushes each control's rect and action once per layout (`userland/toolkit/src/widgets/controls/targets.rs:38`), reads back `state(action)` to draw it (`userland/toolkit/src/widgets/controls/look.rs:39`), and gets a `Step<A>` from each input: `Idle`, `Repaint` when a hover or press changed, or `Act(A)` on a click (`userland/toolkit/src/widgets/controls/step.rs:21-27`). The skeleton's `feed` turns a window's pointer events into those calls (`userland/app_skeleton/src/input/feed.rs:28-35`). Moving within one control repaints nothing; crossing to the next repaints once.

Keyboard focus belongs to the app. It keeps the focused index, moves it on Tab and Shift-Tab, and acts on Enter or Space; the widget only draws the ring. Settings keeps a cursor per page and a search focus this way (`userland/capsule_settings/src/settings/ui/hover_look.rs:32-44`).

## Text

`widgets::text` gives every widget the same text handling: `width`, `centre_top`, `left`, `centre`, `right`, `truncate` (to a width, with `…`, cut on a char boundary), `tail`, `wrap`, `clip` and `fit_size`, with monospace forms (`userland/toolkit/src/widgets/text/mod.rs:32-40`). All measure with the TrueType metrics at the size `readable_px` gives, and `centre_top` is shared by painter and hit test so text and its box cannot drift (`userland/toolkit/src/widgets/text/measure.rs:24-38`).

## Tests

`theme_proofs/src/widget_tests/` holds 156 files of tests that draw real widgets on the host, in every theme, at 4, 5 and 8 quarters. Every test takes the `serial()` lock, since the scale and the theme are process-wide (`userland/theme_proofs/src/widget_tests/serial.rs:27-66`). Across them they check that:

- every changed pixel lies inside `at`;
- `hit` is true on the drawn part and false outside `at`;
- each ink, read back from drawn pixels, reaches its contrast floor on its fill in every state;
- the states draw in the order above.

The whole crate has 283 tests, and all passed at 8cf2e318a. `scripts/check_ui_literals.py` fails on a new ARGB literal in the widgets or in an app's UI.

## Adding a widget

1. Write `widgets/<name>/` to the contract above, with roles and sizes only.
2. Add `mod <name>;` and `pub use <name>::<Name>;` to `widgets/mod.rs`.
3. Add its proofs under `theme_proofs/src/widget_tests/`, taking `serial()`.
4. Add its row to [Widget list](widget-list.md).

Any app can then `use nonos_toolkit::widgets::<Name>;`. A missed step 2 fails to compile; nothing fails silently.

## What widgets do not do

- Keep state or call back into the app. The app holds the state; a widget draws it.
- Lay themselves out. A widget is drawn into the rect it is given.
- Animate. A state change draws the new state in the next frame.
