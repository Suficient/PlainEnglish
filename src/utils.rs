pub(crate) fn extract(accept: impl Fn(char) -> bool, s: &str) -> (&str, &str) {
    let extract_end = s
    .char_indices() // this turns the &str into an iterator
    .find_map(|(idx,c)| if c == ' ' {None} else {Some(idx)}) //i do not know how find_map works
    .unwrap_or_else(||s.len());

    let extracted =  &s[..extract_end];
    let remainder = &s[extract_end..];
    return (remainder,extracted)
}