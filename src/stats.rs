pub fn stats(silent: bool, num_read: usize, total_bytes: &mut usize) {
    *total_bytes += num_read;
    if !silent {
        eprintln!("Bytes read: {}", total_bytes);
    }
}
