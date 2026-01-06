use std::ffi::c_void;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

type DWORD = u32;

const MOUSEEVENTF_LEFTDOWN: DWORD = 0x0002;
const MOUSEEVENTF_LEFTUP: DWORD = 0x0004;
const MOUSEEVENTF_RIGHTDOWN: DWORD = 0x0008;
const MOUSEEVENTF_RIGHTUP: DWORD = 0x0010;

#[link(name = "user32")]
unsafe extern "system" {
    fn mouse_event(
        dwflags: DWORD,
        dx: DWORD,
        dy: DWORD,
        dwdata: DWORD,
        dwextrainfo: *mut c_void,
    );

    fn GetCursorPos(lppoint: *mut Point) -> i32;
    fn SetCursorPos(x: i32, y: i32) -> i32;
}

#[derive(Debug)]
pub enum MouseError {
    GetCursorFailed,
    SetCursorFailed,
}

pub type MouseResult<T> = Result<T, MouseError>;
pub struct Mouse;

impl Mouse {
    
    pub fn new() -> Self {
        Self
    }
    
    fn get_position(&self) -> MouseResult<Point> {
        let mut point = Point { x: 0, y: 0 };

        unsafe {
            let result = GetCursorPos(&mut point as *mut Point);
            if result == 0 {
                return Err(MouseError::GetCursorFailed);
            }
        }

        Ok(point)
    }

    /// Set the cursor position directly
    fn set_position(&self, x: i32, y: i32) -> MouseResult<()> {
        unsafe {
            let result = SetCursorPos(x, y);
            if result == 0 {
                return Err(MouseError::SetCursorFailed);
            }
        }

        Ok(())
    }
    
    pub fn move_to(&self, x: i32, y: i32, smooth: Option<u32>, sensitivity: Option<f32>) -> MouseResult<()> {
        let sens = sensitivity.unwrap_or(1.0);
        let target_x = (x as f32 * sens) as i32;
        let target_y = (y as f32 * sens) as i32;

        match smooth {
            Some(steps) if steps > 1 => {
                let current = self.get_position()?;

                let delta_x = target_x - current.x;
                let delta_y = target_y - current.y;

                let step_x = delta_x as f32 / steps as f32;
                let step_y = delta_y as f32 / steps as f32;

                for i in 1..=steps {
                    let new_x = current.x + (step_x * i as f32) as i32;
                    let new_y = current.y + (step_y * i as f32) as i32;

                    self.set_position(new_x, new_y)?;
                    std::thread::sleep(std::time::Duration::from_millis(1));
                }

                Ok(())
            }
            _ => {
                self.set_position(target_x, target_y)
            }
        }
    }
    
    pub fn left_click(&self) -> MouseResult<()> {
        unsafe {
            mouse_event(MOUSEEVENTF_LEFTDOWN, 0, 0, 0, std::ptr::null_mut());
            std::thread::sleep(std::time::Duration::from_millis(10));
            mouse_event(MOUSEEVENTF_LEFTUP, 0, 0, 0, std::ptr::null_mut());
        }

        Ok(())
    }
    
    pub fn right_click(&self) -> MouseResult<()> {
        unsafe {
            mouse_event(MOUSEEVENTF_RIGHTDOWN, 0, 0, 0, std::ptr::null_mut());
            std::thread::sleep(std::time::Duration::from_millis(10));
            mouse_event(MOUSEEVENTF_RIGHTUP, 0, 0, 0, std::ptr::null_mut());
        }

        Ok(())
    }
}

impl Default for Mouse {
    fn default() -> Self {
        Self::new()
    }
}