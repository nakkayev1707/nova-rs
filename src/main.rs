extern crate image;

use windows::Win32::Graphics::Gdi::{CreateCompatibleDC, CreateCompatibleBitmap, BitBlt, SRCCOPY, SelectObject};
use windows::Win32::UI::WindowsAndMessaging::{GetDesktopWindow, GetDC, ReleaseDC};
use image::{DynamicImage, RgbaImage};

use image::{Rgb, RgbImage};

fn main() {
    println!("Hello, world!");
}

fn capture_screen() -> DynamicImage {
    unsafe {
        let hdc_screen = GetDC(GetDesktopWindow());

        // Create a compatible device context
        let hdc_mem = CreateCompatibleDC(hdc_screen);

        // Define the screen width and height (adjust to your screen's resolution)
        let width = 1920; // Replace with actual screen width
        let height = 1080; // Replace with actual screen height

        // Create a compatible bitmap
        let hbitmap = CreateCompatibleBitmap(hdc_screen, width, height);

        // Select the bitmap into the memory DC
        SelectObject(hdc_mem, hbitmap);

        // Copy the screen data into the bitmap
        BitBlt(hdc_mem, 0, 0, width, height, hdc_screen, 0, 0, SRCCOPY);

        // Create a buffer to store the image data
        let mut buffer = vec![0u8; (width * height * 4) as usize]; // 4 bytes per pixel (RGBA)

        // Get the bitmap data (DIB format)
        let mut bitmap_info = windows::Win32::Graphics::Gdi::BITMAPINFO::default();
        bitmap_info.bmiHeader.biWidth = width;
        bitmap_info.bmiHeader.biHeight = -height; // Negative to indicate top-down bitmap
        bitmap_info.bmiHeader.biPlanes = 1;
        bitmap_info.bmiHeader.biBitCount = 32; // 32-bit color (RGBA)

        // Copy the bitmap data into the buffer
        let result = windows::Win32::Graphics::Gdi::GetDIBits(
            hdc_mem,
            hbitmap,
            0,
            height as u32,
            *(buffer.as_mut_ptr() as *mut _),
            &mut bitmap_info,
            windows::Win32::Graphics::Gdi::DIB_RGB_COLORS,
        );

        if result == 0 {
            panic!("Failed to capture screen!");
        }

        // Convert the raw buffer to an image
        let img = RgbaImage::from_raw(width as u32, height as u32, buffer).unwrap();

        // Release the device contexts
        ReleaseDC(GetDesktopWindow(), hdc_screen);

        // Return the captured image
        DynamicImage::ImageRgba8(img)
    }
}
