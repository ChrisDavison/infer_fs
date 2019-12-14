extern crate time;

use std::fs::File;
use std::io::{Read, BufRead, BufReader, Error};

fn guess_time_format(row: &str) -> String {
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
            },
            Err(_) => continue
        }
    }
    out
}

fn split_at(s: String, delim: char) -> Vec<String> {
    let mut v: Vec<String> = Vec::new();
    let n = s.len();
    let mut rem: String = if s.ends_with(delim) { s[..n-1].to_string() } else { s.clone() };

    loop {
        match rem.find(delim) {
            Some(loc) => {
                if loc > rem.len() {
                    panic!("loc > rem.len");
                }
                v.push(rem[..loc].trim().to_string());
                rem = rem[loc+1..].to_string();
            },
            None => {
                if rem.len() < 1{
                    panic!("rem.len < 1");
                }
                v.push(rem);
                break
            }
        }
    }
    v
}

fn extract_time(row: String, delim: char, col: usize) -> String {
    let v = split_at(row, delim);
    v[col].clone()
}

fn formatted_time(row: Result<String, Error>, delim: char, col: usize)
-> Result<time::Timespec, time::ParseError> {
    let r = row.unwrap();
    let t = extract_time(r, delim, col);
    let fmt = guess_time_format(&t);
    match time::strptime(&t.clone(), &fmt.clone()){
        Ok(t) => Ok(t.to_timespec()),
        Err(e) => Err(e)
    }
}

/// Infer samplerate from timestamped dataset
///
/// Automatically skip the first row to account for potential headers.
pub fn infer_samplerate(filename: String, delim: char, num_rows: usize, col: usize)
-> Result<f64, Error> {
    let f = try!(File::open(&filename));
    let fbuf = BufReader::new(f);

    let tvec: Vec<time::Timespec> = fbuf.lines()
                                  .skip(1)
                                  .take(num_rows)
                                  .map(|row| formatted_time(row, delim, col).unwrap())
                                  .collect();

    // Calculate the difference of neighbouring times
    // By zipping the time vector with a 1-offset time vector
    let sum = tvec.iter()
                  .zip(tvec.iter().skip(1))
                  .map(|pair| (*pair.1) - (*pair.0))
                  .fold(0, |acc, tm| acc + tm.num_milliseconds());

    if tvec.len() == 0{
        Ok(0.0)
    } else {
        Ok(1.0 / (sum as f64 / (tvec.len() - 1) as f64 * 1e-3))
    }
}

#[cfg(test)]
mod tests {
    extern crate time;

    use super::{guess_time_format, split_at};

    #[test]
    fn test_split_comma() {
        let s = "this; is; semicolon; separated; data".to_string();
        let splt = split_at(s, ';');
        assert_eq!(splt[0], "this");
    }

    #[test]
    fn test_split_semicolon() {
        let s = "this; is; semicolon; separated; data".to_string();
        let splt = split_at(s, ';');
        assert_eq!(splt[0], "this");
    }

    #[test]
    fn test_split_semicolon_len() {
        let s = "this; is; semicolon; separated; data".to_string();
        let splt = split_at(s, ';');
        assert_eq!(splt.len(), 5);
    }

    #[test]
    fn test_split_delim_at_end(){
        let s = "this; is; semicolon; separated; data;".to_string();
        let splt = split_at(s, ';');
        assert_eq!(splt[0], "this");
    }

    #[test]
    fn test_split_delim_at_end_len(){
        let s = "this; is; semicolon; separated; data;".to_string();
        let splt = split_at(s, ';');
        assert_eq!(splt.len(), 5);
    }

    #[test]
    fn test_guess_time() {
        let pairs: Vec<(String, String)> = vec![
                 ("2015-07-09 23:08:08.123".to_string(), "%Y-%m-%d %H:%M:%S.%f".to_string()),
                 ("150709 23:08:08.123".to_string(), "%y%m%d %H:%M:%S.%f".to_string()),
                 ("20150709 23:08:08.123".to_string(), "%Y%m%d %H:%M:%S.%f".to_string()),
                 ("15-07-09 23:08:08.123".to_string(), "%y-%m-%d %H:%M:%S.%f".to_string()),
                 ("23:08:08.123 09-07-2015".to_string(), "%H:%M:%S.%f %d-%m-%Y".to_string()),
                 ];
        for pair in pairs.iter() {
            assert_eq!(pair.1, guess_time_format(&pair.0))
        }
    }
}
