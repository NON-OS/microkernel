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

"""The round trip, through LibreOffice's UNO bridge: for each format, a new
Writer document with a paragraph in each bundled font is saved, closed,
reopened and compared, edited and saved again, closed, reopened and
compared, then exported to PDF to see the fonts embedded. Every step prints
one line; the run exits 0 only when every check holds."""

import os
import subprocess
import tempfile
import time
from pathlib import Path

import uno
from com.sun.star.beans import PropertyValue

from . import check

FILTERS = {"odt": "writer8", "docx": "MS Word 2007 XML"}


def prop(name, value):
    p = PropertyValue()
    p.Name, p.Value = name, value
    return p


def connect(pipe: str, seconds: float = 60):
    local = uno.getComponentContext()
    resolver = local.ServiceManager.createInstanceWithContext("com.sun.star.bridge.UnoUrlResolver", local)
    deadline = time.monotonic() + seconds
    while True:
        try:
            ctx = resolver.resolve(f"uno:pipe,name={pipe};urp;StarOffice.ComponentContext")
            return ctx, ctx.ServiceManager.createInstanceWithContext("com.sun.star.frame.Desktop", ctx)
        except Exception:
            if time.monotonic() > deadline:
                raise
            time.sleep(0.5)


def load(desktop, url: str):
    return desktop.loadComponentFromURL(url, "_blank", 0, (prop("Hidden", True),))


def paragraphs(doc) -> list:
    out, e = [], doc.Text.createEnumeration()
    while e.hasMoreElements():
        out.append(e.nextElement().getString())
    return out


def para_fonts(doc) -> list:
    """Each paragraph's font, read off its first run of text: in .docx the
    paragraph mark keeps a font of its own."""
    out, e = [], doc.Text.createEnumeration()
    while e.hasMoreElements():
        p = e.nextElement()
        runs = p.createEnumeration()
        while runs.hasMoreElements():
            r = runs.nextElement()
            if r.getString().strip():
                out.append(r.CharFontName)
                break
    return out


def append(doc, text: str, font: str) -> None:
    t = doc.Text
    cur = t.createTextCursorByRange(t.End)
    if paragraphs(doc) != [""]:
        t.insertControlCharacter(cur, 0, False)  # PARAGRAPH_BREAK
    cur = t.createTextCursorByRange(t.End)
    cur.CharFontName = font
    t.insertString(cur, text, False)


def store(doc, url: str, fmt: str) -> None:
    doc.storeToURL(url, (prop("FilterName", FILTERS[fmt]), prop("Overwrite", True)))


def work_path(ctx) -> str:
    """LibreOffice's Work path, where its dialogs open and save first."""
    ps = ctx.ServiceManager.createInstanceWithContext("com.sun.star.util.PathSettings", ctx)
    return ps.Work


def one(desktop, folder: Path, fmt: str) -> list:
    """The failed checks for one format, each line said as it runs."""
    failed, url = [], uno.systemPathToFileUrl(str(folder / f"roundtrip.{fmt}"))
    say = lambda ok, what: (print(f"[OFFICE] {fmt} {what}: {'ok' if ok else 'FAILED'}", flush=True),
                           None if ok else failed.append(what))
    doc = desktop.loadComponentFromURL("private:factory/swriter", "_blank", 0, (prop("Hidden", True),))
    append(doc, check.ORIGINAL, check.FONTS[0])
    store(doc, url, fmt)
    doc.close(True)
    say((folder / f"roundtrip.{fmt}").stat().st_size > 0, "saved and closed")
    doc = load(desktop, url)
    say(check.text_kept(paragraphs(doc), [check.ORIGINAL]), "reopened with its text")
    append(doc, check.EDIT, check.FONTS[1])
    store(doc, url, fmt)
    doc.close(True)
    doc = load(desktop, url)
    say(check.text_kept(paragraphs(doc), [check.ORIGINAL, check.EDIT]), "edited, saved, reopened with both")
    say(para_fonts(doc) == list(check.FONTS), f"keeps its fonts {para_fonts(doc)}")
    pdf = folder / f"roundtrip.{fmt}.pdf"
    doc.storeToURL(uno.systemPathToFileUrl(str(pdf)), (prop("FilterName", "writer_pdf_Export"),))
    doc.close(True)
    missing = check.fonts_embedded(pdf.read_bytes())
    say(not missing, f"renders in its own fonts (missing {missing})" if missing else "renders in its own fonts")
    return failed


def run(folder: Path, soffice: str = "soffice", documents: bool = True) -> int:
    """`documents` says `folder` is the person's Documents, where the
    image's registry file points LibreOffice's Work path."""
    folder.mkdir(parents=True, exist_ok=True)
    pipe = f"nonos_office_{os.getpid()}"
    with tempfile.TemporaryDirectory() as profile:
        office = subprocess.Popen([soffice, "--headless", "--invisible", "--norestore", "--nologo",
                                   f"-env:UserInstallation={uno.systemPathToFileUrl(profile)}",
                                   f"--accept=pipe,name={pipe};urp;"],
                                  stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        try:
            ctx, desktop = connect(pipe)
            failed = [f for fmt in FILTERS for f in one(desktop, folder, fmt)]
            work = work_path(ctx)
            starts = work == uno.systemPathToFileUrl(str(folder)) or not documents
            print(f"[OFFICE] Work path {work}: {'ok' if starts else 'FAILED'}", flush=True)
            failed += [] if starts else ["Work path"]
            try:
                desktop.terminate()
            except Exception:
                pass
        finally:
            try:
                office.wait(timeout=30)
            except subprocess.TimeoutExpired:
                office.kill()
    print(f"[OFFICE] round trip: {'PASS' if not failed else 'FAIL'}", flush=True)
    return 0 if not failed else 1
