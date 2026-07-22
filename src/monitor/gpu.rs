use crate::monitor::{HardwareMonitor, HardwareMonitorMetricKind, Monitor};
use all_smi::AllSmi;
use std::any::Any;

pub struct GpuMonitor {
    smi: AllSmi,
    usage: f32,
    temperature: f32,
}

impl GpuMonitor {
    pub fn new() -> Self {
        let smi = AllSmi::new().unwrap();

        Self {
            smi,
            usage: 0.0,
            temperature: 0.0,
        }
    }
}

impl Monitor for GpuMonitor {
    fn update(&mut self) {
        if let Some(gpu_info) = self.smi.get_gpu_info().first() {
            self.usage = gpu_info.utilization as f32;
            self.temperature = gpu_info.temperature as f32;
        }
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl HardwareMonitor for GpuMonitor {
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
