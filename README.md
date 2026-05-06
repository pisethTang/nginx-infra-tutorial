# Rust tutorial 


### Terminologies

// every executable Rust program must contain a function with the name `main`
// `rustc` := checks and compiles your program
// `cargo` := Rust's build system and package manager 
// `Dependencies` := the library your code needs. 




### References:
1. [Caleb Curry video](https://www.youtube.com/watch?v=jAm7xrRxEUE&list=PL_c9BZzLwBRIymgB73pHXIgazpB-uszKU&index=1)
2. [Uni Brown](https://rust-book.cs.brown.edu/ch01-03-hello-cargo.html)
3. [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
4. [Let's Get Rusty video](https://www.youtube.com/watch?v=DhiHhQimSa4)
    The person mentions that if I want to learn rust to build backend services, then I would need to learn the following:
        1. async excutors 
        2. locks and atomics
        3. message passing via channels  
        4. Actix 
        5. logging and tracing 
        6. error handling and reporting 
        7. sqlx, sea-orm, Diesel




```

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str){
    alert(&format!("Hello {name}"));
}
```
