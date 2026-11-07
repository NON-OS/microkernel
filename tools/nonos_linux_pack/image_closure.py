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

"""An install's closure by the device's rule (capsule_linux install/get/
closure.rs, index_find.rs, install_if.rs), so an image holds exactly the
packages the device would have fetched one by one: the listed package
first, then each need as it is popped, a name met again skipped, a need
named only by a provider after the named ones, then install_if's."""

# install_if.rs SYSTEM: met by the system, never by a package.
SYSTEM = ("wayland-libs-server",)


def next_need(index, wanted: list, later: list, done: set):
    while wanted:
        w = wanted.pop()
        if not done:
            return w, True
        if index.provided_in(w, done):
            continue
        if w.startswith("so:") or index.listed(w) is not None:
            return w, False
        later.append(w)
    while later:
        w = later.pop()
        if not index.provided_in(w, done):
            return w, False
    return None


def triggered(index, done: set):
    for p in index.pkgs:
        rule = p.install_if
        met = rule and all(n in done or n in SYSTEM for n in rule) and any(n in done for n in rule)
        if p.name not in done and met:
            return p.name
    return None


def closure(index, name: str, extras: tuple = (), most: int = 1024) -> list:
    """The packages, in the device's order; ValueError naming the need that
    nothing provides, or when there are more than `most`."""
    wanted, later, done, names = list(reversed(extras)) + [name], [], [], set()
    while True:
        step = next_need(index, wanted, later, names)
        if step is None:
            trig = triggered(index, names)
            if trig is None:
                return done
            step = (trig, True)
        want, exact = step
        # resolve_installed with nothing installed: an image brings its whole closure.
        pkg = index.resolve(want, exact)
        if pkg is None:
            raise ValueError(f"nothing provides {want}")
        if pkg.name in names:
            continue
        if len(done) == most:
            raise ValueError(f"the closure passes {most} packages")
        names.add(pkg.name)
        done.append(pkg)
        wanted.extend(pkg.depends)
