pub mod cpu;
pub mod gpu;
pub mod ram;
pub mod vram;

use crate::metric::{Metric, MetricKind, MetricSet};

pub trait Monitor {
    fn update(&mut self);
    fn metrics(&self) -> &MetricSet;
}

pub struct MonitorStore {
    monitors: Vec<Box<dyn Monitor>>,
}

impl Default for MonitorStore {
    fn default() -> Self {
        Self::new()
    }
}

impl MonitorStore {
    pub fn new() -> Self {
        Self { monitors: vec![] }
    }

    pub fn update_all(&mut self) {
        self.monitors.iter_mut().for_each(|monitor| {
            monitor.update();
        })
    }

    pub fn add_monitor<M: Monitor + 'static>(&mut self, monitor: M) {
        self.monitors.push(Box::new(monitor));
    }

    pub fn get_metric(&self, metric: MetricKind) -> Option<&Metric> {
        self.monitors
            .iter()
            .find_map(|monitor| monitor.metrics().get(metric))
    }
}
