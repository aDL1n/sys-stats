use crate::monitor::Monitor;
use crate::monitor::ram::RamMonitor;
use crate::render::Text;
use crate::util;
use crate::widget::{Position, Widget, WidgetRenderContext};
use std::cell::Cell;

pub struct RamWidget {
    width: Cell<u16>,
}

impl RamWidget {
    pub(crate) fn new() -> Box<Self> {
        let width = Cell::new(70);

        Box::new(Self { width })
    }
}

impl Widget for RamWidget {
    fn draw(&self, context: &WidgetRenderContext, position: Position, height: u16) {
        let render_target = context.render_target;
        let text_renderer = context.text_renderer;
        let monitor_store = context.monitor_store;

        let value = monitor_store.get_monitor::<RamMonitor>().unwrap()
            .read_string();
        let value_text = Text::from(format!("RAM\n{}", value));
        self.width.set(text_renderer.get_width(&value_text));

        let rect = util::rectangle(self.width.get(), height, &position);

        text_renderer.draw(render_target, &value_text, &rect, context.white_brush);
    }

    fn width(&self) -> u16 {
        self.width.get()
    }
}
