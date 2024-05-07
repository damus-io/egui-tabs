
# egui-tabs

A tab view for egui

[Demo](https://damus-io.github.io/egui-tabs/)

## Usage

warning: API in in alpha and is currently unstable 

[example](examples/basic.rs)

```rust
Tabs::new(3).show(ui, |ui, ind| {
    if ind == 0 {
        ui.label("Tab A");
    } else if ind == 1 {
        ui.label("Tab B");
    } else if ind == 2 {
        ui.label("Tab C");
    }
});
``` 
