use egui::{vec2, Layout, Sense};

pub struct Tabs {
    cols: i32,
    height: f32,
    sense: Sense,
    layout: Layout,
    clip: bool,
}

impl Tabs {
    pub fn new(cols: i32) -> Self {
        let height = 20.0;
        let sense = Sense::click();
        let layout = Layout::default();

        let clip = false;
        Tabs {
            cols,
            height,
            sense,
            layout,
            clip,
        }
    }

    pub fn sense(mut self, sense: Sense) -> Self {
        self.sense = sense;
        self
    }

    pub fn clip(mut self, clip: bool) -> Self {
        self.clip = clip;
        self
    }

    /// The layout of the content in the cells
    pub fn layout(mut self, layout: Layout) -> Self {
        self.layout = layout;
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    pub fn show<F>(&mut self, ui: &mut egui::Ui, add_tab: F)
    where
        F: Fn(&mut egui::Ui, i32),
    {
        if self.cols == 0 {
            return;
        }

        let mut rect = ui.available_rect_before_wrap();
        let cell_width = rect.max.x / self.cols as f32;
        rect.set_width(cell_width);
        rect.set_height(self.height);

        let tabs_id = ui.id().with("tabs");

        for ind in 0..self.cols {
            let resp = ui.allocate_rect(rect, self.sense);

            if resp.clicked() {
                ui.ctx().data_mut(|d| d.insert_temp(tabs_id, ind));
            }

            let is_selected = ui
                .ctx()
                .data_mut(|d| d.get_temp::<i32>(tabs_id).unwrap_or(-1))
                == ind;

            if is_selected {
                ui.painter()
                    .rect_filled(rect, 0.0, ui.visuals().selection.bg_fill);
            } else if resp.hovered() {
                ui.painter()
                    .rect_filled(rect, 0.0, ui.visuals().widgets.hovered.bg_fill);
            }

            let mut child_ui = ui.child_ui(rect, self.layout);
            if self.clip {
                let margin = egui::Vec2::splat(ui.visuals().clip_rect_margin);
                let margin = margin.min(0.5 * ui.spacing().item_spacing);
                let clip_rect = rect.expand2(margin);
                child_ui.set_clip_rect(clip_rect.intersect(child_ui.clip_rect()));
            }

            if is_selected {
                let stroke_color = child_ui.style().visuals.selection.stroke.color;
                child_ui.style_mut().visuals.override_text_color = Some(stroke_color);
            }

            add_tab(&mut child_ui, ind);

            rect = rect.translate(vec2(cell_width, 0.0))
        }
    }

    /*
        pub fn _show_strip<F>(&self, ui: &mut egui::Ui, add_tab: F)
        where
            F: Fn(&mut egui::Ui, i32),
        {
            use egui_extras::{Size, StripBuilder, StripLayoutFlags};

            egui::Frame::none().show(ui, |ui| {
                ui.set_height(self.height);

                StripBuilder::new(ui)
                    .sizes(Size::remainder(), self.cols as usize)
                    .clip(true)
                    .sense(Sense::click())
                    .horizontal(|mut strip| {
                        for ind in 0..self.cols {
                            let flags = StripLayoutFlags {
                                hovered: ind == 1,
                                ..StripLayoutFlags::default()
                            };

                            let (_rect, resp) = strip.cell_with_flags(flags, |ui| {
                                add_tab(ui, ind);
                            });

                            /*
                            let is_tab_hovered = response.as_ref().map_or(false, |r| r.hovered());
                            if is_row_hovered {
                                self.layout.ui.data_mut(|data| {
                                    data.insert_temp(self.hovered_row_index_id, row_index)
                                });
                            }
                            */
                        }
                    });
            });
        }
    */
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
