
/// Converts a vector of floats into a comma-separated string with rounded integers
pub fn float_list_to_string(floats: &[f32]) -> String {
    floats.iter()
        .map(|&f| f.round() as i32) // Round each float to the nearest integer
        .map(|i| i.to_string())     // Convert each integer to a string
        .collect::<Vec<_>>()
        .join(",")                 // Join the strings with commas
}

/// Converts a string float to Some(f32), None if invalid
pub fn string_to_float(s: &str) -> Option<f32> {
    s.trim().parse::<f32>().ok()
}

/// Converts a comma-separated string of floats to vector of floats, toss invalid
pub fn string_to_float_list(float_list: &str) -> Vec<f32> {
    float_list
    .split(',')
    .filter_map(|s| s.trim().parse::<f32>().ok())
    .collect()
}
