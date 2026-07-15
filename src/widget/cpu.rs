use crate::monitor::cpu::CpuMonitor;
use crate::monitor::{Monitor, MonitorStore};
use crate::util::ByteString;
use crate::widget::{Size, Widget};
use windows::Win32::Graphics::Direct2D::Common::{D2D1_COLOR_F, D2D_RECT_F};
use windows::Win32::Graphics::Direct2D::ID2D1HwndRenderTarget;
use windows::Win32::Graphics::DirectWrite::IDWriteFactory;
use windows::Win32::Graphics::{Direct2D, DirectWrite};
use windows::core::w;

pub struct CpuWidget {
    size: Size,
}

impl CpuWidget {
    pub(crate) fn new() -> Box<Self> {
        let size = Size {
            width: 100,
            height: 30,
        };

        Box::new(Self {
            size
        })
    }
}

impl Widget for CpuWidget {

    fn draw(
        &self,
        render_target: &ID2D1HwndRenderTarget,
        write_factory: &IDWriteFactory,
        monitor_store: &MonitorStore,
    ) {
        unsafe {
            let value = monitor_store.get_monitor::<CpuMonitor>()
                .unwrap()
                .read();

            let string: ByteString = ByteString::from(format!("CPU {}", value as i32));

            let text_format = write_factory.CreateTextFormat(
                w!("Segoe UI"),
                None,
                DirectWrite::DWRITE_FONT_WEIGHT_NORMAL,
                DirectWrite::DWRITE_FONT_STYLE_NORMAL,
                DirectWrite::DWRITE_FONT_STRETCH_NORMAL,
                14.0,
                w!("en-US"),
            ).unwrap();

            let brush = render_target.CreateSolidColorBrush(
                &D2D1_COLOR_F { r: 1.0, g: 1.0, b: 1.0, a: 1.0 },
                None
            ).unwrap();

            let rect = D2D_RECT_F { left: 10.0, top: 10.0, right: 100.0, bottom: 50.0 };

            render_target.DrawText(
                string.get_utf16(),
                &text_format,
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
