use crate::metric::{CpuMetric, Metric, MetricKind, MetricSet};
use crate::monitor::Monitor;
use sysinfo::{Components, CpuRefreshKind, RefreshKind, System};

pub struct CpuMonitor {
    system: System,
    metrics: MetricSet,
}

impl CpuMonitor {
    pub fn new() -> Self {
        let mut metrics = MetricSet::new();
        metrics.register(
            MetricKind::Cpu(CpuMetric::Usage),
            Metric::new(|value| format!("{value:.0}%")),
        );

        metrics.register(
            MetricKind::Cpu(CpuMetric::Frequency),
            Metric::new(|value| format!("{value:.2} GHz")),
        );

        Self {
            system: System::new_with_specifics(
                RefreshKind::nothing().with_cpu(CpuRefreshKind::everything()),
            ),
            metrics,
        }
    }
}

impl Default for CpuMonitor {
    fn default() -> Self {
        Self::new()
    }
}

impl Monitor for CpuMonitor {
    fn update(&mut self) {
        self.system.refresh_cpu_all();

        self.metrics.set_raw_value(
            MetricKind::Cpu(CpuMetric::Usage),
            self.system.global_cpu_usage(),
        );

        let cpus = self.system.cpus();
        let frequency = if cpus.is_empty() {
            0.0
        } else {
            (cpus.iter().map(|cpu| cpu.frequency() as f32).sum::<f32>() / cpus.len() as f32) / 1000.0
        };
        self.metrics
            .set_raw_value(MetricKind::Cpu(CpuMetric::Frequency), frequency);
    }

    fn metrics(&self) -> &MetricSet {
        &self.metrics
    }
}
