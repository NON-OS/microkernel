# Linux windows in the NØNOS frame

A Wayland guest's window drawn inside the toolkit's own frame
(nonos_toolkit::decorations): the NØNOS title bar with the app's title,
minimise, maximise and close, the border and an opaque background, the same
as the Terminal's beside it.

- Image: dev image of s2/gui at e6a160dea6 (s1/heap-bins merged), standard
  profile, TCG, 4 CPUs, 8 GiB.
- swayimg-desktop.png: the whole screen, swayimg open over the Terminal.
- swayimg-window.png: the window alone, cropped from it.

- swayimg-closed-by-button.png: the frame's close button clicked (an
  absolute pointer click at the button): swayimg exits with status 0, as the
  Terminal's lines show.
- swayimg-maximised.png: the frame's maximise button clicked, on s2/gui at
  1628a2e32a: the window keeps its title bar, fills the screen below the
  menubar, and is opaque. A few pixels of the frame's margin still show the
  desktop at its edges.

Dragging by the title bar and minimise are not shown here yet.
