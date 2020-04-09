use std::{fs, io, path};

/// Read file content, iterate over every line, apply given Fn(&str) -> T and return collected Results
pub fn do_on_each_line_of_file_collect<T, P: AsRef<path::Path>, F: Fn(&str) -> T>(
    path: P,
    todo: F,
) -> io::Result<Vec<T>> {
    Ok(fs::read_to_string(path)?.lines().map(todo).collect())
}

/// Read file content, iterate over every line, apply given Fn(&str) -> T and sum it all
pub fn do_on_each_line_of_file_sum<T: std::iter::Sum, P: AsRef<path::Path>, F: Fn(&str) -> T>(
    path: P,
    todo: F,
) -> io::Result<T> {
    Ok(fs::read_to_string(path)?.lines().map(todo).sum())
}
