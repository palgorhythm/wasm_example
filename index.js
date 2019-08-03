const rust = import('./wasm_example');

rust.then(rustModule => {
  rustModule.create_stuff(); //* calling stuff we defined in rust, woohoo!!!
  rustModule.run_alert('ja🅱️ascript lol');
});
