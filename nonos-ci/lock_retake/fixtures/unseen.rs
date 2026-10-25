// What the check does not follow, by design: a name two functions share, a
// lock name two statics share, a method call, and words in comments and
// strings ("GATE.lock()" here, or "{" which is not a brace).
static GATE: Mutex<u32> = Mutex::new(0);
static STATE: Mutex<u32> = Mutex::new(0);
static STATE: RwLock<u32> = RwLock::new(0);

fn shared_name() {
    let _gate = lock_responsive(&GATE);
    helper();
    self.takes_gate();
    let note = "GATE.lock() {";
    // GATE.lock();
    let _ = '{';
}

mod other {
    fn helper() {}
}

fn helper() {
    let _gate = lock_responsive(&GATE);
}

fn states() {
    let a = STATE.lock();
    let b = STATE.lock();
}
