pub mod app;
pub mod components;
pub mod gamedata;
use cfg_if::cfg_if;

cfg_if! {
if #[cfg(feature = "hydrate")] {

  use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen]
    pub fn hydrate() {
      use app::App;

      console_error_panic_hook::set_once();

      leptos::mount_to_body(App);
    }
}
}
