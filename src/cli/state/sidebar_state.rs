use std::vec::Vec;

use crate::cli::events::{page_status::PageStatus, pages::Pages};

pub struct SidebarState {
    pub current_page: Pages,
    pub page_status: Vec<(Pages, PageStatus)>,
}

impl SidebarState {
    pub fn new() -> SidebarState {
        let mut page_status = Vec::new();

        page_status.push((Pages::TemplateSelection, PageStatus::InProcess));
        page_status.push((Pages::Models, PageStatus::Locked));
        page_status.push((Pages::Fields, PageStatus::Locked));
        page_status.push((Pages::ProjectConfiguration, PageStatus::Locked));
        page_status.push((Pages::Generation, PageStatus::Locked));

        SidebarState {
            current_page: Pages::TemplateSelection,
            page_status,
        }
    }
}
