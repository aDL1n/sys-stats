use all_smi::AllSmi;
use crate::metric::{MemoryMetric, Metric, MetricKind, MetricSet};
use crate::monitor::Monitor;

pub struct VramMonitor {
    smi: AllSmi,
    metrics: MetricSet,
}

impl VramMonitor {
    pub fn new() -> Self {
        let mut metrics = MetricSet::default();

        for metric in [MemoryMetric::Used, MemoryMetric::Free] {
            metrics.register(
                MetricKind::Vram(metric),
                Metric::new(|value| format!("{value:.1} GB")),
            );
        }
        metrics.register(
            MetricKind::Vram(MemoryMetric::Usage),
            Metric::new(|value| format!("{value:.0}%")),
        );
        
        Self {
            smi: AllSmi::new().unwrap(),
            metrics
        }
    }
}

impl Default for VramMonitor {
    fn default() -> Self {
        Self::new()
    }
}

impl Monitor for VramMonitor {
    fn update(&mut self) {
        if let Some(gpu_info) = self.smi.get_gpu_info().first() {
            let used = bytes_to_gigabytes(gpu_info.used_memory);
            let total = bytes_to_gigabytes(gpu_info.total_memory);
            let free = (total - used).max(0.0);
            let usage = percentage(gpu_info.used_memory, gpu_info.total_memory);

            self.metrics
                .set_raw_value(MetricKind::Vram(MemoryMetric::Used), used);
            self.metrics
                .set_raw_value(MetricKind::Vram(MemoryMetric::Free), free);
            self.metrics
                .set_raw_value(MetricKind::Vram(MemoryMetric::Usage), usage);
        }
    }

    fn metrics(&self) -> &MetricSet {
        &self.metrics
    }
}

fn bytes_to_gigabytes(value: u64) -> f32 {
    const GIGABYTE: f32 = 1_073_741_824.0;
    value as f32 / GIGABYTE
}

fn percentage(value: u64, total: u64) -> f32 {
    if total == 0 {
        0.0
    } else {
        value as f32 / total as f32 * 100.0
    }
}
