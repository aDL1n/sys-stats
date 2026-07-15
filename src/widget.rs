use crate::monitor::MonitorStore;
use windows::Win32::Graphics::Direct2D::Common::{D2D1_COLOR_F, D2D_RECT_F};
use windows::Win32::Graphics::Direct2D::ID2D1HwndRenderTarget;
use windows::Win32::Graphics::DirectWrite::IDWriteFactory;
pub mod cpu;

pub trait Widget {
    fn draw(
        &self,
        render_target: &ID2D1HwndRenderTarget,
        write_factory: &IDWriteFactory,
        monitor_store: &MonitorStore,
    );

    fn size(&self) -> &Size;
}

pub struct WidgetStore {
    widgets: Vec<Box<dyn Widget + Send + Sync>>,
}

impl Default for WidgetStore {
    fn default() -> Self {
        Self::new()
    }
}

impl WidgetStore {
    pub fn new() -> Self {
        Self { widgets: vec![] }
    }

    pub fn add_widget(&mut self, widget: Box<dyn Widget + Send + Sync>) {
        self.widgets.push(widget);
    }

    pub fn draw_all(
        &self,
        render_target: &ID2D1HwndRenderTarget,
        write_factory: &IDWriteFactory,
        store: &MonitorStore,
    ) {
        unsafe {
            let brush = render_target.CreateSolidColorBrush(
                &D2D1_COLOR_F { r: 1.0, g: 1.0, b: 1.0, a: 1.0 },
                None
            ).unwrap();

            let mut current_x = 0.0;
            let margin = 10.0;

            for widget in &self.widgets {
                let width = widget.size().width() as f32;
                let height = widget.size().height() as f32;

                let rect = D2D_RECT_F {
                    left: current_x,
                    top: 0.0,
                    right: current_x + width,
                    bottom: height,
                };

                widget.draw(render_target, write_factory, store);

                render_target.DrawRectangle(&rect, &brush, 1.0, None);
                current_x += width + margin;
            }
        }
    }
}

pub struct Size {
    width: u16,
    height: u16,
}

impl Size {
    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn width(&self) -> u16 {
        self.width
    }
}
