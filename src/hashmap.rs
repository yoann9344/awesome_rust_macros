/// Helper to generate String hashmaps, HashMap<String, String>, using sugar syntax
///
/// ```rust
/// let my_hash_map = hashmap_string!(
///     "key" => "value",
///     "destination" => "/plop/blabla",
///     "another" => "truc",
/// );
/// ```
#[macro_export]
macro_rules! hashmap_string {
    ($($k:expr => $v:expr),* $(,)?) => {{
        HashMap::from([
            $(($k.to_string(), $v.to_string()),
        )*])
    }};
}

/// Helper to insert rows to String hashmaps, HashMap<String, String>, using sugar syntax
///
/// ```rust
/// hashmap_string_insert!(my_hash_map, "inserted_key" => "value");
/// hashmap_string_insert!(
///     my_hash_map,
///     "new_key" => "truc",
///     "another_new_key" => "truc",
/// );
/// ```
#[macro_export]
macro_rules! hashmap_string_insert {
    ($header:ident, $($k:expr => $v:expr),* $(,)?) => {{
        $(
            let _ = $header.insert($k.to_string(), $v.to_string());
        )*
    }};
}


/// Helper to generate String hashmaps, HashMap<String, u64>, using sugar syntax
#[macro_export]
macro_rules! hashmap_u64 {
    ($($k:expr => $v:expr),* $(,)?) => {{
        HashMap::from([$(($k.to_string(), $v as u64),)*])
    }};
}
