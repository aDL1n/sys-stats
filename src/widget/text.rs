use crate::metric::MetricKind;
use crate::render::{Text, WidgetRenderContext};
use crate::util::{self, Position};
use crate::widget::Widget;

pub struct TextWidget {
    title: String,
    metric: MetricKind,
    width: u16,
}

impl TextWidget {
    pub fn new(title: impl Into<String>, metric: MetricKind) -> Self {
        Self {
            title: title.into(),
            metric,
            width: 30,
        }
    }
}

impl Widget for TextWidget {
    fn draw(&mut self, context: &WidgetRenderContext, position: Position, height: u16) {
        let Some(metric) = context.monitor_store.get_metric(self.metric) else {
            return;
        };

        let text = Text::from(format!("{}\n{}", self.title, metric.formatted_value()));
        self.width = context.text_renderer.get_width(&text);
        let rectangle = util::rectangle(self.width, height, &position);

        context.text_renderer.draw(
            context.render_target,
            &text,
            &rectangle,
            context.white_brush,
        );
    }

    fn width(&self) -> u16 {
        self.width
    }
}
