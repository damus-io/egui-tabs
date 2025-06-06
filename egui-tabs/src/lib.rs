use egui::{vec2, Color32, CursorIcon, Layout, Sense};

pub struct Tabs {
    cols: i32,
    height: f32,
    sense: Sense,
    layout: Layout,
    clip: bool,
    selected_bg: TabColor,
    selected_fg: TabColor,
    hover_bg: TabColor,
    hover_fg: TabColor,
    selected: Option<i32>,
}

pub enum VisualsVariant {
    HoverBackground,
    HoverForeground,
    SelectedBackground,
    SelectedForeground,
}

pub enum TabColor {
    Nothing,
    VisualsDefault(VisualsVariant),
    Custom(Color32),
}

impl TabColor {
    pub fn custom(color: Color32) -> Self {
        TabColor::Custom(color)
    }

    pub fn visuals(variant: VisualsVariant) -> Self {
        TabColor::VisualsDefault(variant)
    }

    pub fn none() -> Self {
        TabColor::Nothing
    }

    pub fn color(&self, visuals: &egui::Visuals) -> Option<Color32> {
        match self {
            TabColor::Nothing => None,
            TabColor::VisualsDefault(VisualsVariant::HoverBackground) => {
                Some(visuals.widgets.hovered.bg_fill)
            }
            TabColor::VisualsDefault(VisualsVariant::HoverForeground) => {
                Some(visuals.widgets.hovered.fg_stroke.color)
            }
            TabColor::VisualsDefault(VisualsVariant::SelectedBackground) => {
                Some(visuals.selection.bg_fill)
            }
            TabColor::VisualsDefault(VisualsVariant::SelectedForeground) => {
                Some(visuals.selection.stroke.color)
            }
            TabColor::Custom(c) => Some(*c),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct TabState {
    ind: i32,
    hovered_tab: i32,
    selected_tab: i32,
}

impl TabState {
    pub fn is_hovered(&self) -> bool {
        self.hovered_tab == self.ind
    }

    pub fn is_selected(&self) -> bool {
        self.selected_tab == self.ind
    }

    pub fn hovered_tab(&self) -> Option<i32> {
        if self.hovered_tab < 0 {
            None
        } else {
            Some(self.hovered_tab)
        }
    }

    pub fn selected_tab(&self) -> Option<i32> {
        if self.selected_tab < 0 {
            None
        } else {
            Some(self.selected_tab)
        }
    }

    pub fn index(&self) -> i32 {
        self.ind
    }
}

#[derive(Default, Debug)]
pub struct TabResponse<T> {
    inner: Vec<egui::InnerResponse<T>>,
    hovered: Option<i32>,
    selected: Option<i32>,
}

impl<T> TabResponse<T> {
    pub fn hovered(&self) -> Option<i32> {
        self.hovered
    }

    pub fn selected(&self) -> Option<i32> {
        self.selected
    }

    pub fn inner(self) -> Vec<egui::InnerResponse<T>> {
        self.inner
    }
}

impl Tabs {
    pub fn new(cols: i32) -> Self {
        let height = 20.0;
        let sense = Sense::click();
        let layout = Layout::default();
        let clip = false;
        let hover_bg = TabColor::visuals(VisualsVariant::HoverBackground);
        let hover_fg = TabColor::visuals(VisualsVariant::HoverForeground);
        let selected_bg = TabColor::visuals(VisualsVariant::SelectedBackground);
        let selected_fg = TabColor::visuals(VisualsVariant::SelectedForeground);
        let selected: Option<i32> = None;

        Tabs {
            cols,
            height,
            sense,
            layout,
            clip,
            selected_bg,
            selected_fg,
            hover_bg,
            hover_fg,
            selected,
        }
    }

    pub fn hover_bg(mut self, bg_fill: TabColor) -> Self {
        self.hover_bg = bg_fill;
        self
    }

    pub fn hover_fg(mut self, hover_fg: TabColor) -> Self {
        self.hover_fg = hover_fg;
        self
    }

    pub fn selected_fg(mut self, selected_fg: TabColor) -> Self {
        self.selected_fg = selected_fg;
        self
    }

    pub fn selected_bg(mut self, bg_fill: TabColor) -> Self {
        self.selected_bg = bg_fill;
        self
    }

    /// The initial selection value
    pub fn selected(mut self, selected: i32) -> Self {
        self.selected = Some(selected);
        self
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

    pub fn show<F, R>(&mut self, ui: &mut egui::Ui, mut add_tab: F) -> TabResponse<R>
    where
        F: FnMut(&mut egui::Ui, TabState) -> R,
    {
        let mut inner = Vec::with_capacity(self.cols as usize);

        if self.cols == 0 {
            return TabResponse {
                selected: None,
                hovered: None,
                inner,
            };
        }

        let mut rect = ui.available_rect_before_wrap();
        let cell_width = rect.width() / self.cols as f32;
        rect.set_width(cell_width);
        rect.set_height(self.height);

        let tabs_id = ui.id().with("tabs");
        let hover_id = tabs_id.with("hover");
        let mut any_hover = false;

        let mut selected: Option<i32> = self.selected;
        let mut hovered: Option<i32> = None;

        for ind in 0..self.cols {
            let resp = ui.allocate_rect(rect, self.sense);

            let selected_tab = if resp.clicked() {
                selected = Some(ind);
                ui.ctx().data_mut(|d| d.insert_temp(tabs_id, ind));
                ind
            } else {
                ui.ctx()
                    .data(|d| d.get_temp::<i32>(tabs_id))
                    .or(self.selected)
                    .unwrap_or(-1)
            };

            let hovered_tab = if resp.hovered() {
                any_hover = true;
                hovered = Some(ind);
                ui.ctx().data_mut(|d| d.insert_temp(hover_id, ind));
                ind
            } else {
                ui.ctx().data(|d| d.get_temp::<i32>(hover_id)).unwrap_or(-1)
            };

            let tab_state = TabState {
                ind,
                selected_tab,
                hovered_tab,
            };

            if tab_state.is_selected() {
                selected = Some(ind);
                if let Some(c) = self.selected_bg.color(ui.visuals()) {
                    ui.painter().rect_filled(rect, 0.0, c);
                }
            } else if tab_state.is_hovered() {
                hovered = Some(ind);
                ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
                if let Some(c) = self.hover_bg.color(ui.visuals()) {
                    ui.painter().rect_filled(rect, 0.0, c);
                }
            }

            let mut child_ui =
                ui.new_child(egui::UiBuilder::new().layout(self.layout).max_rect(rect));
            if self.clip {
                let margin = egui::Vec2::splat(ui.visuals().clip_rect_margin);
                let margin = margin.min(0.5 * ui.spacing().item_spacing);
                let clip_rect = rect.expand2(margin);
                child_ui.set_clip_rect(clip_rect.intersect(child_ui.clip_rect()));
            }

            // set foreground colors if we have them
            if tab_state.is_selected() {
                if let Some(c) = self.selected_fg.color(ui.visuals()) {
                    child_ui.style_mut().visuals.override_text_color = Some(c);
                }
            } else if tab_state.is_hovered() {
                if let Some(c) = self.hover_fg.color(ui.visuals()) {
                    child_ui.style_mut().visuals.override_text_color = Some(c);
                }
            }

            let user_value = add_tab(&mut child_ui, tab_state);
            /*
            let child_rect = child_ui.min_rect();
            let resp = child_ui.interact(child_rect, child_ui.id(), self.sense);

            if resp.hovered() {
                ui.painter()
                    .rect_filled(child_rect, 0.0, egui::Color32::RED);
                ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
                hovered = Some(ind);
                any_hover = true;
            }

            if resp.clicked() {
                ui.painter()
                    .rect_filled(child_rect, 0.0, egui::Color32::BLUE);
                selected = Some(ind);
                ui.ctx().data_mut(|d| d.insert_temp(tabs_id, ind));
            }
            */

            inner.push(egui::InnerResponse::new(user_value, resp));

            rect = rect.translate(vec2(cell_width, 0.0))
        }

        if !any_hover {
            ui.data_mut(|data| data.remove::<i32>(hover_id));
            hovered = None;
        }

        TabResponse {
            selected,
            hovered,
            inner,
        }
    }
}
