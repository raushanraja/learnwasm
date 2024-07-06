use std::usize;

use wasm_bindgen::prelude::*;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello: {}", name));
}

#[wasm_bindgen]
pub fn sum(left: usize, right: usize) {
    alert(&format!("Sum is: {}", add(left, right)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
