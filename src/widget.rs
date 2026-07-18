use windows::core::w;
use crate::monitor::MonitorStore;
use crate::util::Position;
use windows::Win32::Graphics::Direct2D::Common::D2D1_COLOR_F;
use windows::Win32::Graphics::Direct2D::{ID2D1HwndRenderTarget, ID2D1SolidColorBrush};
use windows::Win32::Graphics::DirectWrite;
use windows::Win32::Graphics::DirectWrite::{IDWriteFactory, IDWriteTextFormat};

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
    monitor_store: &'a MonitorStore,
    text_brush: &'a ID2D1SolidColorBrush,
    text_format: &'a IDWriteTextFormat,
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
            let text_brush = &render_target
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

            let text_format = &write_factory
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
            text_format.SetTextAlignment(DirectWrite::DWRITE_TEXT_ALIGNMENT_CENTER);
            text_format.SetParagraphAlignment(DirectWrite::DWRITE_PARAGRAPH_ALIGNMENT_CENTER);

            let widget_context = WidgetRenderContext {
                render_target,
                monitor_store,
                text_brush,
                text_format
            };

            for widget in widgets {
                let widget_position = Position::new(offset_x, 0);

                widget.draw(&widget_context, widget_position, 40);

                offset_x += widget.width() + margin;
            }
        }
    }
}
