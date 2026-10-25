// Every guard here is gone before the next take: no finding.
static GATE: Mutex<u32> = Mutex::new(0);

fn temporaries() {
    let n = GATE.lock().count_ones();
    if GATE.lock().is_power_of_two() {
        takes();
    }
    let _ = GATE.lock();
    takes();
    let held = GATE.lock();
    drop(held);
    takes();
    let tried = GATE.try_lock();
    takes();
}

fn takes() {
    let _gate = lock_responsive(&GATE);
}
