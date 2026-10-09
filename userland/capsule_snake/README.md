# capsule_snake

`snake` is the desktop Snake game, `app.snake`, an app on
`nonos_app_skeleton`. It has four modes (Arcade, Classic, Time Attack with a
90-second limit, and Zen) and four difficulties (Easy to Insane), a run
screen, a game-over receipt, and a ranks screen with the best runs and the
awards earned (`src/snake/state/`, `src/snake/paint/`).

The home screen's two cards both lead somewhere. "Daily challenge" names
the day's mode and difficulty, picked from the wall clock's day number so
all sixteen pairs come round in sixteen days (`src/snake/state/daily.rs`);
a click sets that pair on the New Run panel, where Start begins it. "Best
run" is the highest stored score and opens the ranks. On the New Run panel
"Wrap edges" is locked in Zen (always on) and Classic (always off), and a
click on it there changes nothing. The run footer is Pause, Restart and
Quit; the game makes no sound and offers no sound switch.

## Capabilities

`CAPSULE_REQUIRED_CAPS = 0x1859`: CoreExec, IPC, Memory, FileSystem,
GraphicsDisplayQuery and GraphicsSurfaceCreate. It has no network or
hardware authority, and no Debug.

FileSystem is for the ranks and awards: they are kept in `vfs_pool` at
`/games/snake/ranks.dat` and `/games/snake/awards.dat`, and each save is
written with `write_file` and then committed to the disk store with
`persist` (`src/snake/store/`). `vfs_pool` lets `persist` through only on an
install where the user chose persistence at setup; otherwise the ranks last
until power off. If the vfs stops answering, the game stops trying to save
for the rest of the session (`src/snake/store/gate.rs`).

The Ranks screen says, beside Back, what became of the last save or load
(`src/snake/state/kept.rs`, `src/snake/paint/rank_kept.rs`): "Ranks saved
to disk"; "Ranks kept until power off: this boot keeps nothing on disk"
(or the vfs's reason, such as "no space left"); "Ranks not saved:" with the
reason when the write itself failed; or "Stored ranks not read:" when a
ranks or awards file exists but did not read or decode. A first run with
no files says nothing.

Proofs: `apps_proofs` `snake_tests` (the daily pick, the locked switch, and
every line the Ranks screen can show).

## Endpoints and build

Service `service:4732:app.snake`, reply `4733`, and the instance windows
`app.snake.1` (4850) and `app.snake.2` (4852). The kernel mirror is
`src/userspace/capsule_snake`. Build with `make nonos-mk-snake`; sign with
`make nonos-mk-snake-sign`.

`tests/host/` holds play and rank geometry checks; the crate has no
`[[test]]` entry for them, so `cargo test` does not run them.

The app model is described in
[Applications, window manager and desktop shell](../../docs/handbook/desktop/app-model.md).
