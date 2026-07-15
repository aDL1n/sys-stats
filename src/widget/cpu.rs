use crate::monitor::cpu::CpuMonitor;
use crate::monitor::{Monitor, MonitorStore};
use crate::util::ByteString;
use crate::widget::{Size, Widget};
use windows::Win32::Graphics::Gdi::{HDC, TextOutW};

pub struct CpuWidget {
    size: Size,
}

impl CpuWidget {
    pub(crate) fn new() -> Box<Self> {
        let size = Size {
            width: 100,
            height: 30,
        };

        Box::new(Self {
            size
        })
    }
}

impl Widget for CpuWidget {
    fn draw(&self, hdc: HDC, monitor_store: &MonitorStore) {
        unsafe {
            let value = monitor_store.get_monitor::<CpuMonitor>()
                .unwrap()
                .read();
            
            let string: ByteString = ByteString::from(format!("CPU {}", value as i32));
            
            TextOutW(hdc, 10, 8, string.get_bytes());
        }
    }

    fn size(&self) -> &Size {
        &self.size
    }
}
