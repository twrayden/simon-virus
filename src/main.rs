#[cfg(windows)]
extern crate winapi;
use std::io::Error;

#[cfg(windows)]
fn start_virus() -> Result<i32, Error> {
    use winapi::shared::windef::POINT;
    use winapi::um::winuser::{
        GetAsyncKeyState, GetCursorPos, IsWindow, SetWindowPos, ShowWindow, WindowFromPoint,
        HWND_TOP, SW_SHOWNORMAL, VK_MENU, VK_SHIFT,
    };

    let mut running = true;

    let mut last_x = 0;
    let mut last_y = 0;

    while running {
        unsafe {
            if GetAsyncKeyState(VK_MENU) != 0 && GetAsyncKeyState(VK_SHIFT) != 0 {
                running = false;
            }

            let mut current_pos = POINT { x: 0, y: 0 };

            GetCursorPos(&mut current_pos);

            let current_window = WindowFromPoint(current_pos);

            if IsWindow(current_window) != 0 {
                let curr_pos = &current_pos;
                
                let curr_x = curr_pos.x;
                let curr_y = curr_pos.y;

                let curr_x_distance = curr_x - last_x;
                let curr_y_distance = curr_y - last_y;

                
                let curr_x_offset = if curr_x_distance < 0 {
                    return -10;
                } else {
                    return 10;
                }

                let curr_y_offset if curr_y_distance < 0 {
                    return -10;
                } else {
                    return 10;
                }

                println!("x: {} y: {}", curr_pos.x, curr_pos.y);
                
                ShowWindow(current_window, SW_SHOWNORMAL);

                SetWindowPos(
                    current_window,
                    HWND_TOP,
                    curr_pos.x + curr_x_offset,
                    curr_pos.y + curr_y_offset,
                    curr_pos.x + 100,
                    curr_pos.y + 100,
                    0,
                );
            }

            last_x = curr_pos.x;
            last_y = curr_pos.y;
        }
    }

    return Ok(0);
}

#[cfg(not(windows))]
fn start_virus() -> Result<(), Error> {
    return Ok(());
}

fn main() {
    start_virus().unwrap();
}
