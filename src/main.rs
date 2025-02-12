use windows::Win32::Foundation::{BOOL, LPARAM, RECT};
use windows::Win32::Graphics::Gdi::{EnumDisplayMonitors, HDC, HMONITOR};
use windows::core::*;
use std::ptr;

#[link(name = "Dxva2")]
extern "system" {
    fn GetMonitorBrightness(
        hmonitor: HMONITOR,
        pdwMinimumBrightness: *mut u32,
        pdwCurrentBrightness: *mut u32,
        pdwMaximumBrightness: *mut u32,
    ) -> BOOL;

    fn SetMonitorBrightness(hmonitor: HMONITOR, dwNewBrightness: u32) -> BOOL;
}

// Corrected callback function signature
unsafe extern "system" fn monitor_enum_proc(
    hmonitor: HMONITOR,
    _: HDC,
    _: *mut RECT,
    brightness: LPARAM,
) -> BOOL {
    let mut min_brightness = 0;
    let mut max_brightness = 0;
    let mut current_brightness = 0;

    if GetMonitorBrightness(hmonitor, &mut min_brightness, &mut current_brightness, &mut max_brightness).as_bool() {
        let new_brightness = brightness.0 as u32;
        if new_brightness >= min_brightness && new_brightness <= max_brightness {
            SetMonitorBrightness(hmonitor, new_brightness);
        }
    }
    true.into()
}

fn set_brightness(level: u32) -> Result<()> {
    unsafe {
        EnumDisplayMonitors(None, None, Some(monitor_enum_proc), LPARAM(level as isize));
    }
    Ok(())
}

fn main() {
    match set_brightness(100) {
        Ok(_) => println!("Brightness set to 100%"),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}
