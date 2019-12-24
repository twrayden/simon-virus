#[cfg(windows)]
extern crate winapi;
use std::io::Error;

#[cfg(windows)]
fn start_virus() -> Result<i32, Error> {
  use winapi::shared::windef::{POINT, RECT};
  use winapi::um::winuser::{
    GetAncestor, GetAsyncKeyState, GetCursorPos, GetForegroundWindow, GetSystemMetrics,
    GetWindowRect, IsWindow, SetWindowPos, WindowFromPoint, GA_ROOT, HWND_TOP, SM_CXSCREEN,
    SM_CYSCREEN, VK_MENU, VK_SHIFT,
  };

  let mut running = true;

  while running {
    unsafe {
      if GetAsyncKeyState(VK_MENU) != 0 && GetAsyncKeyState(VK_SHIFT) != 0 {
        running = false;
      }

      let mut current_pos = POINT { x: 0, y: 0 };

      GetCursorPos(&mut current_pos);

      let current_window = GetForegroundWindow();
      let over_window = WindowFromPoint(current_pos);

      let mut current_rect = RECT {
        top: 0,
        bottom: 0,
        left: 0,
        right: 0,
      };

      GetWindowRect(current_window, &mut current_rect);

      if IsWindow(current_window) != 0
        && current_window == GetAncestor(current_window, GA_ROOT)
        && over_window == current_window
      {
        let curr_pos = &current_pos;
        let curr_rect = &current_rect;

        let curr_x_offset = 5;
        let curr_y_offset = 5;

        let mut curr_x = curr_pos.x + curr_x_offset;
        let mut curr_y = curr_pos.y + curr_y_offset;

        let screen_x = GetSystemMetrics(SM_CXSCREEN);
        let screen_y = GetSystemMetrics(SM_CYSCREEN);

        if curr_rect.right > screen_x {
          curr_x = 0;
        } else if curr_rect.bottom > screen_y {
          curr_y = 0;
        } else if curr_rect.left < 0 {
          curr_x = screen_x;
        } else if curr_rect.top < 0 {
          curr_y = screen_y;
        }

        let width = curr_x + 100;
        let height = curr_y + 100;

        println!("screen - x: {} y {}", screen_x, screen_y);
        println!("cursor - x: {} y: {}", curr_pos.x, curr_pos.y);

        SetWindowPos(current_window, HWND_TOP, curr_x, curr_y, width, height, 0);
      }
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
