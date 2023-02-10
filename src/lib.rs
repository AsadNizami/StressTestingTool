use std::fs::File;
use pyo3::prelude::*;
use std::io;
use std::io::{BufReader, BufRead};


fn read_file(filename: String) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let content = BufReader::new(file);

    let lines: Vec<String> = content
        .lines()
        .map(|line| line.expect("Something went wrong"))
        .collect();
    
    Ok(lines)
}

fn test(checker_sol: String, optimal_sol: String) -> String{
    let checker_file:Vec<String> = read_file(checker_sol).expect("Something went wrong");
    let optimal_file:Vec<String> = read_file(optimal_sol).expect("Something went wrong");

    let fail_message = String::from("Test Failed");
    let pass_message:String = String::from("Test Passed");
    
    if checker_file.len() != optimal_file.len() {
        return String::from("Test Failed");
    }

    for i in 0..checker_file.len() {
        if checker_file[i] != optimal_file[i] {
            println!("Test Failed on line {}", i+1);
            println!("Actual Output: {}", optimal_file[i]);
            println!("Expected Output: {}", checker_file[i]);
            return fail_message;
        }
    }

    return pass_message;
}

#[pyfunction]
fn checker() -> Py<PyAny> {
    let cwd = std::env::current_dir();
    println!("{:?}", cwd);

    let res = test(String::from("data/checker_sol.txt"), String::from("data/optimal_sol.txt"));

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
