pub mod cpu;
pub mod ram;

use std::any::Any;

pub trait Monitor {
    fn update(&mut self);

    fn read(&self) -> f32;
    fn read_string(&self) -> String;

    fn as_any(&self) -> &dyn Any;
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
