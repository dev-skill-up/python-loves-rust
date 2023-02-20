```{=latex}
\usepackage{hyperref}
\usepackage{graphicx}
\usepackage{listings}
\usepackage{textcomp}
\usepackage{fancyvrb}

\newcommand{\passthrough}[1]{\lstset{mathescape=false}#1\lstset{mathescape=true}}
\newcommand{\tightlist}{}
```

```{=latex}
\title{PyO3: Python Loves Rust}
\author{Moshe Zadka -- https://cobordism.com}
\date{}

\begin{document}
\begin{titlepage}
\maketitle
\end{titlepage}

\frame{\titlepage}
```

```{=latex}
\begin{frame}
\frametitle{Acknowledgement of Country}

Belmont (in San Francisco Bay Area Peninsula)

Ancestral homeland of the Ramaytush Ohlone people

\end{frame}
```

I live in Belmont,
in the San Francisco Bay Area Peninsula.
I wish to acknowledge it as the
ancestral homeland
of the
Ramaytush Ohlone people.

## Short Intro to Rust

```{=latex}
\begin{frame}
\frametitle{Rust: Intro}

\pause

What \pause

Why \pause

How

\end{frame}
```

### What is Rust?

```{=latex}
\begin{frame}
\frametitle{Rust: What?}

\pause

Low-level \pause

Zero-cost abstractions \pause

Memory safe!

\end{frame}
```

### Why is Rust?

```{=latex}
\begin{frame}
\frametitle{Rust: Why?}

\pause

Performance \pause

Safety \pause

"Low-level parsing"

\end{frame}
```

### Counting characters

```{=latex}
\begin{frame}
\frametitle{Toy Example: Counting}

\pause

Check whether character appears more than X times \pause

Optionally, reset counts on spaces/newlines \pause

"Toy example" \pause

Just interesting enough

\end{frame}
```

### Enum

```{=latex}
\begin{frame}[fragile]
\frametitle{Rust example: Enum}

\begin{lstlisting}
enum Reset {
    NewlinesReset,
    SpacesReset,
    NoReset,
}
\end{lstlisting}
\end{frame}
```

### Struct

```{=latex}
\begin{frame}[fragile]
\frametitle{Rust example: Struct}

\begin{lstlisting}
struct Counter {
    what: char,
    min_number: u64,
    reset: Reset, 
}
\end{lstlisting}
\end{frame}
```

### Implementation

```{=latex}
\begin{frame}[fragile]
\frametitle{Rust example: Impl}

\begin{lstlisting}
impl Counter {
    fn has_count(
        &self,
        data: &str,
    ) -> bool {
        has_count(self, data.chars())
    }
\end{lstlisting}
\end{frame}
```

### Function

```{=latex}
\begin{frame}[fragile]
\frametitle{Rust example: Loop}

\begin{lstlisting}
fn has_count(cntr: &Counter, chars: std::str::Chars) -> bool {
    let mut current_count : u64 = 0;
    for c in chars {
        if got_count(cntr, c, &mut current_count) {
            return true;
        }
    }
    false
}
\end{lstlisting}
\end{frame}
```

### Counting

```{=latex}
\begin{frame}[fragile]
\frametitle{Rust example: Counting}

\begin{lstlisting}
fn got_count(cntr: &Counter, c: char, current_count: &mut u64) -> bool {
    if *current_count >= cntr.min_number {
        return true;
    }
    maybe_reset(cntr, c, current_count);
    maybe_incr(cntr, c, current_count);
    false
}
\end{lstlisting}
\end{frame}
```

### Reset

```{=latex}
\begin{frame}[fragile]
\frametitle{Rust example: Reset}

\begin{lstlisting}
fn maybe_reset(cntr: &Counter, c: char, current_count: &mut u64) -> () {
    match (c, cntr.reset) {
        ('\n', Reset::NewlinesReset) | (' ', Reset::SpacesReset)=> {
            *current_count = 0;
        }
        _ => {}
    };
}
\end{lstlisting}
\end{frame}
```

### Increment

```{=latex}
\begin{frame}[fragile]
\frametitle{Rust example: Increment}

\begin{lstlisting}
fn maybe_incr(cntr: &Counter, c: char, current_count: &mut u64) -> (){
    if c == cntr.what {
        *current_count += 1;
    };
}
\end{lstlisting}
\end{frame}
```

## PyO3


```{=latex}
\begin{frame}[fragile]
\frametitle{PyO3}

Inline \pause

Modify together

\end{frame}
```

### Include

```{=latex}
\begin{frame}[fragile]
\frametitle{PyO3 example: Include}

\begin{lstlisting}
use pyo3::prelude::*;
\end{lstlisting}
\end{frame}
```

### Wrap enum

```{=latex}
\begin{frame}[fragile]
\frametitle{PyO3 example: Wrap enum}

\begin{lstlisting}
#[pyclass]
#[derive(Clone)]
#[derive(Copy)]
enum Reset {
    /* ... */
}
\end{lstlisting}
\end{frame}
```

### Wrap struct

```{=latex}
\begin{frame}[fragile]
\frametitle{PyO3 example: Wrap struct}

\begin{lstlisting}
#[pyclass]
struct Counter {
    /* ... */
}
\end{lstlisting}
\end{frame}
```

### Wrap impl

```{=latex}
\begin{frame}[fragile]
\frametitle{PyO3 example: Wrap impl}

\begin{lstlisting}
#[pymethods]
impl Counter {
    #[new]
    fn new(what: char, min_number: u64, reset: Reset) -> Self {
        Counter{what: what, min_number: min_number, reset: reset}
    }
    /* ... */
}
\end{lstlisting}
\end{frame}
```

### Define module

```{=latex}
\begin{frame}[fragile]
\frametitle{PyO3 example: Define module}

\begin{lstlisting}
#[pymodule]
fn counter(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Counter>()?;
    m.add_class::<Reset>()?;
    Ok(())
}
\end{lstlisting}
\end{frame}
```

### Maturin develop

```{=latex}
\begin{frame}[fragile]
\frametitle{Maturin develop}

\begin{lstlisting}
(venv)$ maturin develop
\end{lstlisting}
\end{frame}
```

### Maturin develop

```{=latex}
\begin{frame}[fragile]
\frametitle{Maturin build}

\begin{lstlisting}
(venv)$ maturin build
\end{lstlisting}
\end{frame}
```

## Python

```{=latex}
\begin{frame}[fragile]
\frametitle{Python}

Use!

\end{frame}
```

### Import

```{=latex}
\begin{frame}[fragile]
\frametitle{Import}
```


```python
import counter
```

```{=latex}
\end{frame}
```

### Construct

```{=latex}
\begin{frame}[fragile]
\frametitle{Constructor}
```


```python
cntr = counter.Counter('c', 3, counter.Reset.NewlinesReset)
```

```{=latex}
\end{frame}
```

### Call

```{=latex}
\begin{frame}[fragile]
\frametitle{Call}
```


```python
cntr.has_count("hello-c-c-c-goodbye")
```




    True



```{=latex}
\end{frame}
```

### Call

```{=latex}
\begin{frame}[fragile]
\frametitle{Call}
```


```python
cntr.has_count("hello-c-c-\nc-goodbye")
```




    False



```{=latex}
\end{frame}
```

## Conclusion

```{=latex}
\begin{frame}[fragile]
\frametitle{Take-aways}

Why?

\end{frame}
```

### Rust and Python is easy

```{=latex}
\begin{frame}[fragile]
\frametitle{Rust + Python}

Easy!

\end{frame}
```

### Use each one for its purposes

```{=latex}
\begin{frame}[fragile]
\frametitle{Differences}

\pause

Rust: \pause High-performance, \pause safe, \pause learning curve, \pause awkward prototyping

\pause

Python: \pause Easy, \pause tight iteration, \pause Speed cap

\end{frame}
```

```{=latex}
\begin{frame}[fragile]
\frametitle{Combined}

\pause

Prototype in Python \pause

Move perf bottlenecks to Rust \pause

\end{frame}
```

```{=latex}
\begin{frame}[fragile]
\frametitle{Stronger together}

\pause

Deployment \pause

Development \pause

Enjoy!


\end{frame}
```

```{=latex}
\end{document}
```
