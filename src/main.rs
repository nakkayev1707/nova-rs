extern crate image;

use windows::Win32::Graphics::Gdi::{
    CreateCompatibleDC, CreateCompatibleBitmap, BitBlt, SRCCOPY, SelectObject, GetDIBits, BITMAPINFO, BI_RGB, DeleteObject
};
use windows::Win32::UI::WindowsAndMessaging::{GetDC, ReleaseDC, GetDesktopWindow, GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};
use image::{DynamicImage, GrayImage, RgbImage, RgbaImage};


fn main() {
    let img = capture_screen();
    img.save("screenshot.png").unwrap();

    let rgb: RgbImage = RgbImage::new(10, 10);
    let luma: GrayImage = DynamicImage::ImageRgb8(rgb).into_luma8();
}

fn capture_screen() -> DynamicImage {
    unsafe {
        let hdc_screen = GetDC(GetDesktopWindow());

        let width = GetSystemMetrics(SM_CXSCREEN) as i32;
        let height = GetSystemMetrics(SM_CYSCREEN) as i32;

        let hdc_mem = CreateCompatibleDC(hdc_screen);
        let hbitmap = CreateCompatibleBitmap(hdc_screen, width, height);

        SelectObject(hdc_mem, hbitmap);

        BitBlt(hdc_mem, 0, 0, width, height, hdc_screen, 0, 0, SRCCOPY);

        let mut buffer = vec![0u8; (width * height * 4) as usize]; // 4 bytes per pixel (RGBA)

        let mut bitmap_info = BITMAPINFO {
            bmiHeader: windows::Win32::Graphics::Gdi::BITMAPINFOHEADER {
                biSize: std::mem::size_of::<windows::Win32::Graphics::Gdi::BITMAPINFOHEADER>() as u32,
                biWidth: width,
                biHeight: -height, // Negative to indicate top-down bitmap
                biPlanes: 1,
                biBitCount: 32, // 32-bit color (RGBA)
                biCompression: BI_RGB,
                biSizeImage: 0,
                biXPelsPerMeter: 0,
                biYPelsPerMeter: 0,
                biClrUsed: 0,
                biClrImportant: 0,
            },
            bmiColors: [windows::Win32::Graphics::Gdi::RGBQUAD { rgbBlue: 0, rgbGreen: 0, rgbRed: 0, rgbReserved: 0 }; 1],
        };

        let result = GetDIBits(
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

        let img = RgbaImage::from_raw(width as u32, height as u32, buffer).unwrap();

        // Clean up
        DeleteObject(hbitmap);
        ReleaseDC(GetDesktopWindow(), hdc_screen);
        DeleteObject(hdc_mem);

        DynamicImage::ImageRgba8(img)
    }
}