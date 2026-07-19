use crate::monitor::Monitor;
use std::any::Any;
use sysinfo::{CpuRefreshKind, RefreshKind, System};

pub struct CpuMonitor {
    system: System,
    data: f32,
}

impl CpuMonitor {
    pub fn new() -> Self {
        let system = System::new_with_specifics(
            RefreshKind::nothing().with_cpu(CpuRefreshKind::everything()),
        );

        Self { system, data: 0f32 }
    }
}

impl Monitor for CpuMonitor {
    fn update(&mut self) {
        self.system.refresh_cpu_all();
        self.data = self.system.global_cpu_usage();
    }

    fn read(&self) -> f32 {
        self.data
    }

    fn read_string(&self) -> String {
        format!("{:.0}%", self.data)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}