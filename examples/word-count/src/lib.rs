// Source adopted from
// https://github.com/tildeio/helix-website/blob/master/crates/word_count/src/lib.rs
#![feature(proc_macro, specialization)]
#[macro_use]
extern crate pyo3;
extern crate rayon;

use std::fs::File;
use std::io::prelude::*;

use pyo3::prelude::*;
use rayon::prelude::*;

use pyo3::py::modinit as pymodinit;

fn matches(word: &str, search: &str) -> bool {
    let mut search = search.chars();
    for ch in word.chars().skip_while(|ch| !ch.is_alphabetic()) {
        match search.next() {
            None => {
                return !ch.is_alphabetic();
            }
            Some(expect) => {
                if ch.to_lowercase().next() != Some(expect) {
                    return false;
                }
            }
        }
    }
    return search.next().is_none();
}

fn wc_line(line: &str, search: &str) -> i32 {
    let mut total = 0;
    for word in line.split(' ') {
        if matches(word, search) {
            total += 1;
        }
    }
    total
}

fn wc_sequential(lines: &str, search: &str) -> i32 {
    lines
        .lines()
        .map(|line| wc_line(line, search))
        .fold(0, |sum, line| sum + line)
}

fn wc_parallel(lines: &str, search: &str) -> i32 {
    lines.par_lines().map(|line| wc_line(line, search)).sum()
}

#[pymodinit(_word_count)]
fn init_mod(py: Python, m: &PyModule) -> PyResult<()> {
    py_exception!(_word_count, JSONDecodeError);

    m.add("JSONDecodeError", py.get_type::<JSONDecodeError>());

    #[pyfn(m, "search")]
    fn search(py: Python, path: String, search: String) -> PyResult<i32> {
        return Err(JSONDecodeError::new(format!("ouch!")));
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let count = py.allow_threads(move || wc_parallel(&contents, &search));
        Ok(count)
    }

    #[pyfn(m, "search_sequential")]
    fn search_sequential(path: String, search: String) -> PyResult<i32> {
        Err(JSONDecodeError::new("ouch"))
    }

    Ok(())
}
