use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Direct2D::Common::{
    D2D_RECT_F, D2D_SIZE_U, D2D1_ALPHA_MODE_PREMULTIPLIED, D2D1_COLOR_F, D2D1_PIXEL_FORMAT,
};
use windows::Win32::Graphics::Direct2D::{
    D2D1_FACTORY_TYPE_SINGLE_THREADED, D2D1_HWND_RENDER_TARGET_PROPERTIES,
    D2D1_PRESENT_OPTIONS_NONE, D2D1_RENDER_TARGET_PROPERTIES, D2D1CreateFactory, ID2D1Brush,
    ID2D1Factory, ID2D1HwndRenderTarget, ID2D1SolidColorBrush,
};
use windows::Win32::Graphics::DirectWrite::{
    DWRITE_FACTORY_TYPE_SHARED, DWriteCreateFactory, IDWriteFactory, IDWriteTextFormat,
};
use windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_B8G8R8A8_UNORM;
use windows::Win32::Graphics::{Direct2D, DirectWrite};
use windows::core::w;

use crate::monitor::MonitorStore;
use crate::util::Position;
use crate::widget::{WIDGET_MARGIN, Widget};
use crate::{MONITOR_STORE, WIDGET_STORE, util};

static CLEAR_COLOR: D2D1_COLOR_F = D2D1_COLOR_F {
    r: 0.0,
    g: 0.0,
    b: 0.0,
    a: 0.0,
};

pub struct Text {
    value: String,
}

impl Text {
    pub fn from(value: String) -> Text {
        Text { value }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

pub struct TextRenderer {
    write_factory: IDWriteFactory,
    format: IDWriteTextFormat,
}

impl TextRenderer {
    fn new() -> Self {
        unsafe {
            let write_factory: IDWriteFactory = DWriteCreateFactory(DWRITE_FACTORY_TYPE_SHARED)
                .expect("Failed to create DirectWrite factory");
            let format = Self::create_text_format(&write_factory);

            Self {
                write_factory,
                format,
            }
        }
    }

    fn create_text_format(write_factory: &IDWriteFactory) -> IDWriteTextFormat {
        unsafe {
            let format = write_factory
                .CreateTextFormat(
                    w!("Segoe UI"),
                    None,
                    DirectWrite::DWRITE_FONT_WEIGHT_NORMAL,
                    DirectWrite::DWRITE_FONT_STYLE_NORMAL,
                    DirectWrite::DWRITE_FONT_STRETCH_NORMAL,
                    13.0,
                    w!("en-US"),
                )
                .unwrap();

            format
                .SetTextAlignment(DirectWrite::DWRITE_TEXT_ALIGNMENT_CENTER)
                .unwrap();
            format
                .SetParagraphAlignment(DirectWrite::DWRITE_PARAGRAPH_ALIGNMENT_CENTER)
                .unwrap();
            format
                .SetWordWrapping(DirectWrite::DWRITE_WORD_WRAPPING_NO_WRAP)
                .unwrap();

            format
        }
    }

    pub fn get_width(&self, text: &Text) -> u16 {
        let text_utf16: Vec<u16> = text.value.encode_utf16().collect();
        if text_utf16.is_empty() {
            return 0;
        }

        unsafe {
            let text_layout = self
                .write_factory
                .CreateTextLayout(text_utf16.as_slice(), &self.format, f32::MAX, f32::MAX)
                .unwrap();

            let mut metrics = std::mem::zeroed();
            text_layout.GetMetrics(&mut metrics).unwrap();

            metrics.width.ceil() as u16
        }
    }

    pub fn draw(
        &self,
        render_target: &ID2D1HwndRenderTarget,
        text: &Text,
        rect: &D2D_RECT_F,
        brush: &ID2D1Brush,
    ) {
        let text_utf16: Vec<u16> = text.value.encode_utf16().collect();

        unsafe {
            render_target.DrawText(
                text_utf16.as_slice(),
                &self.format,
                rect,
                brush,
                Direct2D::D2D1_DRAW_TEXT_OPTIONS_NONE,
                DirectWrite::DWRITE_MEASURING_MODE_NATURAL,
            );
        }
    }
}

struct D2DRenderer {
    factory: ID2D1Factory,
    render_target: Option<ID2D1HwndRenderTarget>,
}

impl D2DRenderer {
    unsafe fn new() -> Self {
        let factory: ID2D1Factory = D2D1CreateFactory(D2D1_FACTORY_TYPE_SINGLE_THREADED, None)
            .expect("Failed to create Direct2D factory");

        Self {
            factory,
            render_target: None,
        }
    }

    unsafe fn get_render_target(
        &mut self,
        hwnd: HWND,
        width: u32,
        height: u32,
    ) -> &ID2D1HwndRenderTarget {
        if self.render_target.is_none() {
            let target = self.create_render_target(hwnd, width, height);
            self.render_target = Some(target);
        } else {
            let render_target = self.render_target.as_ref().unwrap();

            let current = render_target.GetPixelSize();
            if current.width != width || current.height != height {
                render_target
                    .Resize(&D2D_SIZE_U { width, height })
                    .expect("Failed to resize window");
            }
        }

        &self.render_target.as_ref().unwrap()
    }

    unsafe fn create_render_target(
        &mut self,
        hwnd: HWND,
        width: u32,
        height: u32,
    ) -> ID2D1HwndRenderTarget {
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

        self.factory
            .CreateHwndRenderTarget(&props, &hwnd_props)
            .unwrap()
    }

    fn discard_resources(&mut self) {
        self.render_target = None;
    }
}

pub struct WidgetRenderContext<'a> {
    pub render_target: &'a ID2D1HwndRenderTarget,
    pub monitor_store: &'a MonitorStore,
    pub text_renderer: &'a TextRenderer,
    pub white_brush: &'a ID2D1SolidColorBrush,
}

pub struct WidgetRenderer {}

impl WidgetRenderer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(
        &self,
        widgets: &Vec<Box<dyn Widget>>,
        render_target: &ID2D1HwndRenderTarget,
        text_renderer: &TextRenderer,
        monitor_store: &MonitorStore,
    ) {
        let mut offset_x = 0;

        unsafe {
            let white_brush = &render_target
                .CreateSolidColorBrush(
                    &D2D1_COLOR_F {
                        r: 1.0,
                        g: 1.0,
                        b: 1.0,
                        a: 1.0,
                    },
                    None,
                )
                .unwrap();

            let widget_context = WidgetRenderContext {
                render_target,
                monitor_store,
                text_renderer,
                white_brush,
            };
            for widget in widgets {
                let widget_position = Position::new(offset_x, 0);

                widget.draw(&widget_context, widget_position, 40);

                offset_x += widget.width() + WIDGET_MARGIN;
            }
        }
    }
}

pub struct WindowRenderer {
    d2d_renderer: D2DRenderer,
    text_renderer: TextRenderer,
    widget_renderer: WidgetRenderer,
}

impl WindowRenderer {
    pub fn new() -> Self {
        unsafe {
            let d2d_renderer = D2DRenderer::new();
            let text_renderer = TextRenderer::new();
            let widget_renderer = WidgetRenderer::new();

            Self {
                d2d_renderer,
                text_renderer,
                widget_renderer,
            }
        }
    }

    pub fn render(&mut self, hwnd: HWND) {
        unsafe {
            let rect = util::get_client_rect(hwnd);

            let width = (rect.right - rect.left) as u32;
            let height = (rect.bottom - rect.top) as u32;

            if width == 0 || height == 0 {
                return;
            }

            let render_target = self.d2d_renderer.get_render_target(hwnd, width, height);
            render_target.BeginDraw();
            render_target.Clear(Some(&CLEAR_COLOR));

            MONITOR_STORE.with_borrow(|monitor_store| {
                WIDGET_STORE.with_borrow(|widget_store| {
                    self.widget_renderer.render(
                        widget_store.get_widgets(),
                        render_target,
                        &self.text_renderer,
                        monitor_store,
                    );
                });
            });

            let result = render_target.EndDraw(None, None);
            if result.is_err() {
                self.d2d_renderer.discard_resources();
            }
        }
    }
}
