// Held across a call that takes it, two calls down: the shape GATE had.
static GATE: Mutex<u32> = Mutex::new(0);

fn reclaim() {
    let _gate = lock_responsive(&GATE);
    tidy();
    super::dma_held::reclaim_dma();
}

fn tidy() {}

fn reclaim_dma() {
    deeper();
}

fn deeper() {
    let ready = {
        let _gate = lock_responsive(&GATE);
        1
    };
}
// expect: GATE held across reclaim_dma() at line 7, which takes it
