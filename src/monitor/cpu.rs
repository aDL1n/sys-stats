use crate::monitor::{HardwareMonitor, HardwareMonitorMetricKind, Monitor};
use std::any::Any;
use sysinfo::{Component, Components, CpuRefreshKind, RefreshKind, System};

pub struct CpuMonitor {
    system: System,
    usage: f32,
    temperature: f32,
}

impl CpuMonitor {
    pub fn new() -> Self {
        // let system = System::new_with_specifics(
        //     RefreshKind::nothing().with_cpu(CpuRefreshKind::nothing().with_cpu_usage()),
        // );
        let system = System::new_all();
        Self {
            system,
            usage: 0.0,
            temperature: 0.0,
        }
    }
}

impl Monitor for CpuMonitor {
    fn update(&mut self) {
        let system = &mut self.system;
        system.refresh_cpu_usage();

        let components = Components::new_with_refreshed_list();
        for component in components.iter() {
            println!("{} - {}", component.label(), component.temperature().unwrap_or(0.0));
        }

        self.usage = system.global_cpu_usage();
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl HardwareMonitor for CpuMonitor {
    fn read(&self, metric: &HardwareMonitorMetricKind) -> String {
        match metric {
            HardwareMonitorMetricKind::USAGE => format!("{:.0}%", self.usage),
            HardwareMonitorMetricKind::TEMPERATURE => format!("{:.0}°", self.temperature),
        }
    }

    fn read_raw(&self, metric: &HardwareMonitorMetricKind) -> f32 {
        match metric {
            HardwareMonitorMetricKind::USAGE => self.usage,
            HardwareMonitorMetricKind::TEMPERATURE => self.temperature,
        }
    }
}
