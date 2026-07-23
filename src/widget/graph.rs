use crate::metric::MetricKind;
use crate::monitor::MonitorStore;
use crate::render::WidgetRenderContext;
use crate::util::{self, BoundedQueue, Position};
use crate::widget::Widget;

pub struct GraphWidget {
    metric: MetricKind,
    width: u16,
    max_value: f32,
    values: BoundedQueue<f32>,
}

impl GraphWidget {
    pub fn new(metric: MetricKind, width: u16) -> Self {
        assert!(width > 0, "graph width must be positive");

        Self {
            metric,
            width,
            max_value: 100.0,
            values: BoundedQueue::new(width as usize),
        }
    }

    pub fn with_max_value(mut self, max_value: f32) -> Self {
        assert!(max_value > 0.0, "graph max value must be positive");
        self.max_value = max_value;
        self
    }
}

impl Widget for GraphWidget {
    fn update(&mut self, monitor_store: &MonitorStore) {
        if let Some(metric) = monitor_store.get_metric(self.metric) {
            self.values.push(metric.raw_value());
        }
    }

    fn draw(&mut self, context: &WidgetRenderContext, position: Position, height: u16) {
        if context.monitor_store.get_metric(self.metric).is_none() {
            return;
        }

        let rect = util::rectangle(self.width, height, &position);

        unsafe {
            for (index, value) in self.values.iter().enumerate() {
                let line_x = rect.right as u16 - 1 - index as u16;
                let line_height = ((*value / self.max_value) * height as f32)
                    .round()
                    .clamp(0.0, height as f32) as u16;
                let line_position = Position::new(line_x, rect.top as u16 - line_height);
                let line_rect = util::rectangle(1, line_height, &line_position);

                context
                    .render_target
                    .FillRectangle(&line_rect, context.white_brush);
            }
        }
    }

    fn width(&self) -> u16 {
        self.width
    }
}
