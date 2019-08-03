const rust = import('./wasm_example');

rust.then(rustModule => {
  rustModule.create_stuff(); //* calling stuff we defined in rust, woohoo!!!
  rustModule.run_alert(
    '=========HEY WASSUP THIS IS FROM ja🅱️ascript lol=========='
  );
});
