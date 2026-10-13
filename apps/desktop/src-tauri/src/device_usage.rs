/// Apps the OS reports as holding a capture device right now, excluding Quiro.
///
/// Windows records every app that opens the camera or microphone under
/// `CapabilityAccessManager\ConsentStore\<capability>`; an entry whose
/// `LastUsedTimeStop` is zero is still using the device. Packaged apps are keyed
/// by package family name, desktop apps by their executable path with `\`
/// replaced by `#`.
#[cfg(windows)]
pub fn apps_using(capability: &str) -> Vec<String> {
    use winreg::{RegKey, enums::HKEY_CURRENT_USER};

    let Ok(store) = RegKey::predef(HKEY_CURRENT_USER).open_subkey(format!(
        r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\{capability}"
    )) else {
        return Vec::new();
    };
    let own_exe = std::env::current_exe()
        .ok()
        .map(|path| path.to_string_lossy().to_lowercase());

    let in_use = |key: &RegKey| {
        key.get_value::<u64, _>("LastUsedTimeStart")
            .is_ok_and(|start| start != 0)
            && key
                .get_value::<u64, _>("LastUsedTimeStop")
                .is_ok_and(|stop| stop == 0)
    };

    let mut apps = Vec::new();
    for name in store.enum_keys().flatten() {
        let Ok(key) = store.open_subkey(&name) else {
            continue;
        };

        if name == "NonPackaged" {
            for exe_key in key.enum_keys().flatten() {
                let path = exe_key.replace('#', "\\");
                if own_exe.as_deref() == Some(path.to_lowercase().as_str()) {
                    continue;
                }
                if key.open_subkey(&exe_key).is_ok_and(|key| in_use(&key))
                    && let Some(stem) = std::path::Path::new(&path).file_stem()
                {
                    let stem = stem.to_string_lossy();
                    let mut chars = stem.chars();
                    apps.extend(
                        chars
                            .next()
                            .map(|first| first.to_uppercase().chain(chars).collect()),
                    );
                }
            }
        } else if in_use(&key) {
            let family = name.split('_').next().unwrap_or(&name);
            apps.push(family.rsplit('.').next().unwrap_or(family).to_string());
        }
    }

    apps.sort();
    apps.dedup();
    apps
}

#[cfg(not(windows))]
pub fn apps_using(_capability: &str) -> Vec<String> {
    Vec::new()
}
