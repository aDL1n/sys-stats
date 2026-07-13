use std::ptr;
use std::sync::atomic::{AtomicPtr, Ordering};
use windows::Win32;
use windows::Win32::Foundation::{COLORREF, HINSTANCE, HWND, LPARAM, LRESULT, RECT, WPARAM};
use windows::Win32::Graphics::Gdi::{
    BeginPaint, CreateSolidBrush, EndPaint, FillRect, PAINTSTRUCT, SetBkMode, TRANSPARENT, TextOutW,
};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::WindowsAndMessaging::{CS_HREDRAW, CS_VREDRAW, CreateWindowExW, DefWindowProcW, DispatchMessageW, FindWindowExW, FindWindowW, GetMessageW, GetWindowRect, IDC_ARROW, LoadCursorW, MSG, PostQuitMessage, RegisterClassExW, SW_SHOW, SWP_NOACTIVATE, SetWindowPos, ShowWindow, TranslateMessage, WM_DESTROY, WM_PAINT, WNDCLASSEXW, WS_CHILD, WS_EX_NOACTIVATE, WS_EX_TOOLWINDOW, WS_VISIBLE, EVENT_OBJECT_LOCATIONCHANGE};
use windows::core::w;
use windows::Win32::UI::Accessibility::{SetWinEventHook, UnhookWinEvent};

const WINDOW_WIDTH: i32 = 200;

static OVERLAY_HWND: AtomicPtr<std::ffi::c_void> = AtomicPtr::new(ptr::null_mut());

fn main() {
    if let Err(e) = create_overlay_window() {
        eprintln!("Error: {:?}", e);
        std::process::exit(1);
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

unsafe fn update_stats_position(hwnd: HWND) {
    let (window_x, window_y, window_height) = get_stats_position(WINDOW_WIDTH);

    SetWindowPos(
        hwnd,
        Some(Win32::UI::WindowsAndMessaging::HWND_TOP),
        window_x,
        window_y,
        WINDOW_WIDTH,
        window_height,
        SWP_NOACTIVATE,
    )
    .expect("Can't change stats window position");
}

unsafe extern "system" fn win_event_proc(
    _h_win_event_hook: windows::Win32::UI::Accessibility::HWINEVENTHOOK,
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
        update_stats_position(HWND(OVERLAY_HWND.load(Ordering::Relaxed) as *mut _));
    }
}

fn create_overlay_window() -> windows::core::Result<()> {
    unsafe {
        let instance: HINSTANCE = GetModuleHandleW(None)?.into();
        let class_name = w!("sys-stats");

        let class = WNDCLASSEXW {
            cbSize: size_of::<WNDCLASSEXW>() as u32,
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wnd_proc),
            hInstance: instance,
            hCursor: LoadCursorW(None, IDC_ARROW)?,
            lpszClassName: class_name,
            ..Default::default()
        };

        let registered_class = RegisterClassExW(&class);
        if registered_class == 0 {
            let error = windows::core::Error::from_thread();
            if error.code().0 != 1410 {
                return Err(error);
            }
        }

        let taskbar_hwnd = get_taskbar_hwnd();
        let (window_x, window_y, window_height) = get_stats_position(WINDOW_WIDTH);

        let window_style = WS_EX_NOACTIVATE | WS_EX_TOOLWINDOW;

        let hwnd = CreateWindowExW(
            window_style,
            class_name,
            w!("Overlay"),
            WS_CHILD | WS_VISIBLE,
            window_x,
            window_y,
            WINDOW_WIDTH,
            window_height,
            Some(taskbar_hwnd),
            None,
            Some(instance),
            None,
        )?;
        OVERLAY_HWND.store(hwnd.0, Ordering::Relaxed);
        update_stats_position(hwnd);

        ShowWindow(hwnd, SW_SHOW);

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

    Ok(())
}
