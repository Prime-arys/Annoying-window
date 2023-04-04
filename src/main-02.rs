// Sefl mooving window

use std::io::{self, Write, Read};
use std::thread;
use std::time::Duration;

use winit::platform::windows::WindowExtWindows;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use winapi;
use image;

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("A fantastic window!")
        .build(&event_loop)
        .unwrap();

    let mut x = 0;
    let mut y = 0;
    let mut dx = 1;
    let mut dy = 1;

    //hide console window
    let _ = unsafe { winapi::um::wincon::FreeConsole() };


    //get full screen size
    let mut width = 0;
    let mut height = 0;
    let mut hmonitor = unsafe { winapi::um::winuser::MonitorFromWindow(window.hwnd() as winapi::shared::windef::HWND, winapi::um::winuser::MONITOR_DEFAULTTONEAREST) };
    let mut monitor_info = winapi::um::winuser::MONITORINFO {
        cbSize: std::mem::size_of::<winapi::um::winuser::MONITORINFO>() as u32,
        rcMonitor: winapi::shared::windef::RECT { left: 0, top: 0, right: 0, bottom: 0 },
        rcWork: winapi::shared::windef::RECT { left: 0, top: 0, right: 0, bottom: 0 },
        dwFlags: 0,
    };
    unsafe { winapi::um::winuser::GetMonitorInfoA(hmonitor, &mut monitor_info) };
    width = monitor_info.rcMonitor.right - monitor_info.rcMonitor.left;
    height = monitor_info.rcMonitor.bottom - monitor_info.rcMonitor.top;

       
       
    
    //set image on background of the window using image library and winapi library (png format)
    let mut image = image::open("S:\\0\\rust\\tool\\src\\im.png").unwrap();
    let mut image_bytes = image.into_rgba16().into_raw();
    let mut image_bytes_ptr = image_bytes.as_mut_ptr();
    let mut image_bytes_len = image_bytes.len();
    let mut image_bytes_size = std::mem::size_of::<u8>() * image_bytes_len;
    let mut image_bytes_hbitmap = unsafe { winapi::um::wingdi::CreateBitmap(width/4, height/4, 1, 32, image_bytes_ptr as *const winapi::ctypes::c_void) };
    let mut image_bytes_hdc = unsafe { winapi::um::winuser::GetDC(window.hwnd() as winapi::shared::windef::HWND) };
    let mut image_bytes_hdc_mem = unsafe { winapi::um::wingdi::CreateCompatibleDC(image_bytes_hdc) };
    let mut image_bytes_hbitmap_old = unsafe { winapi::um::wingdi::SelectObject(image_bytes_hdc_mem, image_bytes_hbitmap as winapi::shared::windef::HGDIOBJ) };
    unsafe { winapi::um::wingdi::BitBlt(image_bytes_hdc, 0, 0, width/4, height/4, image_bytes_hdc_mem, 0, 0, winapi::um::wingdi::SRCCOPY) };
    unsafe { winapi::um::wingdi::SelectObject(image_bytes_hdc_mem, image_bytes_hbitmap_old as winapi::shared::windef::HGDIOBJ) };
    unsafe { winapi::um::wingdi::DeleteDC(image_bytes_hdc_mem) };
    unsafe { winapi::um::winuser::ReleaseDC(window.hwnd() as winapi::shared::windef::HWND, image_bytes_hdc) };
    unsafe { winapi::um::wingdi::DeleteObject(image_bytes_hbitmap as winapi::shared::windef::HGDIOBJ) };

    

    

    //get window size
    let mut rect = winapi::shared::windef::RECT { left: 0, top: 0, right: 0, bottom: 0 };
    unsafe { winapi::um::winuser::GetWindowRect(window.hwnd() as winapi::shared::windef::HWND, &mut rect) };
    let window_width = rect.right - rect.left;
    let window_height = rect.bottom - rect.top;

    //set window size
    //unsafe { winapi::um::winuser::SetWindowPos(window.hwnd() as winapi::shared::windef::HWND, winapi::um::winuser::HWND_TOPMOST, 0, 0, width/4, height/4, winapi::um::winuser::SWP_SHOWWINDOW) };




    

    

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                x += dx;
                y += dy;

                if x == 0 || x == (width-window_width) - 100 {
                    dx = -dx;
                }
                if y == 0 || y == (height-window_height) - 100 {
                    dy = -dy;
                }

                window.set_outer_position(winit::dpi::PhysicalPosition::new(x, y));
            }
            _ => (),
        }
    });
}