use crate::render::{Text, TextRenderer, WidgetRenderContext};
use crate::util;
use crate::util::{BoundedQueue, Position};
use std::cell::Cell;
use std::sync::Mutex;
use windows::Win32::Graphics::Direct2D::{ID2D1HwndRenderTarget, ID2D1SolidColorBrush};

pub mod cpu;
pub mod gpu;
pub mod ram;

pub const WIDGET_MARGIN: u16 = 10;
const WINDOW_PADDING: i32 = 0;

pub trait Widget {
    fn draw(&self, context: &WidgetRenderContext, position: Position, height: u16);
    fn width(&self) -> u16;
}

pub struct WidgetStore {
    widgets: Vec<Box<dyn Widget>>,
}

impl Default for WidgetStore {
    fn default() -> Self {
        Self::new()
    }
}

impl WidgetStore {
    pub fn new() -> Self {
        Self { widgets: vec![] }
    }

    pub fn add_widget(&mut self, widget: Box<dyn Widget>) {
        self.widgets.push(widget);
    }

    pub fn get_widgets(&self) -> &Vec<Box<dyn Widget>> {
        &self.widgets
    }

    pub fn calculate_width(&self) -> i32 {
        let widgets_width = self
            .widgets
            .iter()
            .map(|widget| widget.width())
            .map(i32::from)
            .sum::<i32>();
        let margins_width = self.widgets.len().saturating_sub(1) as i32 * i32::from(WIDGET_MARGIN);

        widgets_width + margins_width + WINDOW_PADDING
    }
}

pub struct TextPanel {
    width: Cell<u16>,
}

impl TextPanel {
    pub fn new() -> Self {
        Self {
            width: Cell::new(30),
        }
    }

    fn draw(
        &self,
        render_target: &ID2D1HwndRenderTarget,
        text_renderer: &TextRenderer,
        brush: &ID2D1SolidColorBrush,
        position: Position,
        height: u16,
        value: &String,
    ) {
        let text = Text::from(format!("{}", value));

        let text_width = text_renderer.get_width(&text);
        self.width.set(text_width);
        let rectangle = util::rectangle(text_width, height, &position);

        text_renderer.draw(render_target, &text, &rectangle, brush);
    }

    fn width(&self) -> u16 {
        self.width.get()
    }
}

pub struct GraphPanel {
    width: u16,
    values: Mutex<BoundedQueue<f32>>,
}

impl GraphPanel {
    pub fn new(width: u16) -> Self {
        Self {
            width,
            values: Mutex::new(BoundedQueue::new(width as usize)),
        }
    }

    fn draw(
        &self,
        render_target: &ID2D1HwndRenderTarget,
        brush: &ID2D1SolidColorBrush,
        position: Position,
        height: u16,
        value: f32,
    ) {
        let rect = util::rectangle(self.width, height, &position);
        let mut values = self.values.lock().unwrap();

        values.push(value);

        unsafe {
            for (i, &value) in values.iter().enumerate() {
                let line_x = rect.right as u16 - 1 - i as u16;
                let current_line_height = ((value / 100.0) * height as f32)
                    .round()
                    .clamp(0.0, height as f32) as u16;

                let line_y = rect.top as u16 - current_line_height;
                let line_position = Position {
                    x: line_x,
                    y: line_y,
                };
                let line_rect = &util::rectangle(1, current_line_height, &line_position);

                render_target.FillRectangle(line_rect, brush);
            }
        }
    }

    fn width(&self) -> u16 {
        self.width
    }
}
