use crate::monitor::{MemoryMetricKind, MemoryMonitor, MemoryMetricValueKind, Monitor};
use std::any::Any;
use systemstat::Platform;
use systemstat::platform::PlatformImpl;

pub struct RamMonitor {
    platform: PlatformImpl,
    total: u64,
    used: u64,
    free: u64,
}

impl RamMonitor {
    pub fn new() -> Self {
        let platform = PlatformImpl::new();

        Self {
            platform,
            total: 0,
            used: 0,
            free: 0,
        }
    }

    fn calculate_metric(&self, metric: &MemoryMetricKind, value_kind: &MemoryMetricValueKind) -> f32 {
        match value_kind {
            MemoryMetricValueKind::COUNT => match metric {
                MemoryMetricKind::Used => bytes_to_gigabytes(self.used) as f32,
                MemoryMetricKind::Free => bytes_to_gigabytes(self.free) as f32,
            },
            MemoryMetricValueKind::PERCENT => {
                if self.total == 0 {
                    return 0.0;
                }
                let value = match metric {
                    MemoryMetricKind::Used => self.used,
                    MemoryMetricKind::Free => self.free,
                };
                (value as f64 / self.total as f64 * 100.0) as f32
            }
        }
    }

}

impl Monitor for RamMonitor {
    fn update(&mut self) {
        // let memory = self.platform.memory().unwrap();
        //
        // self.total = memory.total.as_u64();
        // self.used = memory.free.as_u64();
        // self.free = memory.total.as_u64();
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl MemoryMonitor for RamMonitor {
    fn read(&self, metric: &MemoryMetricKind, value_kind: &MemoryMetricValueKind) -> String {
        match value_kind {
            MemoryMetricValueKind::COUNT => match metric {
                MemoryMetricKind::Used | MemoryMetricKind::Free =>
                    format!("{}GB", self.calculate_metric(metric, value_kind)),
            }
            MemoryMetricValueKind::PERCENT => match metric {
                MemoryMetricKind::Used | MemoryMetricKind::Free =>
                    format!("{}%", self.calculate_metric(metric, value_kind)),
            }
        }
    }

    fn read_raw(&self, metric: &MemoryMetricKind, value_kind: &MemoryMetricValueKind) -> f32 {
        match value_kind {
            MemoryMetricValueKind::COUNT => match metric {
                MemoryMetricKind::Used | MemoryMetricKind::Free =>
                    self.calculate_metric(metric, value_kind)
            }
            MemoryMetricValueKind::PERCENT => match metric {
                MemoryMetricKind::Used | MemoryMetricKind::Free =>
                    self.calculate_metric(metric, value_kind)
            }
        }
    }
}

fn bytes_to_gigabytes(value: u64) -> u16 {
    const GB: u64 = 1_073_741_824;
    (value / GB) as u16
}
