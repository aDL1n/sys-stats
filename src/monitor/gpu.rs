use crate::metric::{GpuMetric, MemoryMetric, Metric, MetricKind, MetricSet};
use crate::monitor::Monitor;
use all_smi::AllSmi;

pub struct GpuMonitor {
    smi: AllSmi,
    metrics: MetricSet,
}

impl GpuMonitor {
    pub fn new() -> Self {
        let mut metrics = MetricSet::new();
        metrics.register(
            MetricKind::Gpu(GpuMetric::Usage),
            Metric::new(|value| format!("{value:.0}%")),
        );
        metrics.register(
            MetricKind::Gpu(GpuMetric::Temperature),
            Metric::new(|value| format!("{value:.0}°")),
        );
        metrics.register(
            MetricKind::Gpu(GpuMetric::Frequency),
            Metric::new(|value| format!("{value:.0} MHz")),
        );

        Self {
            smi: AllSmi::new().unwrap(),
            metrics,
        }
    }
}

impl Default for GpuMonitor {
    fn default() -> Self {
        Self::new()
    }
}

impl Monitor for GpuMonitor {
    fn update(&mut self) {
        if let Some(gpu_info) = self.smi.get_gpu_info().first() {
            self.metrics
                .set_raw_value(MetricKind::Gpu(GpuMetric::Usage), gpu_info.utilization as f32);
            self.metrics.set_raw_value(
                MetricKind::Gpu(GpuMetric::Temperature),
                gpu_info.temperature as f32,
            );
            self.metrics.set_raw_value(
                MetricKind::Gpu(GpuMetric::Frequency),
                gpu_info.frequency as f32,
            );
        }
    }

    fn metrics(&self) -> &MetricSet {
        &self.metrics
    }
}