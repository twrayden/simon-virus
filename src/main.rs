#[cfg(windows)] extern crate winapi;
use std::io::Error;

#[cfg(windows)]
fn start_virus() -> Result<i32, Error> {
    use std::ffi::OsStr;
    use std::iter::once;
    use std::os::windows::ffi::OsStrExt;
    use std::ptr::null_mut;
    use winapi::shared::windef::{RECT, POINT};
    use winapi::um::winuser::{MoveWindow, GetActiveWindow, GetCursorPos, GetWindowRect, GetAsyncKeyState, VK_MENU};

    let mut running = true;

    while running {

     unsafe {
      if GetAsyncKeyState(VK_MENU) == 1 {
        running = false;
      }

      let current_window = GetActiveWindow();

      let mut current_pos = POINT { x: 0, y: 0 };

      let mut current_rect = RECT { left: 0, right: 0, top: 0, bottom: 0 };
      
      GetWindowRect(current_window, &mut current_rect);
      
      GetCursorPos(&mut current_pos);

      let curr_pos = &current_pos;
      let curr_rect = &current_rect;
      let win_width = curr_rect.right - curr_rect.left;
      let win_height = curr_rect.bottom - curr_rect.top;

      println!("x: {} y: {}", curr_pos.x, curr_pos.y);
      println!("rect: {}", curr_rect.left);
      println!("width: {} height: {}", win_width, win_height);

      MoveWindow(current_window, curr_pos.x, curr_pos.y, win_width, win_height, 0);
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
