use crate::monitor::MonitorStore;
use crate::util::{Position, Size};
use windows::Win32::Graphics::Direct2D::Common::{D2D_RECT_F, D2D1_COLOR_F};
use windows::Win32::Graphics::Direct2D::{ID2D1HwndRenderTarget, ID2D1SolidColorBrush};
use windows::Win32::Graphics::DirectWrite::IDWriteFactory;

pub mod cpu;
pub mod ram;

static WIDTH_OFFSET: i32 = 20;

pub trait Widget {
    fn draw(&self, context: &WidgetRenderContext, position: Position, height: u16);
    fn width(&self) -> u16;
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

    pub fn calculate_width(&self) -> i32 {
        (self
            .widgets
            .iter()
            .map(|widget| widget.width())
            .sum::<u16>() as i32)
            + WIDTH_OFFSET
    }
}

pub struct WidgetRenderContext<'a> {
    render_target: &'a ID2D1HwndRenderTarget,
    write_factory: &'a IDWriteFactory,
    monitor_store: &'a MonitorStore,
    text_brush: &'a ID2D1SolidColorBrush
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

        unsafe {
            let text_brush = render_target
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

            let widget_context = WidgetRenderContext {
                render_target,
                write_factory,
                monitor_store,
                text_brush: &text_brush,
            };

            for widget in widgets {
                let widget_position = Position::new(offset_x, 0);

                widget.draw(&widget_context, widget_position, 30);

                offset_x += widget.width() + margin;
            }
        }
    }
}
