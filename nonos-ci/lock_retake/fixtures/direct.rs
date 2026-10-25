// A lock taken twice on one path, and a reader-writer lock taken for writing
// then reading: both never end.
static GATE: Mutex<u32> = Mutex::new(0);
static VOLUME: RwLock<u32> = RwLock::new(0);

fn twice() {
    let held = lock_responsive(&GATE);
    let again = lock_responsive(&GATE);
}

fn write_then_read() {
    let mut volume = crate::smp::write_responsive(&VOLUME);
    let peek = crate::smp::read_responsive(&VOLUME);
}

fn plain() {
    let held = GATE.lock();
    let again = super::gate::GATE.lock();
}
// expect: GATE taken again at line 8
// expect: VOLUME taken again at line 13
// expect: GATE taken again at line 18
