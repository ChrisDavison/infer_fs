pub fn guess_time_format(row: &str) -> String {
    let mut out = "".to_string();
    let formats: Vec<String> = vec![
        "%Y-%m-%d %H:%M:%S.%f".to_string(),
        "%y%m%d %H:%M:%S.%f".to_string(),
        "%Y%m%d %H:%M:%S.%f".to_string(),
        "%y-%m-%d %H:%M:%S.%f".to_string(),
        "%H:%M:%S.%f %d-%m-%Y".to_string(),
    ];
    for fmt in formats.iter() {
        match time::strptime(row, fmt) {
            Ok(_) => {
                out = fmt.to_string();
                break;
            }
            Err(_) => continue,
        }
    }
    out
}
