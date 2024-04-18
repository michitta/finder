use windows_sys::Win32::{
    Foundation::MAX_PATH,
    UI::WindowsAndMessaging::{GetWindowTextW, GetWindowThreadProcessId},
};

pub fn get_window_name(hwnd: isize) -> String {
    let mut process_name = vec![0u16; MAX_PATH as usize];
    let mut process_id = 0;
    unsafe {
        GetWindowThreadProcessId(hwnd, &mut process_id);
    }

    if process_id == 0 {
        return "Failed to get process ID".to_owned();
    }

    let name_length =
        unsafe { GetWindowTextW(hwnd, process_name.as_mut_ptr(), process_name.len() as i32) };

    String::from_utf16_lossy(&process_name[..name_length as usize])
}
