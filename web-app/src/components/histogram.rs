cfg_if::cfg_if! {
    if #[cfg(any(feature = "csr", feature = "hydrate"))] {
        #[wasm_bindgen]
        extern "C" {
            type ApexCharts;
        }
    }
}
