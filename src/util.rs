use std::collections::VecDeque;
use windows::Win32::Foundation::{HWND, RECT};
use windows::Win32::Graphics::Direct2D::Common::D2D_RECT_F;
use windows::Win32::UI::WindowsAndMessaging::GetClientRect;

pub struct Position {
    pub x: u16,
    pub y: u16,
}

impl Position {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

pub fn rectangle(width: u16, height: u16, position: &Position) -> D2D_RECT_F {
    D2D_RECT_F {
        left: position.x as f32,
        top: (height + position.y) as f32,
        right: (width + position.x) as f32,
        bottom: position.y as f32,
    }
}

pub struct BoundedQueue<T> {
    capacity: usize,
    storage: VecDeque<T>,
}

impl<T> BoundedQueue<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            storage: VecDeque::with_capacity(capacity),
        }
    }

    pub fn push(&mut self, item: T) {
        if self.storage.len() == self.capacity {
            self.storage.pop_front();
        }
        self.storage.push_back(item);
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.storage.iter()
    }
}

pub unsafe fn get_client_rect(hwnd: HWND) -> RECT {
    let mut rect = RECT::default();
    GetClientRect(hwnd, &mut rect).ok();

    rect
}
