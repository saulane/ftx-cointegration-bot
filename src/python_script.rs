use std::fs;
use pyo3::{prelude::*, types::{IntoPyDict, PyModule}};
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

pub async fn get_new_coint_pairs() {
    let script = fs::read_to_string("get_coint_pairs.py").expect("Problem opening python script");

    let gil = Python::acquire_gil();
    let py = gil.python();
    
    //println!("RUNNING :\n[\n{}]", script);
    py.run(&script, None, None);
}
