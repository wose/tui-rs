use std::f64::consts::PI;
use std::f64;

use unicode_width::UnicodeWidthStr;

use widgets::{Widget, Block};
use widgets::canvas::{Canvas, Line};
use buffer::Buffer;
use layout::Rect;
use style::Style;

pub struct Donut<'a> {
    block: Option<Block<'a>>,
    percent: u16,
    label: Option<&'a str>,
    style: Style,
    inner_style: Style,
    label_style: Style,
}

impl <'a> Default for Donut<'a> {
    fn default() -> Donut<'a> {
        Donut {
            block: None,
            percent: 0,
            label: None,
            style: Default::default(),
            inner_style: Default::default(),
            label_style: Default::default(),
        }
    }
}

impl<'a> Donut<'a> {
    pub fn block(&mut self, block: Block<'a>) -> &mut Donut<'a> {
        self.block = Some(block);
        self
    }

    pub fn percent(&mut self, percent: u16) -> &mut Donut<'a> {
        self.percent = percent;
        self
    }

    pub fn label(&mut self, label: &'a str) -> &mut Donut<'a> {
        self.label = Some(label);
        self
    }

    pub fn style(&mut self, style: Style) -> &mut Donut<'a> {
        self.style = style;
        self
    }

    pub fn inner_style(&mut self, style: Style) -> &mut Donut<'a> {
        self.inner_style = style;
        self
    }

    pub fn label_style(&mut self, style: Style) -> &mut Donut<'a> {
        self.label_style = style;
        self
    }

}

impl<'a> Widget for Donut<'a> {
    fn draw(&self, area: &Rect, buf: &mut Buffer) {
        let width = 30;
        let radius = 50;
        let canvas_x = 100;
        let canvas_y = 1.83 * 100.0 * area.height as f64 / area.width as f64;
        let center_x = canvas_x as f64 / 2.0;
        let center_y = canvas_y as f64 / 2.0;

        Canvas::default()
            .x_bounds([0.0, canvas_x as f64])
            .y_bounds([0.0, canvas_y as f64])
            .paint(|ctx| {
                let mut first_point = true;
                let mut s = 0u16;
                let mut last_x = 0.0f64;
                let mut last_y = 0.0f64;
                while s < radius {
                    if s < (radius - width) {
                        s += 1;
                        continue;
                    }

                    let slice = 2.0 * PI / 370.0;
                    for point in 0..370u16 {
                        let si = point as f64 - 90.0;
                        let a = slice * si;
                        let x = center_x as f64 + s as f64 * a.cos();
                        let y = center_y as f64 + s as f64 * a.sin();
                        if !first_point && point as f64 <= self.percent as f64 * 3.6 {
                            ctx.draw(&Line {
                                x1: last_x.round(),
                                y1: last_y.round(),
                                x2: x.round(),
                                y2: y.round(),
                                color: self.style.fg,
                            });
                        } else {
                            first_point = false;
                        }
                        last_x = x;
                        last_y = y;
                    }
                    s += 1;
                }
            })
            .draw(&area, buf);

        let inner_text = format!("{:>3} %", self.percent);

        buf.set_string(
            area.left() + area.width / 2 - inner_text.width() as u16 / 2 - 1,
            area.top() + area.height / 2 - 1,
            &inner_text,
            &self.inner_style);

        if let Some(label) = self.label {
            let label: String = label.into();

            buf.set_string(
                area.left() + area.width / 2 - label.width() as u16 / 2,
                area.bottom() - 1,
                &label,
                &self.label_style);
        };
    }
}
