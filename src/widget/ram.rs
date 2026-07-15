use crate::monitor::Monitor;
use crate::monitor::ram::RamMonitor;
use crate::util::ByteString;
use crate::widget::{Position, Size, Widget, WidgetRenderContext};
use windows::Win32::Graphics::Direct2D::Common::{D2D1_COLOR_F, D2D_RECT_F};
use windows::Win32::Graphics::{Direct2D, DirectWrite};
use windows::core::w;

pub struct RamWidget {
    size: Size,
}

impl RamWidget {
    pub(crate) fn new() -> Box<Self> {
        let size = Size {
            width: 50,
            height: 30,
        };

        Box::new(Self { size })
    }
}

impl Widget for RamWidget {
    fn draw(&self, context: WidgetRenderContext, position: Position) {
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

            let rect= rectangle(&self.size, &position);

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

    fn size(&self) -> &Size {
        &self.size
    }
}

fn rectangle(size: &Size, position: &Position) -> D2D_RECT_F {
    D2D_RECT_F {
        left: position.x as f32,
        top: (size.height + position.y) as f32,
        right: (size.width + position.x) as f32,
        bottom: position.y as f32,
    }
}