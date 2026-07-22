use crate::monitor::gpu::GpuMonitor;
use crate::monitor::{HardwareMonitor, HardwareMonitorMetricKind};
use crate::render::WidgetRenderContext;
use crate::util::Position;
use crate::widget::{GraphPanel, TextPanel, Widget};

pub struct GpuTextWidget {
    metric: HardwareMonitorMetricKind,
    text_panel: TextPanel
}

impl GpuTextWidget {
    pub fn new(metric: HardwareMonitorMetricKind) -> Box<Self> {
        Box::new(Self {
            metric,
            text_panel: TextPanel::new()
        })
    }
}

impl Widget for GpuTextWidget {
    fn draw(&self, context: &WidgetRenderContext, position: Position, height: u16) {
        let monitor = context.monitor_store.get_monitor::<GpuMonitor>()
            .unwrap();
        let value = format!("GPU\n{}", monitor.read(&self.metric));

        self.text_panel.draw(
            context.render_target,
            context.text_renderer,
            context.white_brush,
            position,
            height,
            &value
        )
    }

    fn width(&self) -> u16 {
        self.text_panel.width()
    }
}

pub struct GpuGraphWidget {
    metric: HardwareMonitorMetricKind,
    graph_panel: GraphPanel
}

impl GpuGraphWidget {
    pub fn new(metric: HardwareMonitorMetricKind, width: u16) -> Self {
        Self {
            metric,
            graph_panel: GraphPanel::new(width)
        }
    }
}

impl Widget for GpuGraphWidget {
    fn draw(&self, context: &WidgetRenderContext, position: Position, height: u16) {
        let monitor = context.monitor_store.get_monitor::<GpuMonitor>()
            .unwrap();

        self.graph_panel.draw(
            context.render_target,
            context.white_brush,
            position,
            height,
            monitor.read_raw(&self.metric)
        )
    }

    fn width(&self) -> u16 {
        self.graph_panel.width()
    }
}
