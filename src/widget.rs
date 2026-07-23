use crate::monitor::MonitorStore;
use crate::render::WidgetRenderContext;
use crate::util::Position;

pub mod graph;
pub mod text;

pub use graph::GraphWidget;
pub use text::TextWidget;

pub const WIDGET_MARGIN: u16 = 10;
const WINDOW_PADDING: i32 = 0;

pub trait Widget {
    fn update(&mut self, monitor_store: &MonitorStore) {}
    fn draw(&mut self, context: &WidgetRenderContext, position: Position, height: u16);
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

    pub fn add_widget<W: Widget + 'static>(&mut self, widget: W) {
        self.widgets.push(Box::new(widget));
    }

    pub fn widgets_mut(&mut self) -> &mut [Box<dyn Widget>] {
        &mut self.widgets
    }

    pub fn update_all(&mut self, monitor_store: &MonitorStore) {
        (&mut self.widgets).into_iter().for_each(|widget| {
            widget.update(monitor_store);
        });
    }

    pub fn calculate_width(&self) -> i32 {
        let widgets_width = self
            .widgets
            .iter()
            .map(|widget| widget.width())
            .map(i32::from)
            .sum::<i32>();
        let margins_width = self.widgets.len().saturating_sub(1) as i32 * i32::from(WIDGET_MARGIN);

        widgets_width + margins_width + WINDOW_PADDING
    }
}
