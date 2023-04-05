// Sefl mooving window

use std::fs::File;
use std::io::{self, Write, Read, BufReader};
use std::process::Command;
use std::thread;
use std::time::Duration;

use winit::platform::windows::{WindowExtWindows, WindowBuilderExtWindows, IconExtWindows};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use winapi;
use image::{self, ImageFormat};

fn main() {

    //get actual path
    let mut actual_path = std::env::current_dir().unwrap().to_str().unwrap().to_owned();
    //println!("actual path: {}", actual_path);
    //correct path write
    let mut i = 0;
    while i < actual_path.len() {
        if actual_path.chars().nth(i).unwrap() == '\\' {
            actual_path.replace_range(i..i+1, "\\\\");
            i += 1;
        }
        i += 1;
    }

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        //window initial size
        .with_inner_size(winit::dpi::LogicalSize::new(200, 200))
        //window title
        .with_title("Annoying window")
        //no resize on window
        .with_resizable(false)
        //custom title bar with no minimize and maximize buttons
        .with_enabled_buttons(winit::window::WindowButtons::CLOSE )
        //window icon
        .with_taskbar_icon(Some(winit::window::Icon::from_path(actual_path.clone() + "\\icon.ico", /*icon size Option<PhysicalSize<u32>>*/ None).unwrap()))

        //no title bar

        .build(&event_loop)
        .unwrap();

    let mut x = 0;
    let mut y = 0;
    let mut dx = 1;
    let mut dy = 1;

    //hide console window
    let _ = unsafe { winapi::um::wincon::FreeConsole() };

    //keep window on top
    unsafe { winapi::um::winuser::SetWindowPos(window.hwnd() as winapi::shared::windef::HWND, winapi::um::winuser::HWND_TOPMOST, 0, 0, 0, 0, winapi::um::winuser::SWP_NOMOVE | winapi::um::winuser::SWP_NOSIZE) };



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
    
    let file_needed = actual_path.clone() + "\\im";
    //println!("actual path: {}", actual_path);
    //pause
    //let mut input = String::new();
    //io::stdin().read_line(&mut input).unwrap();
    //open image as png format even if it is not
    let mut image = image::load(
        BufReader::new(File::open(file_needed.to_owned()).unwrap()),
        ImageFormat::Png
    ).unwrap();
    //let mut image = image::open(actual_path.to_owned()).unwrap();
    //let mut image = image::open("S:\\0\\rust\\tool\\src\\im.png").unwrap();

    //get image size
    let mut img_width = image.width() as i32;
    let mut img_height = image.height() as i32;

    //correct image size if it is bigger than screen size
    if img_width > width {
        img_width = width-200;
    }
    if img_height > height {
        img_height = height-200;
    }



    //set window size to image size
    window.set_inner_size(winit::dpi::LogicalSize::new(img_width as f64, img_height as f64));
    


    let mut image_bytes = image.to_rgba8().into_raw();

    //corect pixels color
    let mut i = 0;
    while i < image_bytes.len() {
        let mut temp = image_bytes[i];
        image_bytes[i] = image_bytes[i+2];
        image_bytes[i+2] = temp;
        i += 4;
    }

    let mut image_bytes_ptr = image_bytes.as_mut_ptr();
    let mut image_bytes_len = image_bytes.len();
    //let mut image_bytes_size = std::mem::size_of::<u8>() * image_bytes_len;
    let mut image_bytes_hbitmap = unsafe { winapi::um::wingdi::CreateBitmap(img_width, img_height, 1, 32, image_bytes_ptr as *const winapi::ctypes::c_void) };
    let mut image_bytes_hdc = unsafe { winapi::um::winuser::GetDC(window.hwnd() as winapi::shared::windef::HWND) };
    let mut image_bytes_hdc_mem = unsafe { winapi::um::wingdi::CreateCompatibleDC(image_bytes_hdc) };
    let mut image_bytes_hbitmap_old = unsafe { winapi::um::wingdi::SelectObject(image_bytes_hdc_mem, image_bytes_hbitmap as winapi::shared::windef::HGDIOBJ) };
    unsafe { winapi::um::wingdi::BitBlt(image_bytes_hdc, 0, 0, img_width, img_height, image_bytes_hdc_mem, 0, 0, winapi::um::wingdi::SRCCOPY) };
    unsafe { winapi::um::wingdi::DeleteObject(image_bytes_hbitmap as winapi::shared::windef::HGDIOBJ) };
    unsafe { winapi::um::wingdi::DeleteDC(image_bytes_hdc_mem) };
    unsafe { winapi::um::winuser::ReleaseDC(window.hwnd() as winapi::shared::windef::HWND, image_bytes_hdc) };
    
    

    

    //get window size
    let mut rect = winapi::shared::windef::RECT { left: 0, top: 0, right: 0, bottom: 0 };
    unsafe { winapi::um::winuser::GetWindowRect(window.hwnd() as winapi::shared::windef::HWND, &mut rect) };
    let window_width = rect.right - rect.left;
    let window_height = rect.bottom - rect.top;

    
    //get program PID
    let mut pid = 0;
    unsafe { winapi::um::winuser::GetWindowThreadProcessId(window.hwnd() as winapi::shared::windef::HWND, &mut pid) };
    println!("pid: {}\n", pid);

    //affiche l'ID de procedure du programme
    
    //print de l'ID de procedure du programme
    print!("module ID: {}", unsafe { winapi::um::winuser::GetWindowThreadProcessId(window.hwnd() as winapi::shared::windef::HWND, std::ptr::null_mut()) });
    

    //get program handle
    let mut hprocess = unsafe { winapi::um::processthreadsapi::OpenProcess(winapi::um::winnt::PROCESS_ALL_ACCESS, 0, pid) };




    

    

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        
        //si le programme est en cours d'exécution mais que la fenêtre est fermée alors on la réouvre
        if window.is_visible() == Some((false)) {
            window.set_visible(true);
        };

        
        
        

        match event {
            Event::WindowEvent {
                // si le bouton de fermeture de la fenêtre est appuyé alors on ouvre une nouvelle fenêtre
                event: WindowEvent::KeyboardInput {
                    input:
                        winit::event::KeyboardInput {
                            state: winit::event::ElementState::Pressed,
                            
                            //exit on CTRL+F7
                            virtual_keycode: Some(winit::event::VirtualKeyCode::F7),
                            
                            modifiers: modifiers,
                            ..
                        },
                    device_id: _,
                    is_synthetic: _,
                    
                    
                    },


                ..


                
            } =>
            {
                if modifiers.ctrl() && modifiers.shift() {
                    *control_flow = ControlFlow::Exit;
                }

            }, 

            Event::LoopDestroyed => {
                //kill process
                unsafe { winapi::um::processthreadsapi::TerminateProcess(hprocess, 0) };
            },



            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } =>
            {
                //do not close window and re-open it
                window.set_visible(false);
                //wait 1 second
                std::thread::sleep(std::time::Duration::from_millis(800));
                window.set_visible(true);
                //re set image on background of the window

                //TODO : put this code in a function
                let mut image_bytes_ptr = image_bytes.as_mut_ptr();
                let mut image_bytes_len = image_bytes.len();
                //let mut image_bytes_size = std::mem::size_of::<u8>() * image_bytes_len;
                let mut image_bytes_hbitmap = unsafe { winapi::um::wingdi::CreateBitmap(img_width, img_height, 1, 32, image_bytes_ptr as *const winapi::ctypes::c_void) };
                let mut image_bytes_hdc = unsafe { winapi::um::winuser::GetDC(window.hwnd() as winapi::shared::windef::HWND) };
                let mut image_bytes_hdc_mem = unsafe { winapi::um::wingdi::CreateCompatibleDC(image_bytes_hdc) };
                let mut image_bytes_hbitmap_old = unsafe { winapi::um::wingdi::SelectObject(image_bytes_hdc_mem, image_bytes_hbitmap as winapi::shared::windef::HGDIOBJ) };
                unsafe { winapi::um::wingdi::BitBlt(image_bytes_hdc, 0, 0, img_width, img_height, image_bytes_hdc_mem, 0, 0, winapi::um::wingdi::SRCCOPY) };
                unsafe { winapi::um::wingdi::DeleteObject(image_bytes_hbitmap as winapi::shared::windef::HGDIOBJ) };
                unsafe { winapi::um::wingdi::DeleteDC(image_bytes_hdc_mem) };
                unsafe { winapi::um::winuser::ReleaseDC(window.hwnd() as winapi::shared::windef::HWND, image_bytes_hdc) };
                

                


            },
            
            //*control_flow = ControlFlow::Exit,
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