
# egui-tabs

A tab view for egui

[Demo](https://damus-io.github.io/egui-tabs/)

## Usage

[example](examples/basic/src/main.rs)

```rust
Tabs::new(3).show(ui, |ui, state| {
    let ind = state.index();

    let txt = if ind == 0 {
        "Tab A"
    } else if ind == 1 {
        "Tab B"
    } else if ind == 2 {
        "Tab C"
    } else {
        ""
    };

    ui.add(egui::Label::new(txt).selectable(false));
});
``` 
