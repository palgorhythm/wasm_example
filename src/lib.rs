extern crate wasm_bindgen; //* add the wasm_bindgen lib
use wasm_bindgen::prelude::*; //* import everything in prelude from wasm_bindgen

#[wasm_bindgen] //* define function/obj defs for objects that exists inside of the js we're interfacing with
extern "C" {
  fn alert(s: &str); //* specify what JS function name we're "proxying" into rust

  type HTMLDocument; //*correspond to types in JS
  type Element;
  static document: HTMLDocument; //* a reference to the actual document variable in JS so we can use its methods

  #[wasm_bindgen(method)] //* tells the wasm compiler that this is a method on an object of type HTMLDocument
  fn createElement(this: &HTMLDocument, tagName: &str) -> Element;

  #[wasm_bindgen(method, getter)] //* need to specify it's a getter, bc only takes in the object and no other params
  fn body(this: &HTMLDocument) -> Element;

  #[wasm_bindgen(method, js_name = appendChild)] //* create binding for appendChild js function that we can alias to whatever name we want
  fn appendChildThatICanNameWhateverIWant(this: &Element, item: Element);

  #[wasm_bindgen(method, setter = innerHTML)] //* specify it's a setter and specify its exact name in js
  fn set_inner(this: &Element, html: &str);
}

#[wasm_bindgen]
pub fn run_alert(item: &str){ //* create rust implementation that receives the str input from the js layer. we can call it from JS
  alert(&format!("this is Rust🦀 compiled to WASM👾 and {} is from JS", item)); //* running the function alert that's implemented in js
}

#[wasm_bindgen]
pub fn create_stuff(){ //* native rust function that will use the createElement method from js and that we can call from JS
  let div = document.createElement("div");
  let p = document.createElement("p");
  p.set_inner("hi from WASM👾 in Rust🦀 ! ");
  div.appendChildThatICanNameWhateverIWant(p);
  document.body().appendChildThatICanNameWhateverIWant(div);
}
