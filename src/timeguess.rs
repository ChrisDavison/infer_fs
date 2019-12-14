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

#[test]
fn guess_time() {
    let pairs: Vec<(String, String)> = vec![
        (
            "2015-07-09 23:08:08.123".to_string(),
            "%Y-%m-%d %H:%M:%S.%f".to_string(),
        ),
        (
            "150709 23:08:08.123".to_string(),
            "%y%m%d %H:%M:%S.%f".to_string(),
        ),
        (
            "20150709 23:08:08.123".to_string(),
            "%Y%m%d %H:%M:%S.%f".to_string(),
        ),
        (
            "15-07-09 23:08:08.123".to_string(),
            "%y-%m-%d %H:%M:%S.%f".to_string(),
        ),
        (
            "23:08:08.123 09-07-2015".to_string(),
            "%H:%M:%S.%f %d-%m-%Y".to_string(),
        ),
    ];
    for pair in pairs.iter() {
        assert_eq!(pair.1, guess_time_format(&pair.0))
    }
}
