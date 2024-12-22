use std::{
    io,
    time::{Duration, Instant},
};

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
    mpd: Mpd,
}

impl App {
    pub fn new() -> App {
        App {
            is_running: true,
            mpd: Mpd::new(),
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        // TODO: make configurable
        let tick_rate = 1000;
        let tick_rate = Duration::from_millis(tick_rate);

        let mut last_tick = Instant::now();

        loop {
            if !self.is_running {
                return Ok(());
            }
            terminal.draw(|frame| self.draw(frame))?;

            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout)? {
                self.handle_events()?;
            }

            if last_tick.elapsed() >= tick_rate {
                self.on_tick();
                last_tick = Instant::now();
            }
        }
    }

    fn on_tick(&mut self) {
        self.mpd.on_tick();
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

        let status_text = if self.mpd.is_playing {
            "Mpd is jammin'!"
        } else {
            "Mpd is sleeping."
        };

        Paragraph::new(Line::from(status_text))
            .block(block)
            .render(area, buf)
    }
}
