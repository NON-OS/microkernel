//! What was published and when, kept beside the hand-off it publishes, so a restart keeps the
//! timers, and a new spend, which empties the hand-off, starts them again.

/// The file of the record, inside the hand-off folder.
pub const RECORD: &str = "published";

/// Where a spend was handed. Each leaves through Tor.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Route {
    Lander,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Published {
    /// Seconds since 1970 of the first and the latest publication, and how many there were.
    pub first: u64,
    pub last: u64,
    pub times: u32,
    pub route: Route,
    /// The id the lander gave the hand-off, when it was the lander.
    pub id: Option<String>,
    /// Which lander of the pool took it, by its place in the list.
    pub lander: usize,
}

impl Published {
    pub fn to_text(&self) -> String {
        let id = self.id.as_deref().unwrap_or("");
        format!("{} {} {} lander {id} {}\n", self.first, self.last, self.times, self.lander)
    }

    /// The record read strictly: a field that does not parse refuses the whole of it.
    pub fn from_text(text: &str) -> Option<Published> {
        let mut parts = text.trim_end_matches('\n').split(' ');
        let mut number = || parts.next().and_then(|p| p.parse::<u64>().ok());
        let (first, last, times) = (number()?, number()?, number()?);
        let route = match parts.next()? {
            "lander" => Route::Lander,
            _ => return None,
        };
        let id = parts.next().filter(|s| !s.is_empty()).map(str::to_string);
        let fine = id
            .as_deref()
            .is_none_or(|i| i.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_'));
        let times = u32::try_from(times).ok()?;
        let lander = match parts.next() {
            None => 0,
            Some(at) => at.parse::<usize>().ok()?,
        };
        (fine && parts.next().is_none() && first <= last).then_some(Published {
            first,
            last,
            times,
            route,
            id,
            lander,
        })
    }
}
