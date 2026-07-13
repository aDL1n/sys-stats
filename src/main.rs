pub mod window;

use crate::window::Window;
use std::sync::OnceLock;
use windows::Win32;
use windows::Win32::Foundation::{COLORREF, HWND, LPARAM, LRESULT, RECT, WPARAM};
use windows::Win32::Graphics::Gdi::{
    BeginPaint, CreateSolidBrush, EndPaint, FillRect, PAINTSTRUCT, SetBkMode, TRANSPARENT, TextOutW,
};
use windows::Win32::UI::Accessibility::{SetWinEventHook, UnhookWinEvent};
use windows::Win32::UI::WindowsAndMessaging::{
    DefWindowProcW, DispatchMessageW, EVENT_OBJECT_LOCATIONCHANGE, FindWindowExW, FindWindowW,
    GetMessageW, GetWindowRect, MSG, PostQuitMessage, TranslateMessage, WM_DESTROY, WM_PAINT,
};
use windows::core::{w, PCWSTR};

const WINDOW_WIDTH: i32 = 200;
const WINDOW_CLASS_NAME: PCWSTR = w!("sys-stats");

static WINDOW: OnceLock<Window> = OnceLock::new();

fn main() {
    unsafe {
        let taskbar_hwnd = get_taskbar_hwnd();

        let window = Window::create(taskbar_hwnd, WINDOW_CLASS_NAME).expect("Can't create window");
        WINDOW.set(window)
            .expect("can't set window instance");

        update_window_position();

        let hook = SetWinEventHook(
            EVENT_OBJECT_LOCATIONCHANGE,
            EVENT_OBJECT_LOCATIONCHANGE,
            None,
            Some(win_event_proc),
            0,
            0,
            0,
        );

        let mut msg = MSG::default();
        while GetMessageW(&mut msg, None, 0, 0).into() {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }

        if !hook.is_invalid() {
            UnhookWinEvent(hook).ok();
        }
    }
}

unsafe extern "system" fn wnd_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    unsafe {
        println!("{}", msg);
        match msg {
            WM_PAINT => {
                let mut ps = PAINTSTRUCT::default();
                let hdc = BeginPaint(hwnd, &mut ps);

                let mut rect = RECT::default();
                Win32::UI::WindowsAndMessaging::GetClientRect(hwnd, &mut rect);

                let y_center = (rect.bottom - rect.top) / 2;

                let bg_brush = CreateSolidBrush(COLORREF(0xffffff));
                FillRect(hdc, &rect, bg_brush);
                let _ = Win32::Graphics::Gdi::DeleteObject(bg_brush.into());

                SetBkMode(hdc, TRANSPARENT);

                let _ = TextOutW(hdc, 10, y_center - 8, w!("Text 1").as_wide());
                let _ = TextOutW(hdc, 70, 5, w!("Text 2").as_wide());
                let _ = TextOutW(hdc, 130, 5, w!("Text 3").as_wide());

                EndPaint(hwnd, &ps);
                LRESULT(0)
            }
            WM_DESTROY => {
                PostQuitMessage(0);
                LRESULT(0)
            }
            _ => DefWindowProcW(hwnd, msg, wparam, lparam),
        }
    }
}

fn get_taskbar_rectangle() -> RECT {
    use windows::Win32::UI::Shell::{ABM_GETTASKBARPOS, APPBARDATA, SHAppBarMessage};

    unsafe {
        let mut app_bar_data = APPBARDATA {
            cbSize: size_of::<APPBARDATA>() as u32,
            ..Default::default()
        };
        SHAppBarMessage(ABM_GETTASKBARPOS, &mut app_bar_data);
        app_bar_data.rc
    }
}

unsafe fn get_taskbar_hwnd() -> HWND {
    FindWindowW(w!("Shell_TrayWnd"), None).expect("Can't find taskbar window")
}

unsafe fn get_tray_hwnd(taskbar_hwnd: HWND) -> HWND {
    FindWindowExW(Some(taskbar_hwnd), None, w!("TrayNotifyWnd"), None)
        .expect("Can't find tray window")
}

unsafe fn get_stats_position(window_width: i32) -> (i32, i32, i32) {
    let taskbar_rectangle = get_taskbar_rectangle();
    let taskbar_hwnd = get_taskbar_hwnd();
    let tray_hwnd = get_tray_hwnd(taskbar_hwnd);

    let mut tray_rectangle = RECT::default();
    GetWindowRect(tray_hwnd, &mut tray_rectangle).expect("Can't get tray rectangle");

    let tray_x = tray_rectangle.left - taskbar_rectangle.left;

    let window_x = tray_x - window_width - 10;
    let window_y = 0;
    let window_height = taskbar_rectangle.bottom - taskbar_rectangle.top;

    (window_x, window_y, window_height)
}

unsafe fn update_window_position() {
    let position = get_stats_position(WINDOW_WIDTH);
    WINDOW.get().unwrap().update_position(position);
}

unsafe extern "system" fn win_event_proc(
    _h_win_event_hook: Win32::UI::Accessibility::HWINEVENTHOOK,
    event: u32,
    hwnd: HWND,
    _id_object: i32,
    _id_child: i32,
    _dw_event_thread: u32,
    _dwms_event_time: u32,
) {
    if event != EVENT_OBJECT_LOCATIONCHANGE {
        return;
    }

    let taskbar_hwnd = get_taskbar_hwnd();
    let tray_hwnd = get_tray_hwnd(taskbar_hwnd);

    if hwnd == tray_hwnd || hwnd == taskbar_hwnd {
        update_window_position();
    }
}