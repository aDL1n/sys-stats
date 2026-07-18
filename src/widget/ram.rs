#![windows_subsystem = "windows"]

use crate::monitor::Monitor;
use crate::monitor::ram::RamMonitor;
use crate::util;
use crate::util::ByteString;
use crate::widget::{Position, Widget, WidgetRenderContext};
use windows::Win32::Graphics::{Direct2D, DirectWrite};

pub struct RamWidget {
    width: u16,
}

impl RamWidget {
    pub(crate) fn new() -> Box<Self> {
        let width = 60;

        Box::new(Self { width })
    }
}

impl Widget for RamWidget {
    fn draw(&self, context: &WidgetRenderContext, position: Position, height: u16) {
        unsafe {
            let render_target = context.render_target;
            let monitor_store = context.monitor_store;

            let value = monitor_store.get_monitor::<RamMonitor>().unwrap().read();
            let value: ByteString = ByteString::from(format!("RAM\n{:.2}GB", value));

            let rect = util::rectangle(self.width, height, &position);

            render_target.DrawText(
                value.get_utf16(),
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