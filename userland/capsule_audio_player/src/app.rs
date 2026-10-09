// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

extern crate alloc;

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use nonos_app_skeleton::input::text::is_paste;
use nonos_app_skeleton::{
    App, AppManifest, EventOutcome, InputEvent, InputKind, PaintBuffer, WindowKind,
};

use crate::library::{next_rescan_ms, same_path, Library, Queue, Track};
use crate::model::TrackMeta;
use crate::track::{begin_track, blank, Loaded, Loading};
use crate::transport::{FeedSink, LateSink, State, Transport};
use crate::trouble::{library_trouble, notice, track_trouble};
use crate::ui;
use crate::ui::search_key::{search_key, SearchKey};
use crate::ui::{Action, Frame, Scene, UiState, View};
use crate::waveform::Waveform;
use nonos_audio_proto::MasterVolume;

const WINDOW_ID: u32 = 0x5245_534E;
const WINDOW_W: u32 = 1440;
const WINDOW_H: u32 = 900;
const INPUT_MASK: u32 = (1 << 0) | (1 << 3) | (1 << 4) | (1 << 5);
const FRAME_TICKS: u32 = 5;
/// How long after the last track ends its stream is given back: the server
/// still holds up to 0x4000 samples (170 ms at 48 kHz) of it to play out,
/// and closing drops them.
const DRAIN_MS: i64 = 500;
/// Time one tick may spend reading tags.
const TAG_MS: i64 = 12;
/// How often the window asks the audio service for the system volume.
const MASTER_EVERY_MS: i64 = 2_000;
/// How often a window with a download running and nothing playing ticks.
const FETCH_TICK_MS: i64 = 200;

mod fetched;
mod open_arg;

pub struct PlayerApp {
    transport: Transport,
    meta: TrackMeta,
    waveform: Waveform,
    library: Library,
    queue: Queue,
    shuffle: bool,
    repeat: bool,
    ui: UiState,
    frame: Frame,
    ticks: u32,
    dims: (u32, u32),
    retries: u8,
    /// When an empty library may next be listed, on the uptime clock
    /// (library/rescan_gap.rs).
    rescan_ms: i64,
    /// Why the selected track would not load, shown until another loads.
    trouble: Option<&'static str>,
    /// The track being read and decoded, a tick at a time (`loading.rs`).
    loading: Option<Loading>,
    /// A track handed over from outside the /audio library (`open_arg.rs`),
    /// playing in place of the queue's: kept apart so the library and the
    /// queue's indices stay those of /audio.
    outside: Option<Track>,
    /// When to next ask the desktop shell for a handed-over file.
    arg_due_ms: i64,
    /// When to give the stream back, once play has ended with nothing after.
    release_ms: Option<i64>,
    /// The download's line as last painted, so a tick repaints only when it
    /// moves on (`fetched.rs`).
    fetch_line: String,
    /// Every link asked for, and where each stands (`fetch/list.rs`).
    downloads: crate::fetch::list::Downloads,
    /// The row the download worker is on.
    fetching: Option<u32>,
    /// The system's master volume as the audio service last said it; None
    /// while the service has not answered, when the player's own gain
    /// stands in (`volume.rs`).
    master: Option<MasterVolume>,
    /// When to ask for it again, so a change made with the volume keys
    /// shows on the slider.
    master_due_ms: i64,
}

/// A load begun, and what the bar says about it: the reason when the track
/// could not even be opened.
fn began(result: Result<Loading, &'static str>) -> (Option<Loading>, Option<&'static str>) {
    match result {
        Ok(loading) => (Some(loading), None),
        Err(err) => (None, Some(track_trouble(err))),
    }
}

impl PlayerApp {
    pub fn new() -> PlayerApp {
        // Found now or when play is next pressed (transport/late_sink.rs): a
        // track still loads and shows its waveform without it, and play says
        // there is no audio output rather than sitting in Playing forever.
        let sink: Box<dyn FeedSink> = Box::new(LateSink::new());
        let mut transport = Transport::new(sink);
        let mut meta = TrackMeta { title: String::new(), artist: String::new(), format: String::new() };
        let library = Library::scan();
        let mut queue = Queue::new();
        for i in 0..library.tracks.len() {
            queue.enqueue(i);
        }
        // Begun here and carried on from the ticks, so the window is up and
        // painting while the first track reads and decodes.
        let (loading, trouble) = match queue.current().and_then(|i| library.get(i)) {
            Some(t) => began(begin_track(&mut transport, &mut meta, t, false)),
            // Nothing ships with the system: an empty library waits for
            // the person's own music, and the page says how to add it.
            None => (None, None),
        };
        let library_error = library.error;
        PlayerApp {
            transport,
            meta,
            waveform: blank(),
            library,
            queue,
            shuffle: false,
            repeat: false,
            ui: UiState::new(),
            frame: Frame::new(),
            ticks: 0,
            dims: (WINDOW_W, WINDOW_H),
            retries: 24,
            rescan_ms: next_rescan_ms(library_error, nonos_libc::mk_uptime_ms()),
            trouble,
            loading,
            outside: None,
            arg_due_ms: 0,
            release_ms: None,
            fetch_line: String::new(),
            downloads: crate::fetch::list::Downloads::new(),
            fetching: None,
            master: crate::audio_client::master(),
            master_due_ms: 0,
        }
    }

    /// Set the system volume, and show what the service says is in force.
    fn set_master(&mut self, want: MasterVolume) {
        if let Some(now) = crate::audio_client::set_master(want) {
            self.master = Some(now);
        }
        self.master_due_ms = nonos_libc::mk_uptime_ms() + MASTER_EVERY_MS;
    }

    /// Ask for the system volume every few seconds, for a change made with
    /// the volume keys. True when it changed.
    fn poll_master(&mut self) -> bool {
        let now = nonos_libc::mk_uptime_ms();
        if now < self.master_due_ms {
            return false;
        }
        self.master_due_ms = now + MASTER_EVERY_MS;
        let heard = crate::audio_client::master();
        if heard.is_none() || heard == self.master {
            return false;
        }
        self.master = heard;
        true
    }

    fn rescan(&mut self) -> bool {
        if self.retries == 0 || !self.library.tracks.is_empty() {
            return false;
        }
        let now = nonos_libc::mk_uptime_ms();
        if now < self.rescan_ms {
            return false;
        }
        self.retries -= 1;
        self.library = Library::scan();
        self.rescan_ms = next_rescan_ms(self.library.error, nonos_libc::mk_uptime_ms());
        if self.library.tracks.is_empty() {
            return false;
        }
        self.retries = 0;
        self.queue = Queue::new();
        for i in 0..self.library.tracks.len() {
            self.queue.enqueue(i);
        }
        // A handed-over file playing from outside the library goes on: if it
        // is one of the tracks just listed, it becomes that entry.
        if let Some(t) = self.outside.as_ref() {
            let listed = self.library.tracks.iter().position(|l| same_path(&l.path, &t.path));
            if let Some(i) = listed {
                self.queue.focus(i);
                self.outside = None;
            }
            return true;
        }
        let Self { library, queue, transport, meta, waveform, trouble, loading, .. } = self;
        if let Some(t) = queue.current().and_then(|i| library.get(i)) {
            (*loading, *trouble) = began(begin_track(transport, meta, t, false));
            *waveform = blank();
        }
        true
    }

    fn rows(&self) -> Vec<usize> {
        match self.ui.view {
            View::Library => ui::screen::rows_for(&self.library, &self.queue, self.ui.lib_tab),
            // An address is not words to match: the page is the download's.
            View::Search if crate::fetch::is_address(&self.ui.query) => Vec::new(),
            View::Search => ui::state::filtered(&self.library, &self.ui.query),
            _ => Vec::new(),
        }
    }

    fn select(&mut self, i: usize) {
        self.queue.focus(i);
        self.load_index(i);
    }

    fn act(&mut self, a: Action) -> EventOutcome {
        match a {
            Action::Go(v) => self.ui.go(v),
            Action::Select(i) => self.select(i),
            Action::LibTab(t) => {
                self.ui.lib_tab = t;
                self.ui.scroll = 0;
            }
            Action::Download => self.download(),
            Action::Fetched(id, act) => self.download_act(id, act),
            Action::ClearDownloads => self.clear_downloads(),
            Action::ClearQuery => {
                self.ui.query.clear();
                self.ui.scroll = 0;
            }
            Action::Ctl(ui::Control::Prev) => self.prev_track(),
            Action::Ctl(ui::Control::Next) => self.next_track(),
            Action::Ctl(ui::Control::Shuffle) => self.toggle_shuffle(),
            Action::Ctl(ui::Control::Repeat) => self.repeat = !self.repeat,
            Action::Ctl(ui::Control::Volume(p)) if self.master.is_some() => {
                self.set_master(crate::volume::at_permille(p))
            }
            Action::Ctl(ui::Control::Mute) if self.master.is_some() => {
                let now = self.master.unwrap_or(MasterVolume::FULL);
                self.set_master(crate::volume::toggled(now))
            }
            Action::Ctl(c) => ui::event::apply(&mut self.transport, c),
        }
        EventOutcome::Repaint
    }

    fn hit(&self, x: i32, y: i32) -> Option<Action> {
        let rows = self.rows();
        let lists = ui::Lists {
            rows: &rows,
            queue: self.queue.items(),
            n: self.library.tracks.len(),
            downloads: self.downloads.rows(),
        };
        ui::hit(&self.ui, self.dims, &lists, x, y)
    }

    fn on_click(&mut self, x: i32, y: i32) -> EventOutcome {
        match self.hit(x, y) {
            Some(a) => self.act(a),
            None => EventOutcome::Idle,
        }
    }

    fn on_move(&mut self, x: i32, y: i32) -> EventOutcome {
        let h = match self.hit(x, y) {
            Some(Action::Select(i)) => Some(i),
            _ => None,
        };
        if h == self.ui.hover {
            return EventOutcome::Idle;
        }
        self.ui.hover = h;
        EventOutcome::Repaint
    }

    fn on_wheel(&mut self, delta: i32) -> EventOutcome {
        let page = ui::geometry::page(&ui::geometry::shell(self.dims.0, self.dims.1));
        let len = match self.ui.view {
            View::Downloads => self.downloads.rows().len(),
            _ => self.rows().len(),
        };
        let visible = match self.ui.view {
            View::Downloads => ui::screen::downloads_visible(page),
            View::Search => ui::screen::search_visible(page),
            _ => ui::screen::lib_visible(page),
        };
        self.ui.scroll_by(if delta > 0 { -3 } else { 3 }, len, visible);
        EventOutcome::Repaint
    }

    /// On the Search page, typed keys edit the query and Enter plays the
    /// first result; every other key, and every key elsewhere, is the
    /// transport's.
    fn on_key(&mut self, code: u32) -> EventOutcome {
        if self.ui.view == View::Search {
            match search_key(&mut self.ui.query, code) {
                SearchKey::Edited => {
                    self.ui.scroll = 0;
                    return EventOutcome::Repaint;
                }
                SearchKey::PlayFirst if crate::fetch::is_address(&self.ui.query) => {
                    self.download();
                    return EventOutcome::Repaint;
                }
                SearchKey::PlayFirst => {
                    if let Some(&i) = self.rows().first() {
                        self.select(i);
                    }
                    return EventOutcome::Repaint;
                }
                SearchKey::Pass => {}
            }
        }
        if let Some(s) = ui::shortcut::shortcut(code) {
            self.shortcut(s);
            return EventOutcome::Repaint;
        }
        ui::event::key(&mut self.transport, code)
    }

    fn shortcut(&mut self, s: ui::shortcut::Shortcut) {
        use ui::shortcut::Shortcut;
        match s {
            Shortcut::VolumeUp | Shortcut::VolumeDown => {
                let up = s == Shortcut::VolumeUp;
                match self.master {
                    Some(now) => self.set_master(crate::volume::stepped(now, up)),
                    None => {
                        let step = (1 << 15) * i32::from(crate::volume::STEP) / 100;
                        let q = self.transport.volume_q15() + if up { step } else { -step };
                        self.transport.set_volume(q);
                    }
                }
            }
            Shortcut::Mute => {
                self.act(Action::Ctl(ui::Control::Mute));
            }
            Shortcut::Next => self.next_track(),
            Shortcut::Prev => self.prev_track(),
            Shortcut::Shuffle => self.toggle_shuffle(),
            Shortcut::Repeat => self.repeat = !self.repeat,
            Shortcut::Search => self.ui.go(View::Search),
        }
    }

    fn on_eof(&mut self) {
        if self.outside.is_some() {
            if self.repeat {
                self.load_outside();
            } else {
                self.ended();
            }
            return;
        }
        if self.repeat || self.queue.has_next() {
            self.next_track();
        } else {
            self.ended();
        }
    }

    /// Play has stopped for good: give the stream back once what the server
    /// holds has played out, so a window left open at the end of its queue
    /// does not keep one of the service's slots. Play opens a fresh one.
    fn ended(&mut self) {
        self.release_ms = Some(nonos_libc::mk_time_millis().saturating_add(DRAIN_MS));
    }

    fn release_if_due(&mut self) {
        let Some(at) = self.release_ms else { return };
        if self.transport.state() == State::Playing || self.loading.is_some() {
            self.release_ms = None;
            return;
        }
        if nonos_libc::mk_time_millis() >= at {
            self.release_ms = None;
            self.transport.release();
        }
    }

    fn toggle_shuffle(&mut self) {
        self.shuffle = !self.shuffle;
        if self.shuffle {
            self.queue.shuffle(nonos_libc::mk_time_millis() as u64);
        } else {
            self.queue.restore_order();
        }
    }

    fn prev_track(&mut self) {
        if self.back_to_library() {
            return;
        }
        if let Some(i) = self.queue.back() {
            self.load_index(i);
        }
    }

    fn next_track(&mut self) {
        if self.back_to_library() {
            return;
        }
        if self.repeat {
            if let Some(i) = self.queue.current() {
                self.load_index(i);
            }
            return;
        }
        if let Some(i) = self.queue.advance() {
            self.load_index(i);
        }
    }

    /// Begin loading the track at `i`, to play once it has loaded. A load
    /// still running for another track is dropped, its file closed.
    fn load_index(&mut self, i: usize) {
        if let Some(t) = self.library.get(i) {
            self.outside = None;
            (self.loading, self.trouble) =
                began(begin_track(&mut self.transport, &mut self.meta, t, true));
            self.waveform = blank();
        }
    }

    /// Previous and Next from a handed-over track go back to the library's
    /// current track, which is where the queue still stands; with an empty
    /// library they leave the handed-over track as it is. True when the
    /// handed-over track was playing.
    fn back_to_library(&mut self) -> bool {
        if self.outside.is_none() {
            return false;
        }
        if let Some(i) = self.queue.current() {
            self.load_index(i);
        }
        true
    }

    /// Begin loading the handed-over track again, to play once loaded.
    fn load_outside(&mut self) {
        if let Some(t) = self.outside.as_ref() {
            (self.loading, self.trouble) =
                began(begin_track(&mut self.transport, &mut self.meta, t, true));
            self.waveform = blank();
        }
    }

    /// Read the loaded track's own cover from its tags, once, for the rail,
    /// the bar and Now Playing; a track without one keeps generated art.
    fn show_cover(&mut self) {
        let track = match &self.outside {
            Some(t) => Some(t),
            None => self.queue.current().and_then(|i| self.library.get(i)),
        };
        let Some(t) = track else { return };
        let picture = crate::library::cover_art::read(nonos_libc::mk_getpid(), &t.path);
        // The rail names the track by the title the bar shows, its tags' when
        // they were read.
        let title = if self.meta.title.is_empty() { t.title.as_str() } else { self.meta.title.as_str() };
        crate::ui::art::picture::set_current(&t.path, title, picture);
    }

    /// One tick of the load in progress.
    fn step_load(&mut self) {
        let Some(loading) = self.loading.as_mut() else { return };
        match loading.step(&mut self.transport, &mut self.meta) {
            Loaded::More => {}
            Loaded::Ready(wave) => {
                let play = loading.play;
                self.loading = None;
                self.waveform = wave;
                // Its length is known now (an MP3's from the load's pass),
                // so the library's Time column can say it.
                if self.outside.is_none() {
                    let ms = self.transport.frames_to_ms(self.transport.dur_frames());
                    let current = self.queue.current().and_then(|i| self.library.tracks.get_mut(i));
                    if let Some(track) = current {
                        track.dur_ms = ms;
                    }
                }
                if play {
                    self.transport.play();
                }
                self.show_cover();
            }
            Loaded::Failed(why) => {
                self.loading = None;
                self.trouble = Some(track_trouble(why));
            }
        }
    }
}

impl App for PlayerApp {
    fn manifest(&self) -> AppManifest {
        AppManifest {
            title: b"Resonare",
            window_id: WINDOW_ID,
            kind: WindowKind::Normal,
            initial_x: 60,
            initial_y: 40,
            width: WINDOW_W,
            height: WINDOW_H,
            input_kind_mask: INPUT_MASK,
        }
    }

    fn on_event(&mut self, event: InputEvent) -> EventOutcome {
        match event.kind {
            InputKind::ButtonDown => self.on_click(event.x, event.y),
            InputKind::PointerAbs => self.on_move(event.x, event.y),
            InputKind::Wheel => self.on_wheel(event.delta_y),
            InputKind::KeyDown if self.ui.view == View::Search && is_paste(&event) => {
                self.paste();
                EventOutcome::Repaint
            }
            InputKind::KeyDown => self.on_key(event.code),
            _ => EventOutcome::Idle,
        }
    }

    fn paint(&mut self, fb: &mut PaintBuffer) {
        self.rescan();
        self.dims = (fb.width, fb.height);
        let mut v = self.transport.view(&self.meta);
        if let Some(loading) = &self.loading {
            v.artist = match loading.progress() {
                Some(p) => alloc::format!("Loading {p}%"),
                None => String::from("Loading..."),
            };
        } else if !self.fetch_line.is_empty() {
            v.artist = self.fetch_line.clone();
        }
        if let Some(m) = self.master {
            v.volume_q15 = crate::volume::q15(m);
            v.muted = m.muted;
        }
        v.shuffle = self.shuffle;
        v.repeat = self.repeat;
        // A handed-over track has the bar while it is up: an empty library is
        // not news about it.
        let library = match self.outside {
            Some(_) => None,
            None => library_trouble(self.library.error, self.library.tracks.len()),
        };
        // A folder that could not be read is a fault, said in red; an empty
        // one is where everyone starts, said as the line under the title.
        let (fault, hint) = match self.library.error {
            Some(_) => (library, None),
            None => (None, library),
        };
        v.notice = notice(self.trouble, v.notice, fault);
        if let (true, Some(h)) = (v.title.is_empty() && v.artist.is_empty(), hint) {
            v.artist = String::from(h);
        }
        let rows = self.rows();
        let playing = if self.outside.is_some() { None } else { self.queue.current() };
        let id = match &self.outside {
            Some(t) => t.path.as_str(),
            None => playing.and_then(|i| self.library.get(i)).map_or("", |t| t.path.as_str()),
        };
        let scene = Scene {
            lib: &self.library,
            queue: &self.queue,
            rows: &rows,
            view: &v,
            wave: &self.waveform,
            playing,
            id,
            level: self.transport.level(),
            downloads: self.downloads.rows(),
        };
        self.frame.paint(fb, &self.ui, &scene);
    }

    fn on_tick(&mut self) -> bool {
        // Followed on every tick whatever else the tick does, so a load or a
        // rescan never leaves a finished download unplayed.
        // A few files' tags a tick, so a large library fills in while the
        // window stays live and play goes on (`library/tag_pass.rs`).
        let tagged = self.library.untagged() && self.library.tag_some(TAG_MS);
        let fetched = self.poll_fetch() | self.poll_master() | tagged;
        self.tick_rest() || fetched
    }

    fn busy(&self) -> bool {
        ui::control::playing(&self.transport)
            || self.loading.is_some()
            || self.fetching.is_some()
            || self.library.untagged()
    }

    fn tick_interval_ms(&self) -> i64 {
        if ui::control::playing(&self.transport) || self.loading.is_some() || self.library.untagged() {
            10
        } else if self.fetching.is_some() {
            // The bar's progress, and the outcome picked up within a beat.
            FETCH_TICK_MS
        } else {
            500
        }
    }
}

impl PlayerApp {
    fn tick_rest(&mut self) -> bool {
        if self.rescan() {
            return true;
        }
        if self.poll_open_arg() {
            return true;
        }
        if self.loading.is_some() {
            self.step_load();
            return true;
        }
        self.release_if_due();
        let was = self.transport.state();
        self.transport.pump();
        if was == State::Playing && self.transport.state() == State::Stopped {
            self.on_eof();
        }
        // The pump pauses only when the sink failed; repaint so the transport
        // bar shows why instead of a play button that silently did nothing.
        if was == State::Playing && self.transport.state() == State::Paused {
            return true;
        }
        if !ui::control::playing(&self.transport) {
            return false;
        }
        self.ticks += 1;
        if self.ticks < FRAME_TICKS {
            return false;
        }
        self.ticks = 0;
        true
    }
}
