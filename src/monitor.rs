pub mod cpu;
pub mod ram;
pub mod gpu;

use std::any::Any;

pub trait Monitor {
    fn update(&mut self);
    fn as_any(&self) -> &dyn Any;
}

pub enum MemoryMetricValueKind {
    COUNT,
    PERCENT
}

pub enum MemoryMetricKind {
    Used,
    Free
}

pub trait MemoryMonitor: Monitor {
    fn read(&self, metric: &MemoryMetricKind, value_kind: &MemoryMetricValueKind) -> String;
    fn read_raw(&self, metric: &MemoryMetricKind, value_kind: &MemoryMetricValueKind) -> f32;
}

pub enum HardwareMonitorMetricKind {
    USAGE,
    TEMPERATURE,
}

pub trait HardwareMonitor: Monitor {
    fn read(&self, metric: &HardwareMonitorMetricKind) -> String;
    fn read_raw(&self, metric: &HardwareMonitorMetricKind) -> f32;
}

pub struct MonitorStore {
    monitors: Vec<Box<dyn Monitor + Send + Sync>>,
}

impl MonitorStore {
    pub fn new() -> Self {
        Self { monitors: vec![] }
    }

    pub fn update_all(&mut self) {
        self.monitors.iter_mut().for_each(|monitor| {
            monitor.update();
        })
    }

    pub fn add_monitor(&mut self, monitor: Box<dyn Monitor + Send + Sync>) {
        self.monitors.push(monitor);
    }

    pub fn get_monitor<T: 'static>(&self) -> Option<&T> {
        for monitor in &self.monitors {
            if let Some(concrete_monitor) = monitor.as_any().downcast_ref::<T>() {
                return Some(concrete_monitor);
            }
        }
        None
    }
}
