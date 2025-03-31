use windows::{
    core::PCWSTR,
    Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM},
    Win32::Graphics::Gdi::{BeginPaint, EndPaint, Rectangle, PAINTSTRUCT},
    Win32::System::LibraryLoader::GetModuleHandleW,
    Win32::UI::WindowsAndMessaging::{
        CreateWindowExW, DefWindowProcW, DispatchMessageW, GetMessageW, KillTimer, RegisterClassW,
        SetTimer, TranslateMessage, CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, MSG, WNDCLASSW,
        WM_DESTROY, WM_PAINT, WM_TIMER, WS_OVERLAPPEDWINDOW, WS_VISIBLE, WINDOW_EX_STYLE,
    },
};
use windows::Win32::Foundation::HINSTANCE;

const IDT_TIMER: usize = 1; // Timer identifier

fn main() {
    unsafe {
        // Register the window class
        let h_instance = GetModuleHandleW(None).unwrap();
        let class_name = PCWSTR("my_window_class\0".encode_utf16().collect::<Vec<u16>>().as_ptr());

        let wc = WNDCLASSW {
            hInstance: HINSTANCE::from(h_instance),
            lpszClassName: class_name,
            lpfnWndProc: Some(window_proc),
            style: CS_HREDRAW | CS_VREDRAW,
            ..Default::default()
        };

        RegisterClassW(&wc);

        // Create the window
        let hwnd = CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            class_name,
            PCWSTR("Rust Window Timer Example\0".encode_utf16().collect::<Vec<u16>>().as_ptr()),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            800,
            600,
            None,
            None,
            Option::from(HINSTANCE::from(h_instance)),
            None,
        );

        // Start the message loop
        let mut msg = MSG::default();

        while GetMessageW(&mut msg, None, 0, 0).into() {
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}

// Window procedure to handle messages
extern "system" fn window_proc(
    hwnd: HWND,
    u_msg: u32,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    unsafe {
        match u_msg {
            WM_PAINT => {
                let mut ps = PAINTSTRUCT::default();
                let hdc = BeginPaint(hwnd, &mut ps);

                // Draw a rectangle
                let _ = Rectangle(hdc, 100, 100, 300, 200);

                let _ = EndPaint(hwnd, &ps);
                LRESULT(0)
            }
            WM_TIMER => {
                // Force a redraw of the window
                let _ = windows::Win32::Graphics::Gdi::InvalidateRect(Option::from(hwnd), None, true);
                LRESULT(0)
            }
            WM_DESTROY => {
                KillTimer(Option::from(hwnd), IDT_TIMER).expect("TODO: panic message"); // Stop the timer when the window is destroyed
                windows::Win32::UI::WindowsAndMessaging::PostQuitMessage(0);
                LRESULT(0)
            }
            _ => DefWindowProcW(hwnd, u_msg, w_param, l_param),
        }
    }
}