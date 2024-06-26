use winreg::{
    enums::{HKEY_LOCAL_MACHINE, KEY_READ},
    RegKey,
};

pub fn get_all_applications() -> Vec<(String, String)> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let mut res = Vec::new();

    let uninstall_key = hklm
        .open_subkey_with_flags(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall",
            KEY_READ,
        )
        .unwrap();

    for app in uninstall_key.enum_keys().map(|x| x.unwrap()) {
        let app_key = uninstall_key
            .open_subkey_with_flags(&app, KEY_READ)
            .unwrap();

        let display_name: String = app_key
            .get_value("DisplayName")
            .unwrap_or_else(|_| "".to_string());
        let install_location: String = app_key
            .get_value("InstallLocation")
            .unwrap_or_else(|_| "".to_string());

        if !display_name.is_empty() && !install_location.is_empty() {
            res.push((display_name, install_location));
        }
    }

    res
}
