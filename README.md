# Wasm tutorial 

- Rust-based

- frontend in rust and wasm frameworks (faster than most js frameworks)
- svelte and solid.js 
- browser's ability to render will be the bottleneck (not about )




#### References

1. From Greg Johnson, over the last six months, frontend frameworks written in Rust and WebAssembly have begun overturning the old narrative that WASM is too slow for DOM rendering. In this video we'll take a look at several Rust/WASM frameworks to try to understand the truth about Rust/WASM performance.

EDIT: To clarify about the memory usage: Something changed in the benchmark at some point in the way memory use is being reported, and I'm not sure why that is. If you go back to slightly earlier runs you can see better memory comparisons in which, for example, Sycamore is significantly more efficient than Solid, Yew much more memory efficient than React, etc. I'm pretty sure it was the benchmarking that changed, not the frameworks.

Check results here, for example: https://krausest.github.io/js-framewo...

<u>Links</u>:
- js-framework-benchmark: https://github.com/krausest/js-framew...
- Leptos: https://github.com/leptos-rs/leptos
- Dioxus: https://github.com/DioxusLabs/dioxus
- Sycamore: https://github.com/sycamore-rs/sycamore
- Yew: https://github.com/yewstack/yew