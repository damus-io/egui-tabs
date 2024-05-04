
# egui-tabs

A tab view for egui

## Usage

warning: API in in alpha and is currently unstable 

full example: [examples/basic.rs](basic)

```
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
