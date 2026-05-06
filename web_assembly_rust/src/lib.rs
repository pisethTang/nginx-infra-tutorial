use wasm_bindgen::prelude::*;



#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}


// any computationally expensive stuff ... (blockchain, graphics engine, etc)
fn fib(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib(n-1) + fib(n-2),
    }
}







#[wasm_bindgen]
pub fn greet(n: u32){
    let num = fib(n);
    alert(&format!("Hello fib{n}: {num}"));
}