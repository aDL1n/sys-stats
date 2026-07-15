use crate::monitor::MonitorStore;
use crate::util::{Position, Size};
use windows::Win32::Graphics::Direct2D::Common::{D2D_RECT_F, D2D1_COLOR_F};
use windows::Win32::Graphics::Direct2D::ID2D1HwndRenderTarget;
use windows::Win32::Graphics::DirectWrite::IDWriteFactory;

pub mod cpu;
pub mod ram;

pub trait Widget {
    fn draw(&self, context: WidgetRenderContext, position: Position);

    fn size(&self) -> &Size;
}

pub struct WidgetStore {
    widgets: Vec<Box<dyn Widget>>,
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

    pub fn add_widget(&mut self, widget: Box<dyn Widget>) {
        self.widgets.push(widget);
    }

    pub fn get_widgets(&self) -> &Vec<Box<dyn Widget>> {
        &self.widgets
    }
}

struct WidgetRenderContext<'a> {
    render_target: &'a ID2D1HwndRenderTarget,
    write_factory: &'a IDWriteFactory,
    monitor_store: &'a MonitorStore,
}

pub struct WidgetRenderer {}

impl WidgetRenderer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(
        &self,
        widgets: &Vec<Box<dyn Widget>>,
        render_target: &ID2D1HwndRenderTarget,
        write_factory: &IDWriteFactory,
        monitor_store: &MonitorStore,
    ) {
        let mut offset_x = 0;
        let margin = 10;

        for widget in widgets {
            let widget_context = WidgetRenderContext {
                render_target,
                write_factory,
                monitor_store,
            };
            let widget_position = Position::new(offset_x, 0);

            widget.draw(widget_context, widget_position);

            offset_x += widget.size().width + margin;
        }
    }
}
