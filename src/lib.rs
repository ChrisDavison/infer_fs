extern crate time;

use std::fs::File;
use std::io::{BufRead, BufReader, Error};

mod timeguess;

fn extract_time(row: String, delim: char, col: usize) -> String {
    row.split(delim)
        .nth(col)
        .expect("Couldn't get column from row")
        .to_string()
}

fn formatted_time(
    row: String,
    delim: char,
    col: usize,
) -> Result<time::Timespec, time::ParseError> {
    let t = extract_time(row, delim, col);
    let fmt = timeguess::guess_time_format(&t);
    match time::strptime(&t.clone(), &fmt.clone()) {
        Ok(t) => Ok(t.to_timespec()),
        Err(e) => Err(e),
    }
}

/// Infer samplerate from timestamped dataset
///
/// Automatically skip the first row to account for potential headers.
/// This is a _very_ simple approach to samplerate inference. Currently,
/// only a handful of datetime formats are checked, so this may be a very
/// brittle approach.
pub fn infer_samplerate(
    filename: String,
    delim: char,
    num_rows: usize,
    col: usize,
) -> Result<f64, Error> {
    let f = File::open(&filename)?;
    let fbuf = BufReader::new(f);
    infer_iter(
        fbuf.lines().skip(1).map(|x| x.unwrap()),
        delim,
        num_rows,
        col,
    )
}

fn infer_iter<I>(i: I, delim: char, num_rows: usize, col: usize) -> Result<f64, Error>
where
    I: IntoIterator<Item = String>,
{
    let mut t_prev: Option<time::Timespec> = None;
    let mut diffs = Vec::new();

    // Calculate the difference of neighbouring times
    // By zipping the time vector with a 1-offset time vector
    for row in i.into_iter().take(num_rows) {
        let time = formatted_time(row, delim, col).unwrap();
        if t_prev.is_none() {
            t_prev = Some(time);
        } else {
            diffs.push(time - t_prev.expect("Failed to unwrap 'guaranteed' t_prev"));
            t_prev = Some(time);
        }
    }

    let sum = diffs
        .iter()
        .fold(0.0, |acc, tm| acc + tm.num_milliseconds() as f64);
    if diffs.len() == 0 {
        Ok(0.0)
    } else {
        Ok(1.0 / (sum / diffs.len() as f64 * 1e-3))
    }
}

#[cfg(test)]
mod tests {
    extern crate time;

    use super::*;
    use timeguess::guess_time_format;
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

    #[test]
    fn infer_samplerate_from_csv() {
        let tests = vec![
            ("test_1hz.csv", 1.0),
            ("test_0.33hz.csv", 0.33333333),
            ("test_5hz.csv", 5.0),
        ];
        let delim = ',';
        let col = 0;
        let num_rows = 3;
        let epsilon = 0.00001;
        for (fname, hz) in tests {
            let hz_predicted = infer_samplerate(fname.to_string(), delim, num_rows, col).unwrap();
            assert!((hz_predicted - hz).abs() < epsilon);
        }
    }
}
