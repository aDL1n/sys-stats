use std::cell::RefCell;
use windows::Win32::Foundation::{HWND, RECT};
use windows::Win32::UI::WindowsAndMessaging::{FindWindowExW, FindWindowW, GetWindowRect};
use windows::core::w;

thread_local! {
    pub static TASKBAR: RefCell<Taskbar> = RefCell::new(Taskbar::new())
}

pub struct Taskbar {
    hwnd: HWND,
    tray: Tray,
}

impl Taskbar {
    fn new() -> Self {
        let hwnd = find_taskbar_hwnd();
        let tray = Tray::new(hwnd);

        Self { hwnd, tray }
    }

    pub fn hwnd(&self) -> HWND {
        self.hwnd
    }

    pub fn get_rect(&self) -> RECT {
        unsafe {
            let mut rect = RECT::default();
            GetWindowRect(self.hwnd, &mut rect).expect("Can't get taskbar rectangle");

            rect
        }
    }

    pub fn tray(&self) -> &Tray {
        &self.tray
    }
}

pub struct Tray {
    hwnd: HWND,
}

impl Tray {
    fn new(taskbar_hwnd: HWND) -> Self {
        let hwnd = find_tray_hwnd(taskbar_hwnd);

        Self { hwnd }
    }

    pub fn hwnd(&self) -> HWND {
        self.hwnd
    }

    pub fn get_rect(&self) -> RECT {
        unsafe {
            let mut rect = RECT::default();
            GetWindowRect(self.hwnd, &mut rect).expect("Can't get tray rectangle");

            rect
        }
    }
}

fn find_taskbar_hwnd() -> HWND {
    unsafe { FindWindowW(w!("Shell_TrayWnd"), None).expect("Can't find taskbar window") }
}

fn find_tray_hwnd(taskbar_hwnd: HWND) -> HWND {
    unsafe {
        FindWindowExW(Some(taskbar_hwnd), None, w!("TrayNotifyWnd"), None)
            .expect("Can't find tray window")
    }
}
