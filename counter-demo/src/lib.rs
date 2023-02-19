use pyo3::prelude::*;

#[pyclass]
#[derive(Clone)]
#[derive(Copy)]
enum Reset {
    NewlinesReset,
    SpacesReset,
    NoReset,
}

#[pyclass]
struct Counter {
    what: char,
    min_number: u64,
    reset: Reset, 
}

#[pymethods]
impl Counter {
    #[new]
    fn new(what: char, min_number: u64, reset: Reset) -> Self {
        Counter{what: what, min_number: min_number, reset: reset}
    }
    
    fn has_count(
        &self,
        data: &str,
    ) -> bool {
        let mut current_count = 0;
        for c in data.chars() {
            if current_count >= self.min_number {
                return true;
            }
            match (c, self.reset) {
                ('\n', Reset::NewlinesReset) | (' ', Reset::SpacesReset)=> {
                    current_count = 0;
                }
                _ => {}
            }
            if c != self.what {
                continue;
            }
            current_count += 1;
        }
        false
    }
}

#[pymodule]
fn counter(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Counter>()?;
    m.add_class::<Reset>()?;
    Ok(())
}