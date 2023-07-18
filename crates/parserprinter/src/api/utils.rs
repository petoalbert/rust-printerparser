#[macro_export]
macro_rules! measure_time {
    ($name:expr, $block:expr) => {{
        #[cfg(debug_assertions)]
        {
            println!("{}...", $name);
            let start = std::time::Instant::now();
            let result = $block;
            let duration = start.elapsed();
            println!("{} took: {:?}", $name, duration);
            result
        }
        #[cfg(not(debug_assertions))]
        {
            $block
        }
    }};
}


