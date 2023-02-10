use std::fs::File;
use pyo3::prelude::*;
use std::io;
use std::io::{BufReader, BufRead};


fn read_file(filename: String) -> io::Result<Vec<String>> {
    // Reads a file as and converts it into a vector of string line by line

    let file = File::open(filename)?;
    let content = BufReader::new(file);

    let lines: Vec<String> = content
        .lines()
        .map(|line| line.expect("Something went wrong"))
        .collect();

    Ok(lines)
}

fn test(checker_sol: String, optimal_sol: String) -> String{
    // Check whether the outputs are same or not. Displays the test case if the are not same.

    let checker_file:Vec<String> = read_file(checker_sol).expect("Something went wrong");
    let optimal_file:Vec<String> = read_file(optimal_sol).expect("Something went wrong");

    let fail_message = String::from("Test Failed");
    let pass_message = String::from("Test Passed");

    if checker_file.len() != optimal_file.len() {
        return String::from("Test Failed");
    }

    for i in 0..checker_file.len() {
        if checker_file[i] != optimal_file[i] {
            println!("Test Failed on line: {}", i+1);
            println!("Actual Output: {}", optimal_file[i]);
            println!("Expected Output: {}", checker_file[i]);
            return fail_message;
        }
    }

    return pass_message;
}

#[pyfunction]
fn checker(checker_sol: String, optimal_sol: String) -> Py<PyAny> {
    let res = test(String::from(checker_sol), String::from(optimal_sol));

    return Python::with_gil(
        |py| {
            res.to_object(py)
        }
    );
}

/// A Python module implemented in Rust.
#[pymodule]
fn StressTesting(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(checker, m)?)?;
    Ok(())
}
