use crate::metric::{MemoryMetric, Metric, MetricKind, MetricSet};
use crate::monitor::Monitor;
use sysinfo::{MemoryRefreshKind, RefreshKind, System};

pub struct RamMonitor {
    system: System,
    metrics: MetricSet,
}

impl RamMonitor {
    pub fn new() -> Self {
        let system = System::new_with_specifics(
            RefreshKind::nothing().with_memory(MemoryRefreshKind::everything()),
        );

        let mut metrics = MetricSet::new();
        for metric in [MemoryMetric::Used, MemoryMetric::Free] {
            metrics.register(
                MetricKind::Ram(metric),
                Metric::new(|value| format!("{value:.1} GB")),
            );
        }
        metrics.register(
            MetricKind::Ram(MemoryMetric::Usage),
            Metric::new(|value| format!("{value:.0}%")),
        );

        Self { system, metrics }
    }
}

impl Default for RamMonitor {
    fn default() -> Self {
        Self::new()
    }
}

impl Monitor for RamMonitor {
    fn update(&mut self) {
        self.system.refresh_memory();

        let total = self.system.total_memory();
        let used = self.system.used_memory();
        let free = self.system.free_memory();

        self.metrics
            .set_raw_value(MetricKind::Ram(MemoryMetric::Used), bytes_to_gigabytes(used));
        self.metrics
            .set_raw_value(MetricKind::Ram(MemoryMetric::Free), bytes_to_gigabytes(free));
        self.metrics
            .set_raw_value(MetricKind::Ram(MemoryMetric::Usage), percentage(used, total));
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
