/// This is not really perfect but it can give an idea of execution time on longer processes.
#[macro_export]
macro_rules! time_function {
    ($x:expr) => {{
        use std::time::Instant;
        use regex::Regex;
        let start = Instant::now();
        let result = $x;
        let duration = start.elapsed();
        // The println!() stuff below is cursed and I hope that I can do it better in the future.
        let re = Regex::new(r"(.\w+)\(*").unwrap();
        let Some(caps) = re.captures(stringify!($x)) else { return };
        println!("{} took: {:.4?}", &caps[1], duration);
        result
    }};
}
