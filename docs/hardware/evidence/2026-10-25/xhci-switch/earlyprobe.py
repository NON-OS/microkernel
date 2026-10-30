"""Scratch: from the first frame on, move the absolute pointer between two
targets every few seconds and say, per move, whether the cursor appeared at
the target (exact), where a 1024x768 range puts it (default), or neither.

earlyprobe.py <serial> <qmp.sock> <outdir> <seconds>"""
import json, os, socket, sys, time
from PIL import Image, ImageChops

serial, qmp, outdir, total = sys.argv[1], sys.argv[2], sys.argv[3], int(sys.argv[4])
os.makedirs(outdir, exist_ok=True)
W, H = 1920, 1080
T = [(1600, 900), (400, 300)]
def text():
    try: return open(serial, errors="replace").read()
    except FileNotFoundError: return ""
while "first frame let through" not in text():
    time.sleep(0.5)
t0 = time.time()
s = socket.socket(socket.AF_UNIX); s.connect(qmp); f = s.makefile("rw")
def cmd(c, **a):
    f.write(json.dumps({"execute": c, **({"arguments": a} if a else {})}) + "\n"); f.flush()
    while True:
        r = json.loads(f.readline())
        if "return" in r or "error" in r: return r
f.readline(); cmd("qmp_capabilities")
def v(p, n): return max(0, min(0x7FFF, round(p * 0x7FFF / (n - 1))))
def changed(a, b, x, y):
    box = (max(0, x - 6), max(0, y - 6), min(W, x + 26), min(H, y + 30))
    d = ImageChops.difference(a.crop(box), b.crop(box)).convert("L").point(lambda p: 255 if p > 40 else 0)
    return d.getbbox() is not None
prev, i, where = None, 0, ""
while time.time() - t0 < total:
    tx, ty = T[i % 2]
    cmd("input-send-event", events=[{"type": "abs", "data": {"axis": "x", "value": v(tx, W)}},
                                    {"type": "abs", "data": {"axis": "y", "value": v(ty, H)}}])
    time.sleep(1.0)
    p = os.path.join(outdir, f"m{i:03}.ppm"); cmd("screendump", filename=p); time.sleep(1.0)
    im = Image.open(p).convert("RGB")
    if im.size != (W, H):
        print(f"{time.time()-t0:6.1f}s move {i}: display {im.size}, skipped", flush=True)
    elif prev is not None:
        gx, gy = tx * 1024 // W, ty * 768 // H
        where = ("exact" if changed(prev, im, tx, ty) else "") + ("default" if changed(prev, im, gx, gy) else "")
        print(f"{time.time()-t0:6.1f}s move {i} to {tx},{ty}: {where or 'neither'}", flush=True)
    prev = im if im.size == (W, H) else None
    os.remove(p) if i > 3 and where in ("exact",) else None
    i += 1
    time.sleep(1.0)
