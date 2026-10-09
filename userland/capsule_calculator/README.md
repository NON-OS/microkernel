# capsule_calculator

## Role

`capsule_calculator` is the desktop calculator, `app.calculator`, an 880 by
596 window built on `nonos_app_skeleton`. It has five modes
(`src/calc/mode.rs`): Basic, Scientific (sin, cos, tan and their inverses,
ln, log10, exp, factorial, powers, pi and e), Programmer (bases and bitwise
operations), Convert (length, mass, temperature and data) and
History, which holds the last 32 calculations in memory and scrolls (wheel,
Up and Down) to every one of them; a click on a row puts its result back on
the Basic keypad. Arithmetic is `i128` fixed point with 8 fractional digits;
overflow becomes a typed error, never a wrapped value. It is the worked
example of an app capsule in the
handbook's
[Applications, window manager and desktop shell](../../docs/handbook/desktop/app-model.md).

```text
calculator (App trait)
    |
    | nonos_app_skeleton::run
    v
wm (window) + compositor (scene, damage) + input_router (events)
```

## Microkernel contract

`_start` is one line, `run(calc::Calculator::new)`. Everything the window
needs, the skeleton does: it registers and shares the surface, opens the
window with the window manager, submits the scene to the compositor,
subscribes to input and runs the frame loop. The calculator makes no
syscall of its own beyond what the skeleton and the toolkit's paint
library do.

## Authority

`Capsule.mk` declares `CAPSULE_REQUIRED_CAPS := 0x1819`, which decodes to
exactly:

| Bit | Capability | Purpose |
|---|---|---|
| 0x0001 | CoreExec | run user code |
| 0x0008 | IPC | the window services |
| 0x0010 | Memory | heap and window backing |
| 0x0800 | GraphicsDisplayQuery | fit the window to the display |
| 0x1000 | GraphicsSurfaceCreate | register and share the window surface |

`Debug` is absent. No `Driver`, `Mmio`, `Irq`, `Dma`, `Pio`, `Network`,
`Crypto`, `FileSystem`, `Hardware`, `Admin` or `RegisterService` capability
is requested. Endpoints: `service:4720:app.calculator`, reply `4721`, and
the instance windows `app.calculator.1` (4838) and `app.calculator.2`
(4840).

## Privacy posture

| Invariant | How `capsule_calculator` honors it |
|---|---|
| NO LOGS | No Debug bit, no `MkDebug` call; `debug_tag` in the kernel spawn spec is the empty string. |
| NO TRACES | The history is a 32-entry ring in process memory; nothing is written anywhere. |
| EPHEMERAL | Zero files read or written, zero sockets. |

## Keys

Digits, `.`, `+ - * x /`, `=` or Enter, `%`, `n` (sign), `r` (square root),
`q` (square), `i` (reciprocal), `m` recall, `M` store, `a` add to memory, `s`
subtract from memory, `l` clear memory, `c` or Backspace clear, Esc closes
(`src/calc/event/key_classifier.rs`). In Programmer mode `a` to `f` are the hex
digits instead, as the keypad's A to F are. Ctrl+V or Shift+Insert enters a
copied number or sum as the keys that would type it (`src/calc/paste.rs`). The
`00` key enters two zeros.

## Failure model

- Division by zero, an undefined value (the square root of a negative,
  for one) and overflow each stop the calculation, and the readout says
  which: "Cannot divide by zero", "Not defined" or "Result too large"
  (`src/calc/format/error_text.rs`). Further input is ignored until AC.
- A factorial is "Not defined" only for a negative or fractional number;
  past 28!, the largest the fixed point holds, it is "Result too large".
- A conversion past what the fixed point holds says "Result too large".
- Memory operations set the error state and do not store on overflow.
- The readout has room for the widest value the fixed point can hold.

## Explicit non-goals today

- No currency in Convert: there is no source of live exchange rates, and
  fixed ones would be wrong answers that look right.
- No copy of the display value to the clipboard (paste is supported).

## Verification

- Build: `make nonos-mk-calculator`; sign: `make nonos-mk-calculator-sign`.
  Kernel mirror: `src/userspace/capsule_calculator/`.
- `userland/apps_proofs` includes the error kinds, the fixed point, the
  formatter, the binary operations and the scientific functions, and
  checks that hostile input never wraps, panics or shows a wrong number.
  It also includes the window state, the keypads and their actions, the key
  map and the History page's geometry (`calc_keypad_tests`): every digit key
  enters the digits on its label, the hex letters in Programmer mode, the
  factorial's limits and History reaching all 32 entries.
- `make nonos-mk-host-trust-verify` verifies
  the baked `calculator.manifest.bin` against the trust anchor.
