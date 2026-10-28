# NONOS Operating System
# Copyright (C) 2026 NONOS Contributors
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU Affero General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
# GNU Affero General Public License for more details.
#
# You should have received a copy of the GNU Affero General Public License
# along with this program. If not, see <https://www.gnu.org/licenses/>.

"""QEMU's window, held over the whole of a recording's X display.

GTK makes a window full screen only by asking the window manager
(_NET_WM_STATE_FULLSCREEN), and Xvfb has none, so `full-screen=on` alone
left the window at the guest's first size, 640x480 in the corner, with the
guest scaled down into it. On a display with no window manager any client
may place any window, so this does the manager's one job: twice a second it
moves every mapped top-level window to 0,0 and the screen's size, and GTK's
zoom-to-fit then draws the guest across all of it. It speaks to the X server
through libX11, which every X client links; with no libX11 it says so and
the recording goes on with the window as GTK left it."""

import ctypes
import ctypes.util
import threading

IS_VIEWABLE = 2


class Attributes(ctypes.Structure):
    """Xlib's XWindowAttributes."""
    _fields_ = [("x", ctypes.c_int), ("y", ctypes.c_int), ("width", ctypes.c_int),
                ("height", ctypes.c_int), ("border_width", ctypes.c_int), ("depth", ctypes.c_int),
                ("visual", ctypes.c_void_p), ("root", ctypes.c_ulong), ("class_", ctypes.c_int),
                ("bit_gravity", ctypes.c_int), ("win_gravity", ctypes.c_int),
                ("backing_store", ctypes.c_int), ("backing_planes", ctypes.c_ulong),
                ("backing_pixel", ctypes.c_ulong), ("save_under", ctypes.c_int),
                ("colormap", ctypes.c_ulong), ("map_installed", ctypes.c_int),
                ("map_state", ctypes.c_int), ("all_event_masks", ctypes.c_long),
                ("your_event_mask", ctypes.c_long), ("do_not_propagate_mask", ctypes.c_long),
                ("override_redirect", ctypes.c_int), ("screen", ctypes.c_void_p)]


def misplaced(windows, width, height):
    """The windows to move: each mapped one, given as (id, map_state, x, y,
    w, h), that does not already cover the screen exactly."""
    return [w for w, state, x, y, ww, wh in windows
            if state == IS_VIEWABLE and (x, y, ww, wh) != (0, 0, width, height)]


def library():
    name = ctypes.util.find_library("X11") or "libX11.so.6"
    try:
        x = ctypes.CDLL(name)
    except OSError:
        return None
    x.XOpenDisplay.restype = ctypes.c_void_p
    x.XOpenDisplay.argtypes = [ctypes.c_char_p]
    x.XDefaultRootWindow.restype = ctypes.c_ulong
    x.XDefaultRootWindow.argtypes = [ctypes.c_void_p]
    x.XQueryTree.argtypes = [ctypes.c_void_p, ctypes.c_ulong, ctypes.POINTER(ctypes.c_ulong),
                             ctypes.POINTER(ctypes.c_ulong),
                             ctypes.POINTER(ctypes.POINTER(ctypes.c_ulong)),
                             ctypes.POINTER(ctypes.c_uint)]
    x.XGetWindowAttributes.argtypes = [ctypes.c_void_p, ctypes.c_ulong, ctypes.POINTER(Attributes)]
    x.XMoveResizeWindow.argtypes = [ctypes.c_void_p, ctypes.c_ulong, ctypes.c_int, ctypes.c_int,
                                    ctypes.c_uint, ctypes.c_uint]
    x.XFree.argtypes = [ctypes.c_void_p]
    x.XFlush.argtypes = [ctypes.c_void_p]
    x.XCloseDisplay.argtypes = [ctypes.c_void_p]
    return x


def windows(x, dpy, root):
    """(id, map_state, x, y, w, h) of each child of the root window."""
    root_ret, parent = ctypes.c_ulong(), ctypes.c_ulong()
    kids, n = ctypes.POINTER(ctypes.c_ulong)(), ctypes.c_uint()
    if not x.XQueryTree(dpy, root, ctypes.byref(root_ret), ctypes.byref(parent),
                        ctypes.byref(kids), ctypes.byref(n)):
        return []
    found = []
    for i in range(n.value):
        a = Attributes()
        if x.XGetWindowAttributes(dpy, kids[i], ctypes.byref(a)):
            found.append((kids[i], a.map_state, a.x, a.y, a.width, a.height))
    if kids:
        x.XFree(kids)
    return found


class Filler:
    """A thread holding every window on `display` over the whole screen
    until stop()."""

    def __init__(self, display, width, height, every=0.5):
        self.display, self.size, self.every = display, (width, height), every
        self.done = threading.Event()
        self.thread = None

    def start(self):
        x = library()
        dpy = x.XOpenDisplay(f":{self.display}".encode()) if x else None
        if not dpy:
            print("nonos-drive: no libX11 to place QEMU's window with: it keeps the size GTK gives it",
                  flush=True)
            return self
        self.thread = threading.Thread(target=self.run, args=(x, dpy), daemon=True)
        self.thread.start()
        return self

    def run(self, x, dpy):
        root = x.XDefaultRootWindow(dpy)
        while not self.done.is_set():
            for w in misplaced(windows(x, dpy, root), *self.size):
                x.XMoveResizeWindow(dpy, w, 0, 0, *self.size)
            x.XFlush(dpy)
            self.done.wait(self.every)
        x.XCloseDisplay(dpy)

    def stop(self):
        self.done.set()
        if self.thread is not None:
            self.thread.join(timeout=5)
