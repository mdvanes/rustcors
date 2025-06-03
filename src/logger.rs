#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {{
        let now = chrono::Local::now().format("%Y/%m/%d %H:%M:%S");
        println!("{} {}", now, format!($($arg)*));
    }};
}
