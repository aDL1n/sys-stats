use std::cell::Cell;
use crate::monitor::Monitor;
use crate::monitor::ram::RamMonitor;
use crate::util;
use crate::util::ByteString;
use crate::widget::{Position, Widget, WidgetRenderContext};
use windows::Win32::Graphics::{Direct2D, DirectWrite};

pub struct RamWidget {
    width: Cell<u16>,
}

impl RamWidget {
    pub(crate) fn new() -> Box<Self> {
        let width = Cell::new(60);

        Box::new(Self { width })
    }
}

impl Widget for RamWidget {
    fn draw(&self, context: &WidgetRenderContext, position: Position, height: u16) {
        unsafe {
            let render_target = context.render_target;
            let monitor_store = context.monitor_store;

            let value = monitor_store.get_monitor::<RamMonitor>().unwrap().read();
            let value_text = format!("RAM\n{:.2}GB", value);
            self.width.set(util::get_text_width(&value_text));

            let rect = util::rectangle(self.width.get(), height, &position);

            let value_text_bytes: ByteString = ByteString::from(value_text);
            render_target.DrawText(
                value_text_bytes.get_utf16(),
                context.text_format,
                &rect,
                context.text_brush,
                Direct2D::D2D1_DRAW_TEXT_OPTIONS_NONE,
                DirectWrite::DWRITE_MEASURING_MODE_NATURAL,
            );
        }
    }

    fn width(&self) -> u16 {
        self.width.get()
    }
}