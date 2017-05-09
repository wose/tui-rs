extern crate tui;
extern crate termion;

use std::io;
use std::thread;
use std::time;
use std::sync::mpsc;

use termion::event;
use termion::input::TermRead;

use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{Widget, Block, border, Donut};
use tui::layout::{Group, Direction, Size, Rect};
use tui::style::{Style, Color};

struct App {
    size: Rect,
    progress1: u16,
    progress2: u16,
    progress3: u16,
    progress4: u16,
}

impl App {
    fn new() -> App {
        App {
            size: Rect::default(),
            progress1: 0,
            progress2: 0,
            progress3: 0,
            progress4: 0,
        }
    }

    fn advance(&mut self) {
        self.progress1 += 5;
        if self.progress1 > 100 {
            self.progress1 = 0;
        }
        self.progress2 += 10;
        if self.progress2 > 100 {
            self.progress2 = 0;
        }
        self.progress3 += 1;
        if self.progress3 > 100 {
            self.progress3 = 0;
        }
        self.progress4 += 3;
        if self.progress4 > 100 {
            self.progress4 = 0;
        }
    }
}

enum Event {
    Input(event::Key),
    Tick,
}

fn main() {
    // Terminal initialization
    let backend = TermionBackend::new().unwrap();
    let mut terminal = Terminal::new(backend).unwrap();

    // Channels
    let (tx, rx) = mpsc::channel();
    let input_tx = tx.clone();
    let clock_tx = tx.clone();

    // Input
    thread::spawn(move || {
        let stdin = io::stdin();
        for c in stdin.keys() {
            let evt = c.unwrap();
            input_tx.send(Event::Input(evt)).unwrap();
            if evt == event::Key::Char('q') {
                break;
            }
        }
    });

    // Tick
    thread::spawn(move || {
        loop {
            clock_tx.send(Event::Tick).unwrap();
            thread::sleep(time::Duration::from_millis(500));
        }
    });

    // App
    let mut app = App::new();

    // First draw call
    terminal.clear().unwrap();
    terminal.hide_cursor().unwrap();
    app.size = terminal.size().unwrap();
    draw(&mut terminal, &app);

    loop {
        let size = terminal.size().unwrap();
        if size != app.size {
            terminal.resize(size).unwrap();
            app.size = size;
        }

        let evt = rx.recv().unwrap();
        match evt {
            Event::Input(input) => {
                if input == event::Key::Char('q') {
                    break;
                }
            }
            Event::Tick => {
                app.advance();
            }
        }
        draw(&mut terminal, &app);
    }

    terminal.show_cursor().unwrap();
}

fn draw(t: &mut Terminal<TermionBackend>, app: &App) {

    Group::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .sizes(&[Size::Percent(50), Size::Percent(50)])
        .render(t, &app.size, |t, chunks| {
            Block::default()
                .borders(border::ALL)
                .title("Temperature")
                .render(t, &chunks[0]);

            Group::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .sizes(&[Size::Percent(50), Size::Percent(50)])
                .render(t, &chunks[0], |t, chunks| {
                    Donut::default()
                        .fg(Color::Yellow)
                        .label_style(Style::default().fg(Color::Cyan))
                        .percent(app.progress1)
                        .label("EXT")
                        .render(t, &chunks[0]);
                    Donut::default()
                        .percent(app.progress2)
                        .label("BED")
                        .label_style(Style::default().fg(Color::Cyan))
                        .render(t, &chunks[1]);
                });
            Block::default()
                .borders(border::ALL)
                .title("Cooling")
                .render(t, &chunks[1]);

            Group::default()
                .direction(Direction::Horizontal)
                .sizes(&[Size::Percent(50), Size::Percent(50)])
                .margin(1)
                .render(t, &chunks[1], |t, chunks| {
                    Donut::default()
                        .fg(Color::LightCyan)
                        .percent(app.progress3)
                        .label("FAN1")
                        .label_style(Style::default().fg(Color::Cyan))
                        .render(t, &chunks[0]);
                    Donut::default()
                        .fg(Color::LightCyan)
                        .percent(app.progress4)
                        .label("FAN2")
                        .label_style(Style::default().fg(Color::Cyan))
                        .render(t, &chunks[1]);
                });
        });
;
    t.draw().unwrap();
}
