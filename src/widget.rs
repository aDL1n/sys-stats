use crate::monitor::MonitorStore;
use windows::Win32::Graphics::Gdi::HDC;

pub mod cpu;

pub trait Widget {
    fn draw(&self, hdc: HDC, monitor_store: &MonitorStore);

    fn size(&self) -> &Size;
}

pub struct WidgetStore {
    widgets: Vec<Box<dyn Widget + Send + Sync>>,
}

impl WidgetStore {
    pub fn new() -> Self {
        Self { widgets: vec![] }
    }
    
    pub fn add_widget(&mut self, widget: Box<dyn Widget + Send + Sync>) {
        self.widgets.push(widget);
    }
    
    pub fn draw_all(&self, hdc: HDC, monitor_store: &MonitorStore) {
        for widget in &self.widgets {
            widget.draw(hdc, monitor_store);
        }
    }
}

pub struct Size {
    width: u16,
    height: u16,
}

impl Size {

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn width(&self) -> u16 {
        self.width
    }
}