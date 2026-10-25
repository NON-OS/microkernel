"""Scratch: drive the running QEMU over QMP. qk.py <out.png> [action ...]
Actions: key:<qcode>[+<qcode>...] (one chord), text:<ascii>, wait:<s>,
click:<x>,<y> (absolute, 1920x1080). Ends with a screendump to <out.png>."""
import json, socket, sys, time
from PIL import Image

QMP = "/home/user/privatekernel/target/dev/tree/target/qemu/qmp.sock"
s = socket.socket(socket.AF_UNIX); s.connect(QMP); f = s.makefile("rw")
def cmd(c, **a):
    f.write(json.dumps({"execute": c, **({"arguments": a} if a else {})}) + "\n"); f.flush()
    while True:
        r = json.loads(f.readline())
        if "return" in r or "error" in r: return r
f.readline(); cmd("qmp_capabilities")
def chord(keys):
    r = cmd("send-key", keys=[{"type": "qcode", "data": k} for k in keys], **{"hold-time": 80})
    if "error" in r: print(r, flush=True)
    time.sleep(0.4)
NAMES = {" ": "spc", "-": "minus", ".": "dot", "/": "slash", "_": "shift+minus"}
out = sys.argv[1]
for a in sys.argv[2:]:
    kind, _, v = a.partition(":")
    if kind == "key":
        chord(v.split("+"))
    elif kind == "text":
        for ch in v:
            k = NAMES.get(ch, ch.lower())
            chord((["shift"] if ch.isupper() else []) + k.split("+"))
    elif kind == "wait":
        time.sleep(float(v))
    elif kind == "click":
        x, y = map(int, v.split(","))
        val = lambda p, n: round(p * 0x7FFF / (n - 1))
        cmd("input-send-event", events=[{"type": "abs", "data": {"axis": "x", "value": val(x, 1920)}},
                                        {"type": "abs", "data": {"axis": "y", "value": val(y, 1080)}}])
        time.sleep(0.3)
        for down in (True, False):
            cmd("input-send-event", events=[{"type": "btn", "data": {"down": down, "button": "left"}}])
            time.sleep(0.15)
time.sleep(1.5)
p = out.replace(".png", ".ppm"); cmd("screendump", filename=p); time.sleep(1.2)
Image.open(p).convert("RGB").resize((960, 540)).save(out)
print("saved", out, flush=True)
