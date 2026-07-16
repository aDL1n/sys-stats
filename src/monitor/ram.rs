use crate::monitor::Monitor;
use std::any::Any;
use sysinfo::{MemoryRefreshKind, RefreshKind, System};

pub struct RamMonitor {
    system: System,
    data: f32,
}

impl RamMonitor {
    pub fn new() -> Self {
        let system = System::new_with_specifics(
            RefreshKind::nothing().with_memory(MemoryRefreshKind::everything()),
        );

        Self { system, data: 0f32 }
    }
}

impl Monitor for RamMonitor {
    fn update(&mut self) {
        self.system.refresh_memory();
        self.data = ((self.system.used_memory() + self.system.used_swap()) as f64 / 1_073_741_824.0) as f32;    }

    fn read(&self) -> f32 {
        self.data
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}