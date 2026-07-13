use std::any::Any;
use sysinfo::{CpuRefreshKind, RefreshKind, System};

pub trait Monitor {
    fn update(&mut self);

    fn read(&self) -> f32;

    fn as_any(&self) -> &dyn Any;
}
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

    fn as_any(&self) -> &dyn Any {
        self
    }
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
