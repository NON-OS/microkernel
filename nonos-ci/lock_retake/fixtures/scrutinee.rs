// A match or if-let on a take keeps its guard through the whole body (2021).
static GATE: Mutex<Option<u32>> = Mutex::new(None);

fn matched() {
    match GATE.lock().take() {
        Some(_) => takes(),
        None => {}
    }
}

fn if_let() {
    if let Some(n) = GATE.lock().as_ref() {
        let _ = n;
    } else {
        takes();
    }
}

fn takes() {
    let _gate = lock_responsive(&GATE);
}
// expect: GATE held across takes() at line 6, which takes it
// expect: GATE held across takes() at line 15, which takes it
