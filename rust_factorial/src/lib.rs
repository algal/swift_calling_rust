#[crate_type = "staticlib"]

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

/*
We need #[no_mangle] to ensure the function name is unchanged in the
compiled binary.

We need extern to ensure the the function is exposed in the compiled
binary.
*/
#[no_mangle]
pub extern "C" fn fact(n:u64) -> u64 {
    let mut result:u64 = 1;
    for i  in 1..=n  {
        result = result * i;
    }
    return result
}
