// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors

//! Unwrap helpers that exit instead of panicking.

pub fn ok_val<T, E>(r: Result<T, E>) -> T {
    match r {
        Ok(v) => v,
        Err(_) => std::process::exit(1),
    }
}

pub fn some_val<T>(o: Option<T>) -> T {
    match o {
        Some(v) => v,
        None => std::process::exit(1),
    }
}
