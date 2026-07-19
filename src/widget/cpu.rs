use crate::monitor::Monitor;
use crate::monitor::cpu::CpuMonitor;
use crate::render::Text;
use crate::util;
use crate::util::BoundedQueue;
use crate::widget::{Position, Widget, WidgetRenderContext};
use std::sync::Mutex;

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
        let render_target = context.render_target;
        let monitor_store = context.monitor_store;

        let rect = util::rectangle(self.width, height, &position);

        let value = monitor_store
            .get_monitor::<CpuMonitor>()
            .unwrap()
            .read_string();
        let text = Text::from(format!("CPU\n{}", value));

        context
            .text_renderer
            .draw(render_target, &text, &rect, context.white_brush);
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
                let line_position = Position {
                    x: line_x,
                    y: line_y,
                };
                let line_rect = &util::rectangle(1, current_line_height, &line_position);

                render_target.FillRectangle(line_rect, context.white_brush);
            }
        }
    }

    fn width(&self) -> u16 {
        self.width
    }
}
