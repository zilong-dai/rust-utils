extern crate chrono;

// #[macro_export]
// macro_rules! measure_time {
//     // 定义 #[measure_time] 宏
//     ($func:ident $($args:tt)*) => {{
//         fn __measure_time_function $($args)* {
//             let start = std::time::Instant::now();
//             let result = $func $($args)*;
//             let duration = start.elapsed();
//             println!("Execution time for {}: {}.{:03} seconds", stringify!($func), duration.as_secs(), duration.subsec_millis());
//             result
//         }

//         __measure_time_function $($args)*
//     }};
// }

#[macro_export]
macro_rules! measure_time {
    ($func:expr) => {{
        measure_time!($func, "")
    }};
    ($func:expr, $msg:expr) => {{
        let start_time = chrono::Local::now();
        let file_name = std::file!();
        let cur_line = std::line!();
        let cur_col = std::column!();

        let result = $func;

        let end_time = chrono::Local::now();
        let duration = end_time.signed_duration_since(start_time);

        println!(
            "{} {} execute {}ms at {}:{}:{} {}",
            start_time.format("%Y-%m-%d %H:%M:%S"),
            stringify!($func),
            duration.num_milliseconds(),
            file_name,
            cur_line,
            cur_col,
            $msg
        );
        result
    }};
}
