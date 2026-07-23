use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CpuMetric {
    Usage,
    Frequency,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GpuMetric {
    Usage,
    Temperature,
    Frequency,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MemoryMetric {
    Used,
    Free,
    Usage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MetricKind {
    Cpu(CpuMetric),
    Gpu(GpuMetric),
    Ram(MemoryMetric),
    Vram(MemoryMetric),
}

pub struct Metric {
    raw_value: f32,
    formatter: Box<dyn Fn(f32) -> String>,
}

impl Metric {
    pub fn new(formatter: impl Fn(f32) -> String + 'static) -> Self {
        Self {
            raw_value: 0.0,
            formatter: Box::new(formatter),
        }
    }

    pub fn raw_value(&self) -> f32 {
        self.raw_value
    }

    pub fn formatted_value(&self) -> String {
        (self.formatter)(self.raw_value)
    }

    pub fn set_raw_value(&mut self, value: f32) {
        self.raw_value = value;
    }
}

#[derive(Default)]
pub struct MetricSet {
    metrics: HashMap<MetricKind, Metric>,
}

impl MetricSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, metric_kind: MetricKind, metric: Metric) {
        self.metrics.insert(metric_kind, metric);
    }

    pub fn get(&self, monitor_kind: MetricKind) -> Option<&Metric> {
        self.metrics.get(&monitor_kind)
    }

    pub fn set_raw_value(&mut self, metric: MetricKind, value: f32) {
        let metric = self
            .metrics
            .get_mut(&metric)
            .unwrap_or_else(|| panic!("metric {metric:?} is not registered"));
        metric.set_raw_value(value);
    }
}