use crate::monitor::Monitor;
use crate::monitor::cpu::CpuMonitor;
use crate::util;
use crate::util::ByteString;
use crate::widget::{Position, Widget, WidgetRenderContext};
use windows::Win32::Graphics::{Direct2D, DirectWrite};

pub struct CpuWidget {
    width: u16,
}

impl CpuWidget {
    pub(crate) fn new() -> Box<Self> {
        let width = 30;

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
            let value: ByteString = ByteString::from(format!("CPU\n{}%", value as i32));

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
