#[cfg(not(feature = "grapheme"))]
pub fn reverse(input: &str) -> String {
    return input.chars().rev().collect();
}

#[cfg(feature = "grapheme")]
pub fn reverse(input: &str) -> String {
    use unicode_segmentation::UnicodeSegmentation;
    return input.graphemes(true).rev().collect();
}
