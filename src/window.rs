use crate::{WINDOW_WIDTH, get_stats_position, wnd_proc};
use windows::Win32;
use windows::Win32::Foundation::{HINSTANCE, HWND};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::WindowsAndMessaging::{CS_HREDRAW, CS_VREDRAW, CreateWindowExW, IDC_ARROW, LoadCursorW, RegisterClassExW, SW_HIDE, SW_SHOW, SWP_NOACTIVATE, SetWindowPos, ShowWindow, WNDCLASSEXW, WNDCLASSW, WS_CHILD, WS_EX_NOACTIVATE, WS_EX_TOOLWINDOW, WS_VISIBLE, GWLP_USERDATA, SetWindowLongPtrW, GetWindowLongPtrW};
use windows::core::{PCWSTR, w};

#[derive(Debug)]
pub struct Window {
    pub hwnd: HWND,
}

unsafe impl Sync for Window {}
unsafe impl Send for Window {}

impl Window {
    pub fn create(parent_hwnd: HWND, class_name: PCWSTR) -> windows::core::Result<Self> {
        unsafe {
            let instance: HINSTANCE = Self::create_h_instance();

            let class = Self::create_class_ex(instance, class_name);

            let registered_class = RegisterClassExW(&class);
            if registered_class == 0 {
                let error = windows::core::Error::from_thread();
                if error.code().0 != 1410 {
                    return Err(error);
                }
            }

            let hwnd = Self::create_window_ex(class_name, parent_hwnd, instance)?;

            Ok(Self { hwnd })
        }
    }

    unsafe fn create_h_instance() -> HINSTANCE {
        GetModuleHandleW(None)
            .expect("GetModuleHandleW failed")
            .into()
    }

    unsafe fn create_class_ex(instance: HINSTANCE, class_name: PCWSTR) -> WNDCLASSEXW {
        let cursor = LoadCursorW(None, IDC_ARROW).unwrap();

        WNDCLASSEXW {
            cbSize: size_of::<WNDCLASSEXW>() as u32,
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wnd_proc),
            hInstance: instance,
            hCursor: cursor,
            lpszClassName: class_name,
            ..Default::default()
        }
    }

    unsafe fn create_window_ex(
        class_name: PCWSTR,
        parent_hwnd: HWND,
        instance: HINSTANCE,
    ) -> windows::core::Result<HWND> {
        let window_style = WS_EX_NOACTIVATE | WS_EX_TOOLWINDOW;
        let (window_x, window_y, window_height) = get_stats_position(WINDOW_WIDTH);

        CreateWindowExW(
            window_style,
            class_name,
            w!("Overlay"),
            WS_CHILD | WS_VISIBLE,
            window_x,
            window_y,
            WINDOW_WIDTH,
            window_height,
            Some(parent_hwnd),
            None,
            Some(instance),
            None,
        )
    }

    pub fn update_position(&self, position: (i32, i32, i32)) {
        unsafe {
            SetWindowPos(
                self.hwnd,
                Some(Win32::UI::WindowsAndMessaging::HWND_TOP),
                position.0,
                position.1,
                WINDOW_WIDTH,
                position.2,
                SWP_NOACTIVATE,
            )
            .expect("Can't change stats window position")
        }
    }

    pub fn show(&self) {
        unsafe {
            ShowWindow(self.hwnd, SW_SHOW);
        }
    }

    pub fn hide(&self) {
        unsafe {
            ShowWindow(self.hwnd, SW_HIDE);
        }
    }
}
