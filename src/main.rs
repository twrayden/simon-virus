#[cfg(windows)] extern crate winapi;
use std::io::Error;

#[cfg(windows)]
fn start_virus() -> Result<i32, Error> {
    use std::ffi::OsStr;
    use std::iter::once;
    use std::os::windows::ffi::OsStrExt;
    use std::ptr::null_mut;
    use winapi::um::winuser::{MB_OK, MoveWindow, GetCapture, GetCursorPos, LPPOINT, GetWindowRect, LPRECT, GetAsyncKeyState, VK_MENU, VK_SHIFT, VK_ESCAPE};

    let running = true;

    while(running) {

      if (GetAsyncKeyState(VK_MENU) == 1 && GetAsyncKeyState(VK_SHIFT) == 1 && GetAsyncKeyState(VK_ESCAPE) == 1) {
        running = false;
      }

      let current_window = GetCapture();

      let current_pos = LPPOINT { x: 0, y: 0 };

      let current_rect = LPRECT {width: 0, height:0};
      
      GetWindowRect(&current_window, &current_rect);
      
      GetCursorPos(&current_pos);

      MoveWindow(current_window, current_pos.x + 10, current_pos.y + 10, current_rect.width, current_rect.height, false);
    
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