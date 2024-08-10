use std::iter;

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {{
        let msg = $crate::misc::to_pcstr(&format!("[{}:{}] {}", module_path!(), line!(), format!($($arg)*)));
        unsafe { windows::Win32::System::Diagnostics::Debug::OutputDebugStringA(windows::core::PCSTR(msg.as_ptr())) };
    }};
}

pub fn to_pcstr(s: &str) -> Vec<u8> {
    s.bytes().chain(iter::once(0)).collect()
}
