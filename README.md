# Inferfs

Rust library to infer the samplerate of a dataset.

This will automatically skip one row (to account for potential headers),
and given a timestamp column, will attempt to infer the samplerate over a given
number of samples.

Right now, the timestamp is very roughly guessed based on some common formats
that I receive.  If a nice datetime guessing library comes out for Rust, I'll
update to include, but that stuff is black magic.

## Usage

Only a single function is publicly exported, `infer_samplerate`:

~~~rust
pub fn infer_samplerate(filename: String, delim: char,
                        num_rows: usize, col: usize)
-> Result<f64, Error>
~~~

## Example

~~~rust
extern crate infer_fs;

fn main() {
    let filename = "some_sample_data.csv".to_string();
    let delim = ',';
    let num_rows = 100;
    let col = 0;

    match infer_fs::infer_samplerate(filename, delim, num_rows, col) {
        Ok(fs) => println!("Samplerate: {}", fs),
        Err(e) => println!("{}", e)
    }
}
~~~
