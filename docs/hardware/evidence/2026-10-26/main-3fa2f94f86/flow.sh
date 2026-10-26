#!/usr/bin/env bash
# Scratch: lead/allows flow. $1 name, $2 lane, $3 "full" (Settings, About, Files, USB keyboard) or "save".
S=/tmp/claude-0/-home-user-privatekernel/0d9b5db8-715b-5a9c-b79c-b573aa1baa2a/scratchpad
n=$1; lane=$2; mode=$3; O=$S/m3/boot; mkdir -p $O; rm -f $S/$n.serial
extra=""; [ "$mode" = full ] && { extra="--lane usb-kbd"; export SCRATCH_NO_I8042=1; }
(BOOT_TIMEOUT=1200 $S/boot.sh $n --fresh --lane $lane --lane usb-tablet $extra > $O/qemu-$n.out 2>&1 &)
for i in $(seq 1 400); do grep -aq "first frame let through" $S/$n.serial 2>/dev/null && break; sleep 3; done
sleep 60; cd $S
# QEMU aborts on input-send-event with a device id under -display none; send-key goes to the active keyboard
keys="$(for i in 1 2 3 4 5 6 7 8; do echo key:ret wait:2; done) key:up key:up key:ret wait:2 $(for i in 1 2 3 4; do echo key:ret wait:2; done)"
python3 qk2.py $O/$n-desktop.png $keys wait:50
python3 qk2.py $O/$n-editor.png click:1062,620 wait:2 click:584,1020 wait:6 click:900,260 text:$n key:ctrl+s wait:2 text:/tmp/$n.txt key:ret wait:3
if [ "$mode" = full ]; then
  python3 qk2.py $O/$n-settings.png click:650,1020 wait:8
  python3 qk2.py $O/$n-about.png click:784,1020 wait:8
  python3 qk2.py $O/$n-files.png click:517,1020 wait:8
fi
python3 - <<P
import json,socket
s=socket.socket(socket.AF_UNIX); s.connect("/home/user/privatekernel/target/dev/tree/target/qemu/qmp.sock"); f=s.makefile("rw")
f.readline(); f.write('{"execute":"qmp_capabilities"}\n{"execute":"quit"}\n'); f.flush()
P
until ! ps -eo comm | grep -q qemu; do sleep 2; done; echo done $n
