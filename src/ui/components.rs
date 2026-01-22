//! Reusable UI components and widget helpers
//!
//! Provides info cards, labels, grids, and updatable widget structure
//! for consistent UI styling across tabs.

use gtk4::prelude::*;
use gtk4::{Box, Frame, Grid, Label, Orientation};

/// Reusable UI component builder
pub struct InfoCard;

impl InfoCard {
    /// Creates a framed information card with title
    ///
    /// # Arguments
    ///
    /// * `title` - Card title text (accepts markup)
    ///
    /// # Returns
    ///
    /// Tuple of (Frame, content Box) for adding widgets
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

/// Creates left-aligned information label
///
/// # Arguments
///
/// * `text` - Label text content
///
/// # Returns
///
/// Configured Label widget
pub fn create_info_label(text: &str) -> Label {
    let label = Label::new(Some(text));
    label.set_halign(gtk4::Align::Start);
    label
}

/// Container for widget references requiring periodic updates
///
/// Stores Label references for battery metrics updated by timer
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

/// Creates a grid with homogeneous columns
///
/// # Returns
///
/// Configured Grid widget
pub fn create_row_grid() -> Grid {
    let grid = Grid::new();
    grid.set_column_spacing(8);
    grid.set_column_homogeneous(true);
    grid.set_row_homogeneous(true);
    grid
}
/// Creates vertical expanding spacer
///
/// # Returns
///
/// Box configured to expand vertically
pub fn create_vertical_spacer() -> Box {
    let spacer = Box::new(Orientation::Vertical, 0);
    spacer.set_vexpand(true);
    spacer
}

/// Creates vertical box with standard margins
///
/// # Arguments
///
/// * `spacing` - Vertical spacing between children
///
/// # Returns
///
/// Configured Box widget
pub fn create_content_box(spacing: i32) -> Box {
    let content_box = Box::new(Orientation::Vertical, spacing);
    content_box.set_margin_top(10);
    content_box.set_margin_bottom(10);
    content_box.set_margin_start(10);
    content_box.set_margin_end(10);
    content_box
}
