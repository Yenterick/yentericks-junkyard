use std::collections::HashMap;

use crate::cli::events::{page::Page, page_status::PageStatus};

pub struct SidebarState {
    pub current_page: Page,
    pub page_status: HashMap<Page, PageStatus>,
}

impl SidebarState {
    pub fn new() -> SidebarState {
        let mut page_status = HashMap::new();

        page_status.insert(Page::TemplateSelection, PageStatus::InProcess);
        page_status.insert(Page::Models, PageStatus::Locked);
        page_status.insert(Page::Fields, PageStatus::Locked);
        page_status.insert(Page::ProjectConfiguration, PageStatus::Locked);
        page_status.insert(Page::Generation, PageStatus::Locked);

        SidebarState {
            current_page: Page::TemplateSelection,
            page_status,
        }
    }
}
