# foot, a Wayland terminal, in the NØNOS frame

One TCG boot of a dev image of foot at a4f35c3c98: gui with
linux/pty (pseudo-terminals, not yet on main) and the window-wait,
with foot's boot-test listing (WAYLAND TTY FILES_HOME EXEC: a terminal
starts a shell). The mirror relay ran on an empty cache, so every package
of foot's closure was fetched upstream for the first time; all installed.

- foot-open.png: `linux foot` in the Terminal, foot's window in the frame,
  its shell's prompt.
- foot-shell.png: `echo hello from foot; uname -a` typed into foot and
  answered by its shell over /dev/pts.
- foot-window.png: the window alone.

What foot said it lacks, each still to do: fractional scaling
(wp_fractional_scale_v1), text input (IME), an xkb Compose file, and
memfd seals (F_ADD_SEALS). None stops it.

foot is not listed until linux/pty is reviewed and on main.
