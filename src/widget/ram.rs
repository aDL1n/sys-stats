use crate::monitor::Monitor;
use crate::monitor::ram::RamMonitor;
use crate::util;
use crate::util::ByteString;
use crate::widget::{Position, Widget, WidgetRenderContext};
use windows::Win32::Graphics::{Direct2D, DirectWrite};
use windows::core::w;

pub struct RamWidget {
    width: u16,
}

impl RamWidget {
    pub(crate) fn new() -> Box<Self> {
        let width = 50;

        Box::new(Self { width })
    }
}

impl Widget for RamWidget {
    fn draw(&self, context: &WidgetRenderContext, position: Position, height: u16) {
        unsafe {
            let render_target = context.render_target;
            let write_factory = context.write_factory;
            let monitor_store = context.monitor_store;

            let value = monitor_store.get_monitor::<RamMonitor>().unwrap().read();
            let value: ByteString = ByteString::from(format!("RAM\n{:.2}GB", value));

            let value_text_format = write_factory
                .CreateTextFormat(
                    w!("Segoe UI"),
                    None,
                    DirectWrite::DWRITE_FONT_WEIGHT_NORMAL,
                    DirectWrite::DWRITE_FONT_STYLE_NORMAL,
                    DirectWrite::DWRITE_FONT_STRETCH_NORMAL,
                    13.0,
                    w!("en-US"),
                )
                .unwrap();

            let rect = util::rectangle(self.width, height, &position);

            render_target.DrawText(
                value.get_utf16(),
                &value_text_format,
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