use gtk4::prelude::*;
use gtk4::{Box, Frame, Grid, Label, Orientation};

/// Composants réutilisables pour l'interface
pub struct InfoCard;

impl InfoCard {
    /// Crée une carte d'information avec titre et contenu
    pub fn create(title: &str) -> (Frame, Box) {
        let frame = Frame::new(None);

        let main_box = Box::new(Orientation::Vertical, 0);
        main_box.set_margin_top(6);
        main_box.set_margin_bottom(6);
        main_box.set_margin_start(8);
        main_box.set_margin_end(8);

        // Créer le titre en gras et plus grand
        let title_label = Label::new(None);
        title_label.set_markup(&format!(
            "<span size='large' weight='bold'>{}</span>",
            title
        ));
        title_label.set_halign(gtk4::Align::Start);
        title_label.set_margin_bottom(8);
        main_box.append(&title_label);

        let content_box = Box::new(Orientation::Vertical, 4);
        main_box.append(&content_box);

        frame.set_child(Some(&main_box));
        (frame, content_box)
    }
}

/// Crée le label d'information aligné à gauche
pub fn create_info_label(text: &str) -> Label {
    let label = Label::new(Some(text));
    label.set_halign(gtk4::Align::Start);
    label
}

/// Structure pour maintenir les références aux widgets mis à jour
pub struct UpdatableWidgets {
    pub power_source_value: Label,
    pub status_value: Label,
    pub capacity_label: Label,
    pub health_label: Label,
    pub voltage_value: Label,
    pub current_value: Label,
    pub power_value: Label,
    pub charge_now_value: Label,
    pub threshold_start_label: Option<Label>,
    pub threshold_stop_label: Label,
    pub alarm_label: Option<Label>,
    pub service_label: Label,
}

/// Crée une grille de ligne avec colonnes homogènes
pub fn create_row_grid() -> Grid {
    let grid = Grid::new();
    grid.set_column_spacing(8);
    grid.set_column_homogeneous(true);
    grid.set_row_homogeneous(true);
    grid
}
/// Crée un spacer vertical expansible
pub fn create_vertical_spacer() -> Box {
    let spacer = Box::new(Orientation::Vertical, 0);
    spacer.set_vexpand(true);
    spacer
}

/// Crée une Box verticale avec marges standards
pub fn create_content_box(spacing: i32) -> Box {
    let content_box = Box::new(Orientation::Vertical, spacing);
    content_box.set_margin_top(10);
    content_box.set_margin_bottom(10);
    content_box.set_margin_start(10);
    content_box.set_margin_end(10);
    content_box
}