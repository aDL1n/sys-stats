use crate::monitor::Monitor;
use std::any::Any;
use sysinfo::{MemoryRefreshKind, RefreshKind, System};

pub enum RamMonitorType {
    Available,
    Used,
    AvailablePercentage,
    UsedPercentage,
}

pub struct RamMonitor {
    system: System,
    monitor_type: RamMonitorType,
}

impl RamMonitor {
    pub fn new(monitor_type: RamMonitorType) -> Self {
        let system = System::new_with_specifics(
            RefreshKind::nothing().with_memory(MemoryRefreshKind::everything()),
        );

        Self {
            system,
            monitor_type,
        }
    }

    fn calculate(&self) -> f32 {
        let total = (self.system.total_memory() + self.system.total_swap()) as f64;
        let used = (self.system.used_memory() + self.system.used_swap()) as f64;

        const GB: f64 = 1_073_741_824.0;

        match self.monitor_type {
            RamMonitorType::Used => (used / GB) as f32,
            RamMonitorType::Available => ((total - used) / GB) as f32,
            RamMonitorType::UsedPercentage => ((used / total) * 100.0) as f32,
            RamMonitorType::AvailablePercentage => (((total - used) / total) * 100.0) as f32,
        }
    }
}

impl Monitor for RamMonitor {
    fn update(&mut self) {
        self.system.refresh_memory();
    }

    fn read(&self) -> f32 {
        self.calculate()
    }

    fn read_string(&self) -> String {
        let value = self.calculate();

        match self.monitor_type {
            RamMonitorType::Used | RamMonitorType::Available => format!("{:.2}GB", value),
            RamMonitorType::UsedPercentage | RamMonitorType::AvailablePercentage => format!("{:.0}%", value),
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
