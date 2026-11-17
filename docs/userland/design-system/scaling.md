# Display scale

How an app drawn in [logical pixels](sizes.md) comes out the right size on a large panel: one [display scale](../../overview/glossary.md#display-scale) per process, set by the app skeleton from the display, applied by every widget, and carried into web pages as a device pixel ratio.

## Quarters

The scale is counted in quarters of a screen pixel per logical pixel: 4 is 1x, 5 is 1.25x, 6 is 1.5x and 8 is 2x, clamped to 4..=16 (`userland/toolkit/src/design/scale.rs:17-26`). The toolkit holds it in one atomic, `QUARTERS`, which starts at 4 (`userland/toolkit/src/design/scale.rs:30`).

| Call | Gives | Where |
|---|---|---|
| `scale().px(v)` | `(v * q + 2) / 4`, rounded to nearest | `userland/toolkit/src/design/scale.rs:43-45` |
| `scale().hair()` | `q / 4`, at least 1: a hairline stays a whole pixel | `userland/toolkit/src/design/scale.rs:48-55` |
| `scale().font(px)` | `px * q / 4`, before `readable_px` floors it | `userland/toolkit/src/design/scale.rs:59-61` |
| `scale().quarters()` | the raw value | `userland/toolkit/src/design/scale.rs:38-40` |
| `set_scale(q)` | stores it, clamped | `userland/toolkit/src/design/scale.rs:71-73` |

## Who sets it

The toolkit cannot depend on the [app skeleton](../../overview/glossary.md#app-skeleton), so the skeleton sets it. When it boots an app it fits the window to the display, and `learn_display` stores the menubar, the dock and the frame's quarters, then calls `set_scale(widget_quarters(w, h))` (`userland/app_skeleton/src/runner/chrome.rs:91-96`, `learn_display`). `widget_quarters` is the frame's own scale (`userland/app_skeleton/src/runner/chrome.rs:102-104`), so the widgets and the window frame round them always agree.

The quarters come from the canvas's short side, by one rule shared with first-boot setup and the installer: 8 from 2160, 6 from 1440, 5 from 1000, otherwise 4 (`userland/capsule_install/brand/src/scale_rule.rs:23-30`, `quarters_for`). The skeleton and the desktop shell mount that file rather than copying it (`userland/app_skeleton/src/runner/chrome.rs:34-35`; `userland/capsule_desktop_shell/src/state/scale.rs:25-31`). The compositor already hands a panel of 2560 by 1440 or more a canvas half its size, and the rule goes by that canvas, so nothing is scaled twice (`userland/capsule_desktop_shell/src/state/scale.rs:17-24`).

## The window

A manifest's `width` and `height` are logical. `fit_to_display` multiplies them by `scale().px`, then adds what the frame grows by at its own scale, then caps the window at 88% of the work area (`userland/app_skeleton/src/runner/fit_display.rs:58-75`; `WINDOW_FRACTION`, `:39`). So the content area an app laid itself out for stays the size it asked for, in logical pixels, on every panel.

## In an app

An app keeps its geometry as logical constants and converts them in the one function its painter and its hit test both call. The Clock's controls are logical rects, `keys()` maps each through `scale().px`, and `target_at` is the one hit test for click and hover (`userland/capsule_clock/src/clock/controls.rs:39-76`). [Building an app](building-an-app.md) walks through it.

At the widget boundary, a rect, a point or a width handed to a widget is in screen pixels; a size an app declares as a setting, such as `Col::fixed` or an icon size, is logical and the widget scales it (`userland/toolkit/src/widgets/table/col.rs:42`).

## In a web page

The Browser keeps CSS's own model. A page is laid out in CSS pixels against a viewport of the pane divided by the scale (`userland/capsule_browser/src/browser/state/css_view.rs:23-26`, `css_viewport`). Once layout is done, `doc_to_device` carries every fragment's box, border, radius, clip, shadow, background layer and text size into screen pixels in one pass, and returns at once at 4 quarters (`userland/capsule_browser/src/browser/layout/boxmodel/post/device.rs:28-60`). The relayout key holds the quarters, so a scale change lays the page out again (`userland/capsule_browser/src/browser/event/relayout/pass.rs:84-86`, `laid_key`).

The other way, a click is hit-tested in screen pixels and handed to the page's scripts in CSS pixels (`userland/capsule_browser/src/browser/event/on_page_click.rs:41-46`), through `css_px` and `device_px` (`userland/capsule_browser/src/browser/layout/boxmodel/post/device_units.rs:20-37`). Media queries answer `resolution` and `-webkit-device-pixel-ratio` from the same quarters (`userland/capsule_browser/src/browser/css/parse/media_resolution.rs:19-38`).

## What proves it

- The widgets draw at 4, 5 and 8 quarters in their proofs, which share one lock so no two tests move the process-wide scale at once (`userland/theme_proofs/src/widget_tests/serial.rs:27-39`, `serial`, `SCALES`).
- The skeleton sets the scale before the window opens, and the content grows by it before the frame (`userland/desktop_proofs/tests/skeleton_scale.rs:34-58`).
- 43 `*_scale` test binaries in `apps_proofs/tests/` run ten apps at 4, 5, 6 and 8 quarters and check that the geometry stays in the window and that draw and hit agree, as `clock_controls_scale.rs` does (`userland/apps_proofs/tests/clock_controls_scale.rs:86-90`).
- Pages lay out in CSS pixels and draw at the scale, and links, text and `resolution` follow it (`userland/capsule_browser_proofs/tests/page_device_scale.rs:65-66`; `userland/capsule_browser_proofs/tests/page_device_hit.rs:58-59`).
- A boot at 8cf2e318a, at the default display, showed the desktop and apps at their usual size. No boot on a large panel has been made yet.

## What it does not do

- The installer and first-boot setup keep their own scale, through `nonos_brand`, since they run outside the desktop.
- Scripts see no `window.devicePixelRatio`; the JavaScript engine does not define it.
- A plain-text or error page wraps at the manifest's logical width, not the CSS viewport (`userland/capsule_browser/src/browser/fetch/render_lines.rs:31`).
- A page's scroll offset is kept in screen pixels and is not rescaled when the scale changes.
- Gradient stops and CSS filters are worked out in CSS pixels, not converted.
