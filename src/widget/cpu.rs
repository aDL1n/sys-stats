use crate::monitor::Monitor;
use crate::monitor::cpu::CpuMonitor;
use crate::util;
use crate::util::ByteString;
use crate::widget::{Position, Size, Widget, WidgetRenderContext};
use windows::Win32::Graphics::Direct2D::Common::D2D1_COLOR_F;
use windows::Win32::Graphics::{Direct2D, DirectWrite};
use windows::core::w;

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
    fn draw(&self, context: WidgetRenderContext, position: Position, height: u16) {
        unsafe {
            let render_target = context.render_target;
            let write_factory = context.write_factory;
            let monitor_store = context.monitor_store;

            let brush = render_target
                .CreateSolidColorBrush(
                    &D2D1_COLOR_F {
                        r: 1.0,
                        g: 1.0,
                        b: 1.0,
                        a: 1.0,
                    },
                    None,
                )
                .unwrap();

            let value = monitor_store.get_monitor::<CpuMonitor>().unwrap().read();
            let value: ByteString = ByteString::from(format!("CPU\n{}%", value as i32));

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
                &brush,
                Direct2D::D2D1_DRAW_TEXT_OPTIONS_NONE,
                DirectWrite::DWRITE_MEASURING_MODE_NATURAL,
            );
        }
    }

    fn width(&self) -> u16 {
        self.width
    }
}
