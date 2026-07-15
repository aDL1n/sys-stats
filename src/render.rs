use std::cell::RefCell;
use std::sync::LazyLock;
use windows::Win32::Foundation::{HWND, RECT};
use windows::Win32::Graphics::Direct2D::Common::{
    D2D_SIZE_U, D2D1_ALPHA_MODE_PREMULTIPLIED, D2D1_COLOR_F, D2D1_PIXEL_FORMAT,
};
use windows::Win32::Graphics::Direct2D::{
    D2D1_FACTORY_TYPE_SINGLE_THREADED, D2D1_HWND_RENDER_TARGET_PROPERTIES,
    D2D1_PRESENT_OPTIONS_NONE, D2D1_RENDER_TARGET_PROPERTIES, D2D1CreateFactory, ID2D1Factory,
    ID2D1HwndRenderTarget,
};
use windows::Win32::Graphics::DirectWrite::{
    DWRITE_FACTORY_TYPE_SHARED, DWriteCreateFactory, IDWriteFactory,
};
use windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_B8G8R8A8_UNORM;
use windows::Win32::UI::WindowsAndMessaging::GetClientRect;

use crate::widget::WidgetRenderer;
use crate::{MONITOR_STORE, WIDGET_STORE};

thread_local! {
    static RENDERER: RefCell<D2DRenderer> = RefCell::new(unsafe { D2DRenderer::new() });
    static WIDGET_RENDERER: LazyLock<WidgetRenderer> = LazyLock::new(|| WidgetRenderer::new());
}

struct D2DRenderer {
    factory: ID2D1Factory,
    write_factory: IDWriteFactory,
    render_target: Option<ID2D1HwndRenderTarget>,
}

impl D2DRenderer {
    unsafe fn new() -> Self {
        let factory: ID2D1Factory = D2D1CreateFactory(D2D1_FACTORY_TYPE_SINGLE_THREADED, None)
            .expect("Failed to create Direct2D factory");

        let write_factory: IDWriteFactory = DWriteCreateFactory(DWRITE_FACTORY_TYPE_SHARED)
            .expect("Failed to create DirectWrite factory");

        Self {
            factory,
            write_factory,
            render_target: None,
        }
    }

    unsafe fn get_render_target(&mut self, hwnd: HWND, width: u32, height: u32) -> &ID2D1HwndRenderTarget {
        if self.render_target.is_none() {
            let props = D2D1_RENDER_TARGET_PROPERTIES {
                pixelFormat: D2D1_PIXEL_FORMAT {
                    format: DXGI_FORMAT_B8G8R8A8_UNORM,
                    alphaMode: D2D1_ALPHA_MODE_PREMULTIPLIED,
                },
                ..Default::default()
            };

            let hwnd_props = D2D1_HWND_RENDER_TARGET_PROPERTIES {
                hwnd,
                pixelSize: D2D_SIZE_U { width, height },
                presentOptions: D2D1_PRESENT_OPTIONS_NONE,
            };

            let target = self
                .factory
                .CreateHwndRenderTarget(&props, &hwnd_props)
                .unwrap();

            self.render_target = Some(target);
        } else {
            let render_target = self.render_target.as_ref().unwrap();
            render_target.Resize(&D2D_SIZE_U { width, height }).ok();
        }

        self.render_target.as_ref().unwrap()
    }

    fn discard_resources(&mut self) {
        self.render_target = None;
    }
}

pub unsafe fn draw_window(hwnd: HWND) {
    let mut rect = RECT::default();
    GetClientRect(hwnd, &mut rect).ok();

    let width = (rect.right - rect.left) as u32;
    let height = (rect.bottom - rect.top) as u32;

    if width == 0 || height == 0 {
        return;
    }

    RENDERER.with(|renderer| {
        let mut renderer = renderer.borrow_mut();

        let write_factory = renderer.write_factory.clone();
        let render_target = renderer.get_render_target(hwnd, width, height);

        render_target.BeginDraw();
        render_target.Clear(Some(&D2D1_COLOR_F {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.0,
        }));

        MONITOR_STORE.with_borrow(|monitor_store| {
            WIDGET_STORE.with_borrow(|widget_store| {
                WIDGET_RENDERER.with(|renderer| {
                    renderer.render(
                        widget_store.get_widgets(),
                        render_target,
                        &write_factory,
                        &monitor_store,
                    );
                })
            });
        });

        let result = render_target.EndDraw(None, None);
        if result.is_err() {
            renderer.discard_resources();
        }
    });
}
