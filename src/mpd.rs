use mpd::{Client, State};

#[derive(Debug)]
pub struct Mpd {
    pub is_playing: bool,
}

impl Mpd {
    pub fn new() -> Self {
        Self {
            is_playing: Self::get_status(),
        }
    }

    pub fn on_tick(&mut self) {
        self.sync_state();
    }

    pub fn toggle_play_pause(&mut self) {
        let mut connection = Self::get_connection();
        let _ = connection.toggle_pause();
        self.sync_state();
    }

    fn sync_state(&mut self) {
        self.is_playing = Self::get_status();
    }

    fn get_status() -> bool {
        let mut connection = Self::get_connection();
        let status = connection.status();
        let is_playing = match status {
            Ok(value) => value.state == State::Play,
            Err(_) => false,
        };
        is_playing
    }

    fn get_connection() -> Client {
        // TODO: make configurable, & handle unwrap
        Client::connect("127.0.0.1:6600").unwrap()
    }
}
