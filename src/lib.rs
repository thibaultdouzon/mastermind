use std::cmp;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::types::{PyInt, PyList, PyTuple};
use pyo3::exceptions::ValueError;



type Comb = Vec<u64>;

fn get_clues(guess: &Comb, secret: &Comb) -> (u64, u64) {
    let n_correct: u64 = guess.iter().zip(secret).fold(0u64, |sum, (g, s)| sum + u64::from(g == s));
    let n_almost: u64 = (0..6u64).map(|c| cmp::min(guess.iter().filter(|&g| *g == c).count(), 
                                                   secret.iter().filter(|&s| *s == c).count()) as u64).sum::<u64>() - n_correct;

    return (n_correct, n_almost);
}

fn fit(prop: &Comb, secret: &Comb, n_correct: u64, n_almost: u64) -> bool {
    get_clues(prop, secret) == (n_correct, n_almost)
}


#[pyfunction]
/// Formats the sum of two numbers as string
fn make_a_guess(remain: &mut PyList) -> PyResult<Comb> {
    // println!("{:?}", remain);
    // println!("{:?}", secret);
    
    let mut r_remain: Vec<Comb> = Vec::new();
    for r in remain.iter().map(|r| r.downcast_ref::<PyTuple>().expect("Some tuple")) {
        r_remain.push(r.iter().map(|r| r.downcast_ref::<PyInt>().expect("Some uint").extract().expect("Some uint2")).collect::<Vec<u64>>());
    }

    let mut best_reduce: usize = 0;
    let mut best_comb: Option<Comb> = None;

    for g in r_remain.iter() {
        let mut worst_reduce = remain.len();
        let mut worst_comb_secret: Option<Comb> = None;

        for s in r_remain.iter() {
            
            let (n_correct, n_almost) = get_clues(&g, &s);

            let n_reduce = r_remain.iter().filter(|&p| !fit(p, g, n_correct, n_almost)).count(); 
            
            if n_reduce < worst_reduce {
                worst_reduce = n_reduce;
                worst_comb_secret = Some(g.to_vec());
            }
        }

        if worst_reduce > best_reduce {
            best_reduce = worst_reduce;
            best_comb = worst_comb_secret;
        }
    } 
    match best_comb {
        Some(v) => Ok(v),
        _ => Err(ValueError::py_err("No result found".to_string()))
    }
}

/// This module is a python module implemented in Rust.
#[pymodule]
fn mastermind(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(make_a_guess))?;

    Ok(())
}



#[cfg(test)]
mod tests {
    #[test]
    fn base_get_clues() {
        assert_eq!(crate::get_clues(&[1, 2, 3, 4], &[1, 2, 3, 4]), (4, 0));
        assert_eq!(crate::get_clues(&[1, 2, 3, 3], &[1, 2, 3, 4]), (3, 0));
        assert_eq!(crate::get_clues(&[2, 1, 4, 3], &[3, 4, 1, 1]), (0, 3));
        assert_eq!(crate::get_clues(&[2, 1, 4, 3], &[3, 4, 1, 3]), (1, 2));
    }
}