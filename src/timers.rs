/// Helper to time some block of code.
///
/// ```rust
/// crate::timers_blocks!(
///     iterations: 5000,
///     "For loop" => {
///         let mut headers = String::new();
///         for (key, value) in hmap.iter() {
///             headers += &(format!(r#"{}:{}\n"#, key, value));
///         }
///     },
///     "Map and Join functional" => {
///         let _ = hmap
///             .iter()
///             .map(|kv| format!(r#"{}:{}\n"#, kv.0, kv.1))
///             .collect::<Vec<String>>()
///             .join("");
///     },
///     "Fold functional" => {
///         let _ = hmap
///             .iter()
///             .fold("".to_string(), |acc, kv| acc + &(format!(r#"{}:{}\n"#, kv.0, kv.1)));
///     },
///     "Fold functional init with String::new" => {
///         let _ = hmap
///             .iter()
///             .fold(String::new(), |acc, kv| acc + &(format!(r#"{}:{}\n"#, kv.0, kv.1)));
///     },
/// );
/// ```
#[macro_export]
macro_rules! timers_blocks {
    // (names: $($context:expr),*; iterations: $nb_iter:expr => $($context:expr: $t_block:block),* => {
    (iterations: $nb_iter:expr, $($context:expr => $t_block:block),* $(,)?) => {
        let ordered_ctx = vec![$($context.to_string(),)*];
        let mut timers = crate::hashmap_u64!($(
            $context => 0u64,
        )*);
        for _ in 0..$nb_iter {
            $(
                let timer = Instant::now();
                $t_block
                let duration = timer.elapsed();
                *timers.get_mut($context).unwrap() += duration.subsec_nanos() as u64;
            )*
        }
        for context in ordered_ctx.iter() {
            let total_time = timers.get(context).unwrap();
            println!("'{}' has last: {:?}", context, Duration::from_nanos(total_time/$nb_iter));
        }
    }
}
