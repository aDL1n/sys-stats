use crate::monitor::MonitorStore;
use crate::render::TextRenderer;
use crate::util::Position;
use windows::Win32::Graphics::Direct2D::Common::D2D1_COLOR_F;
use windows::Win32::Graphics::Direct2D::{ID2D1HwndRenderTarget, ID2D1SolidColorBrush};

pub mod cpu;
pub mod ram;

static WIDTH_OFFSET: i32 = 25;
static MARGIN: u16 = 10;

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
    monitor_store: &'a MonitorStore,
    text_renderer: &'a TextRenderer,
    white_brush: &'a ID2D1SolidColorBrush,
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
        text_renderer: &TextRenderer,
        monitor_store: &MonitorStore,
    ) {
        let mut offset_x = 0;

        unsafe {
            let white_brush = &render_target
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
                monitor_store,
                text_renderer,
                white_brush,
            };

            for widget in widgets {
                let widget_position = Position::new(offset_x, 0);

                widget.draw(&widget_context, widget_position, 40);

                offset_x += widget.width() + MARGIN;
            }
        }
    }
}
