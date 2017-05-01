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
use tui::layout::{Group, Direction, Size};
use tui::style::{Style, Color};

struct App {
    progress1: u16,
    progress2: u16,
    progress3: u16,
    progress4: u16,
}

impl App {
    fn new() -> App {
        App {
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
    draw(&mut terminal, &app);

    loop {
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

    let size = t.size().unwrap();

    Group::default()
        .direction(Direction::Horizontal)
        .margin(2)
        .sizes(&[Size::Percent(25), Size::Percent(25), Size::Percent(25), Size::Percent(25)])
        .render(t, &size, |t, chunks| {
            Donut::default()
                .block(Block::default().title("Donut1").borders(border::ALL))
                .style(Style::default().fg(Color::Yellow))
                .label_style(Style::default().fg(Color::Cyan))
                .percent(app.progress1)
                .label("EXT")
                .render(t, &chunks[0]);
            Donut::default()
                .block(Block::default().title("Donut2").borders(border::ALL))
                .percent(app.progress2)
                .label("BED")
                .label_style(Style::default().fg(Color::Cyan))
                .render(t, &chunks[1]);
            Donut::default()
                .block(Block::default().title("Donut2").borders(border::ALL))
                .style(Style::default().fg(Color::Cyan))
                .percent(app.progress3)
                .label("FAN1")
                .label_style(Style::default().fg(Color::Cyan))
                .render(t, &chunks[2]);
            Donut::default()
                .block(Block::default().title("Donut3").borders(border::ALL))
                .style(Style::default().fg(Color::Cyan))
                .percent(app.progress4)
                .label("FAN2")
                .label_style(Style::default().fg(Color::Cyan))
                .render(t, &chunks[3]);
        });

    t.draw().unwrap();
}
