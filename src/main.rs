pub mod monitor;
pub mod render;
mod util;
pub mod widget;
pub mod window;

use crate::monitor::MonitorStore;
use crate::monitor::cpu::CpuMonitor;
use crate::monitor::ram::RamMonitor;
use crate::widget::WidgetStore;
use crate::widget::cpu::CpuWidget;
use crate::widget::ram::RamWidget;
use crate::window::TaskbarWindow;
use std::cell::RefCell;
use std::sync::OnceLock;
use windows::Win32;
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, RECT, WPARAM};
use windows::Win32::Graphics::Gdi;
use windows::Win32::System::Com::{COINIT_MULTITHREADED, CoInitializeEx};
use windows::Win32::UI::Accessibility::{SetWinEventHook, UnhookWinEvent};
use windows::Win32::UI::WindowsAndMessaging::{
    DefWindowProcW, DispatchMessageW, EVENT_OBJECT_LOCATIONCHANGE, FindWindowExW, FindWindowW,
    GetMessageW, GetWindowRect, HTTRANSPARENT, MSG, PostQuitMessage, SetTimer, TranslateMessage,
    WM_DESTROY, WM_NCHITTEST, WM_PAINT, WM_TIMER,
};
use windows::core::{PCWSTR, w};

const WINDOW_WIDTH: i32 = 100;
const WINDOW_CLASS_NAME: PCWSTR = w!("sys-stats");

thread_local! {
    static STATS_WINDOW: OnceLock<TaskbarWindow> = OnceLock::new();
    static MONITOR_STORE: RefCell<MonitorStore> = RefCell::new(MonitorStore::new());
    static WIDGET_STORE: RefCell<WidgetStore> = RefCell::new(WidgetStore::new());
}

fn main() {
    unsafe {
        CoInitializeEx(None, COINIT_MULTITHREADED).unwrap();

        let taskbar_hwnd = get_taskbar_hwnd();

        WIDGET_STORE.with_borrow_mut(|store| {
            store.add_widget(CpuWidget::new());
            store.add_widget(RamWidget::new());
        });

        MONITOR_STORE.with_borrow_mut(|store| {
            store.add_monitor(Box::new(CpuMonitor::new()));
            store.add_monitor(Box::new(RamMonitor::new()));
        });

        STATS_WINDOW.with(|lock| {
            let window = TaskbarWindow::create(taskbar_hwnd, WINDOW_CLASS_NAME)
                .expect("Can't create window");
            SetTimer(Some(window.hwnd), 1, 500, None);

            lock.set(window).expect("can't set window instance");
        });

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
        match msg {
            WM_TIMER => {
                if wparam.0 == 1 {
                    MONITOR_STORE.with_borrow_mut(|store| {
                        store.update_all();
                    })
                }

                Gdi::InvalidateRect(Some(hwnd), None, false);

                LRESULT(0)
            }
            WM_NCHITTEST => LRESULT(HTTRANSPARENT as isize),
            WM_PAINT => {
                let mut ps = Gdi::PAINTSTRUCT::default();
                Gdi::BeginPaint(hwnd, &mut ps);

                render::draw_window(hwnd);

                Gdi::ValidateRect(Some(hwnd), None);
                Gdi::EndPaint(hwnd, &ps);

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
    STATS_WINDOW.with(|lock| {
        lock.get().unwrap().update_position(position);
    })
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
