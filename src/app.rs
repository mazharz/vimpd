use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::Line,
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};

use crate::mpd::Mpd;

#[derive(Debug)]
pub struct App {
    is_running: bool,
    is_playing: bool,
}

impl App {
    pub fn new() -> App {
        let is_playing = Mpd::default().get_status();
        App {
            is_running: true,
            is_playing,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while self.is_running {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.is_running = false;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default();

        let status_text = if self.is_playing {
            "Mpd is jammin'!"
        } else {
            "Mpd is sleeping."
        };

        Paragraph::new(Line::from(status_text))
            .block(block)
            .render(area, buf)
    }
}
