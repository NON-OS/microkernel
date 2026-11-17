# Widget list

Every widget in `nonos_toolkit::widgets`, with what it takes, what its hit test answers and what it draws. Each follows the [contract](widgets.md#the-contract) and takes `.state(State)`, which the table does not repeat. Sizes are the logical ones from [Sizes](sizes.md).

## Text and structure

| Widget | `new` | Builders | `hit` gives | Draws |
|---|---|---|---|---|
| `Label` | `(text)` | `.role(TextRole)`, `.ink(Ink)`, `.align(Align)` | none | one line, cut with `…` to `at` |
| `SectionHeader` | `(title)` | `.note`, `.caption()` | none | a title over its rows |
| `Separator` | `()` | `.soft()` | none | a `border` or `border_soft` hairline |
| `Card` | `()` | `.title`, `.note`; `content(at)` gives the area under its head | `bool` | `surface` panel with a `CARD_RADIUS` outline |
| `Column` | `(at)` | `.gap(u32)`; `next(h)` and `rest()` hand out rects down `at` | none | nothing |

## Controls

| Widget | `new` | Builders | `hit` gives | Draws |
|---|---|---|---|---|
| `Button` | `(label)` | `.kind(Kind)`, `.icon(IconId)`, `.icon_size(u32)` | `bool` | a label in a `CONTROL_RADIUS` body |
| `IconButton` | `(IconId)` | `.kind(Kind)`, `.label`, `.icon_size(u32)` | `bool` | an icon in a square button body, or beside its label |
| `Switch` | `(on)` | none | `bool` | a 42 by 24 track and knob |
| `Checkbox` | `(on)` | `.label` | `bool` | a 20 px box with a tick, label to its right |
| `Radio` | `(selected)` | `.label` | `bool` | a 20 px ring with a dot, label to its right |
| `Slider` | `(value, max)` | none | `Option<u32>`, the value at `x` | a track and knob |
| `Field` | `(text)` | `.icon`, `.placeholder`, `.caret`, `.selection`, `.role`; `caret_at(at, x)` | `bool` | a one-line text or search field |

`Kind` is `Secondary` (the default), `Primary`, `Plain`, `Quiet`, `Danger` or `Tonal` (`userland/toolkit/src/widgets/button/kind.rs:21`).

## Navigation and lists

| Widget | `new` | Builders | `hit` gives | Draws |
|---|---|---|---|---|
| `NavItem` | `(label)` | `.icon`, `.active(bool)`, `.count(u32)`, `.compact()` | `bool` | a sidebar entry |
| `Tabs` | `(&labels, selected)` | `.icons`, `.busy`, `.hovered`; `tab(at, i)` | `Option<usize>` | a tab strip |
| `Row` | `(label)` | `.value`, `.value_ink`, `.note`, `.chevron()`, `.icon`, `.leading(w)`, `.trailing(w)` | `bool` | a list row, 44 or 66 high |
| `Table` | `(&[Col])` | `.rows`, `.first`, `.sort(col, up)`, `.selected`, `.hovered`, `.pressed`; `cell_rect`, `head_rect` | `Option<Cell>` | a `sunken` header with a sort mark over striped 32 px rows |
| `Scrollbar` | `(offset, content, viewport)` | none | `Option<u32>`, the offset at `y` | a thin thumb in its track |

## Status

| Widget | `new` | Builders | `hit` gives | Draws |
|---|---|---|---|---|
| `Pill` | `(label)` | `.tone(Tone)`, `.dot()`, `.icon`, `.count`, `.category`, `.filled` | `bool` | a status label, half-height radius |
| `Progress` | `(done, total)` | `.tone(Tone)` | none | a filled bar |
| `Tooltip` | `(text)` | `.below()`, `.max_width`; `place(anchor, bounds)` gives its rect | none | one line on `raised`, cut past 320 px |
| `Toast` | `(text)` | `.tone(Tone)`, `.action`, `.text_lines`, `.max_width` | `bool`, the action | a notice panel, 360 px wide |

## Overlays

| Widget | `new` | Builders | `hit` gives | Draws |
|---|---|---|---|---|
| `Menu` | `(&[Item])` | `.hovered`, `.pressed`, `.width`; `place(anchor, bounds)`, `Menu::next` for arrow keys | `Option<usize>` | a raised panel of rows |
| `Dialog` | `(title)` | `.body`, `.note(text, Tone)`, `.buttons(&[(label, Kind)])`, `.focus`, `.disable`, `.armed`, `.pointer`; `Dialog::next_focus` for Tab | `Option<usize>`, the button | a 440 px panel over a `scrim` |

## Meanings

An app picks a meaning and the theme supplies the colour. `Ink` names a text role: `Text`, `Text2`, `Muted`, `Quiet`, `Accent`, `Ok`, `Warn` and `Danger`. `Tone` names a status: `Plain`, `Accent`, `Ok`, `Warn` and `Danger` (`userland/toolkit/src/widgets/ink.rs:25-61`).

## Icons

`IconId` names 92 icons, from `About` to `ChevronLeft` (`userland/toolkit/src/icons/id.rs:18-111`). Each is an 8-bit coverage mask built into the toolkit from `userland/assets/icons/<name>.a8`, drawn from the `.svg` beside it (`userland/toolkit/src/icons/table.rs:17-113`, `MASKS`). `widgets::draw::icon(fb, id, x, y, size, ink)` draws one at any size, area-averaging the mask and blending it with an opaque ink (`userland/toolkit/src/widgets/draw/painters.rs:59`; `userland/toolkit/src/icons/draw.rs:25-65`). `Button`, `IconButton`, `NavItem`, `Row`, `Pill`, `Tabs` and `Field` take an `IconId` and draw it the same way.
