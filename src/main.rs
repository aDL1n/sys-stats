#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod metric;
pub mod monitor;
pub mod render;
pub mod taskbar;
mod util;
pub mod widget;
pub mod window;

use crate::metric::{CpuMetric, GpuMetric, MemoryMetric, MetricKind};
use crate::monitor::MonitorStore;
use crate::monitor::cpu::CpuMonitor;
use crate::monitor::gpu::GpuMonitor;
use crate::monitor::ram::RamMonitor;
use crate::render::WindowRenderer;
use crate::widget::{GraphWidget, TextWidget, WidgetStore};
use crate::window::TaskbarWindow;
use std::cell::RefCell;
use std::sync::OnceLock;
use windows::Win32;
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::Graphics::Gdi;
use windows::Win32::System::Com::{COINIT_MULTITHREADED, CoInitializeEx};
use windows::Win32::System::ProcessStatus::EmptyWorkingSet;
use windows::Win32::System::Threading::GetCurrentProcess;
use windows::Win32::UI::Accessibility::{SetWinEventHook, UnhookWinEvent};
use windows::Win32::UI::WindowsAndMessaging::{
    DefWindowProcW, DispatchMessageW, EVENT_OBJECT_LOCATIONCHANGE, GetMessageW, MA_NOACTIVATE, MSG,
    PostQuitMessage, SetTimer, TranslateMessage, WM_DESTROY, WM_MOUSEACTIVATE, WM_PAINT, WM_TIMER,
};
use windows::core::{PCWSTR, w};
use crate::monitor::vram::VramMonitor;

const WINDOW_CLASS_NAME: PCWSTR = w!("sys-stats");

thread_local! {
    static TASKBAR_WINDOW: OnceLock<TaskbarWindow> = OnceLock::new();
    static WINDOW_RENDERER: RefCell<WindowRenderer> = RefCell::new(WindowRenderer::new());
    static MONITOR_STORE: RefCell<MonitorStore> = RefCell::new(MonitorStore::new());
    static WIDGET_STORE: RefCell<WidgetStore> = RefCell::new(WidgetStore::new());
}

fn main() {
    unsafe {
        CoInitializeEx(None, COINIT_MULTITHREADED).unwrap();

        let taskbar_hwnd = taskbar::TASKBAR.with_borrow(|taskbar| taskbar.hwnd());

        MONITOR_STORE.with_borrow_mut(|store| {
            store.add_monitor(CpuMonitor::new());
            store.add_monitor(RamMonitor::new());
            store.add_monitor(GpuMonitor::new());
            store.add_monitor(VramMonitor::new());
        });

        WIDGET_STORE.with_borrow_mut(|store| {
            store.add_widget(TextWidget::new(
                "CPU",
                MetricKind::Cpu(CpuMetric::Frequency),
            ));
            store.add_widget(GraphWidget::new(MetricKind::Cpu(CpuMetric::Usage), 50));
            store.add_widget(TextWidget::new("RAM", MetricKind::Ram(MemoryMetric::Usage)));
            store.add_widget(TextWidget::new(
                "GPU",
                MetricKind::Gpu(GpuMetric::Temperature),
            ));
            store.add_widget(GraphWidget::new(MetricKind::Gpu(GpuMetric::Usage), 50));
            store.add_widget(TextWidget::new(
                "VRAM",
                MetricKind::Vram(MemoryMetric::Used),
            ));
        });

        TASKBAR_WINDOW.with(|lock| {
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

        EmptyWorkingSet(GetCurrentProcess()).ok();

        update_window_position();

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
            WM_MOUSEACTIVATE => LRESULT(MA_NOACTIVATE as isize),
            WM_TIMER => {
                if wparam.0 == 1 {
                    MONITOR_STORE.with_borrow_mut(|monitor_store| {
                        monitor_store.update_all();

                        WIDGET_STORE
                            .with_borrow_mut(|widget_store| widget_store.update_all(monitor_store));
                    });
                }

                Gdi::InvalidateRect(Some(hwnd), None, false);

                LRESULT(0)
            }
            WM_PAINT => {
                WINDOW_RENDERER.with_borrow_mut(|renderer| renderer.render(hwnd));

                Gdi::ValidateRect(Some(hwnd), None);
                update_window_position();

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

fn get_stats_position() -> (i32, i32, i32, i32) {
    let widgets_width = WIDGET_STORE.with_borrow(|store| store.calculate_width());
    let (taskbar_rect, tray_rect) =
        taskbar::TASKBAR.with_borrow(|taskbar| (taskbar.get_rect(), taskbar.tray().get_rect()));

    let screen_x = tray_rect.left - widgets_width;
    let screen_y = taskbar_rect.top;

    let window_x = screen_x - taskbar_rect.left;
    let window_y = screen_y - taskbar_rect.top;
    let window_height = taskbar_rect.bottom - taskbar_rect.top;

    (window_x, window_y, widgets_width, window_height)
}

unsafe fn update_window_position() {
    let position = get_stats_position();

    TASKBAR_WINDOW.with(|lock| {
        if let Some(window) = lock.get() {
            window.update_position(position);
        }
    });
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

    let (taskbar_hwnd, tray_hwnd) =
        taskbar::TASKBAR.with_borrow(|taskbar| (taskbar.hwnd(), taskbar.tray().hwnd()));

    if hwnd == tray_hwnd || hwnd == taskbar_hwnd {
        update_window_position();
    }
}
