use crate::pages::Page;

pub struct State {
    using_page: Option<Box<dyn Page>>,
}

impl State {
    pub fn new() -> Self {
        State { using_page: None }
    }

    pub fn set_page(&mut self, page: Box<dyn Page>) {
        self.using_page = Some(page);
    }

    pub fn get_page(&mut self) -> Option<&mut Box<dyn Page>> {
        self.using_page.as_mut()
    }
}
