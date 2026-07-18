use crate::monitor::Monitor;
use crate::monitor::cpu::CpuMonitor;
use crate::util;
use crate::util::{BoundedQueue, ByteString};
use crate::widget::{Position, Widget, WidgetRenderContext};
use std::sync::Mutex;
use windows::Win32::Graphics::{Direct2D, DirectWrite};

pub struct CpuWidget {
    width: u16,
}

impl CpuWidget {
    pub(crate) fn new() -> Box<Self> {
        let width = 35;

        Box::new(Self { width })
    }
}

impl Widget for CpuWidget {
    fn draw(&self, context: &WidgetRenderContext, position: Position, height: u16) {
        unsafe {
            let render_target = context.render_target;
            let monitor_store = context.monitor_store;

            let rect = util::rectangle(self.width, height, &position);

            let value = monitor_store.get_monitor::<CpuMonitor>().unwrap().read();
            let text_value = format!("CPU\n{}%", value as i32);
            let text_bytes = ByteString::from(text_value);

            render_target.DrawText(
                text_bytes.get_utf16(),
                context.text_format,
                &rect,
                context.text_brush,
                Direct2D::D2D1_DRAW_TEXT_OPTIONS_NONE,
                DirectWrite::DWRITE_MEASURING_MODE_NATURAL,
            );
        }
    }

    fn width(&self) -> u16 {
        self.width
    }
}

pub struct GraphCpuWidget {
    width: u16,
    values: Mutex<BoundedQueue<f32>>,
}

impl GraphCpuWidget {
    pub(crate) fn new() -> Box<Self> {
        let width = 50;
        Box::new(Self {
            width,
            values: Mutex::new(BoundedQueue::new(width as usize)),
        })
    }
}

impl Widget for GraphCpuWidget {
    fn draw(&self, context: &WidgetRenderContext, position: Position, height: u16) {
        unsafe {
            let rect = util::rectangle(self.width, height, &position);

            let render_target = context.render_target;
            let monitor_store = context.monitor_store;

            let value = monitor_store.get_monitor::<CpuMonitor>().unwrap().read();

            let mut values = self.values.lock().unwrap();
            values.push(value);

            for (i, value) in values.iter().enumerate() {
                let line_x = rect.right as u16 - 1 - i as u16;

                let current_line_height = ((value / 100.0) * height as f32)
                    .round()
                    .clamp(0.0, height as f32) as u16;

                let line_y = rect.top as u16 - current_line_height;
                let line_position = Position { x: line_x, y: line_y };
                let line_rect = &util::rectangle(1, current_line_height, &line_position);

                render_target.FillRectangle(line_rect, context.text_brush);
            }
        }
    }

    fn width(&self) -> u16 {
        self.width
    }
}
