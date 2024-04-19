use std::{ffi::OsString, os::windows::ffi::OsStringExt, ptr};

use windows_sys::Win32::{
    Foundation::{HWND, MAX_PATH},
    UI::{
        Accessibility::{AccessibleObjectFromWindow, IAccessible},
        Shell::GetWindowContextHelpId,
        WindowsAndMessaging::{
            GetWindowTextLengthW, GetWindowTextW, GetWindowThreadProcessId, GetWindowWord,
            OBJID_CLIENT,
        },
    },
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

// unsafe fn list_menu_items(hwnd: HWND) -> Result<(), windows_sys::core::HRESULT> {
//     let mut acc_ptr: *mut IAccessible = ptr::null_mut();
//     let hr = AccessibleObjectFromWindow(
//         hwnd,
//         OBJID_CLIENT as u32,
//         &IAccessible::IID as *const _ as _,
//         &mut acc_ptr as *mut _ as _,
//     );

//     if hr != 0 || acc_ptr.is_null() {
//         return Err(hr);
//     }

//     let mut child_count: i32 = 0;
//     let acc = &*acc_ptr;
//     acc.get_accChildCount(&mut child_count);

//     for index in 1..=child_count {
//         let mut child_variant = VARIANT {
//             Anonymous: VARIANT_0 {
//                 vt: VT_I4 as u16,
//                 Anonymous: VARIANT_0_0 { lVal: index },
//             },
//         };
//         let mut child_ptr: *mut IAccessible = ptr::null_mut();
//         acc.get_accChild(child_variant, &mut child_ptr);
//         if !child_ptr.is_null() {
//             let child_acc = &*child_ptr;
//             let mut name_bstr = std::ptr::null_mut();
//             child_acc.get_accName(child_variant, &mut name_bstr);
//             let name = widestring::WideCString::from_ptr_str(name_bstr).to_string_lossy();
//             println!("Menu Item {}: {}", index, name);
//         }
//     }

//     Ok(())
// }
