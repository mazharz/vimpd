use mpd::{Client, State};

#[derive(Default)]
pub struct Mpd {}

impl Mpd {
    fn get_connection(&self) -> Client {
        // TODO: make configurable, & handle unwrap
        Client::connect("127.0.0.1:6600").unwrap()
    }

    pub fn get_status(&self) -> bool {
        let mut connection = self.get_connection();
        let status = connection.status();
        let is_playing = match status {
            Ok(value) => value.state == State::Play,
            Err(_) => false,
        };
        is_playing
    }
}
