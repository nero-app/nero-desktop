use crate::error::Result;
use crate::fetch::Fetch;

pub const LOAD_MORE_MARGIN: f32 = 320.0;

pub trait Page {
    type Item;

    fn items(&self) -> &[Self::Item];
    fn has_next_page(&self) -> bool;
    fn extend(&mut self, next: Self);
}

impl<T> Page for nero_extensions::types::Page<T> {
    type Item = T;

    fn items(&self) -> &[Self::Item] {
        &self.items
    }

    fn has_next_page(&self) -> bool {
        self.has_next_page
    }

    fn extend(&mut self, next: Self) {
        self.items.extend(next.items);
        self.has_next_page = next.has_next_page;
    }
}

pub struct Paginated<T> {
    page_index: u16,
    loading_more: bool,
    data: Fetch<T>,
}

impl<T> Default for Paginated<T> {
    fn default() -> Self {
        Self {
            page_index: 0,
            loading_more: false,
            data: Fetch::default(),
        }
    }
}

impl<T: Page> Paginated<T> {
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn can_load_more(&self) -> bool {
        !self.loading_more && self.data.loaded().is_some_and(Page::has_next_page)
    }

    pub fn is_loading(&self) -> bool {
        self.loading_more || matches!(self.data, Fetch::Loading)
    }

    pub fn is_loading_more(&self) -> bool {
        self.loading_more
    }

    pub fn start_loading_more(&mut self) -> Option<u16> {
        if !self.can_load_more() {
            return None;
        }

        self.loading_more = true;

        Some(self.page_index.saturating_add(2))
    }

    pub fn loaded(&mut self, result: Result<T>) {
        let was_loading_more = self.loading_more;
        self.loading_more = false;

        match (&mut self.data, result) {
            (Fetch::Loaded(current), Ok(next)) if was_loading_more => {
                current.extend(next);
                self.page_index = self.page_index.saturating_add(1);
            }
            (Fetch::Loaded(_), Err(_)) if was_loading_more => {}
            (_, result) => self.data = result.into(),
        }
    }

    pub fn items(&self) -> impl Iterator<Item = &T::Item> {
        self.data
            .loaded()
            .into_iter()
            .flat_map(|data| data.items().iter())
    }

    pub fn data(&self) -> &Fetch<T> {
        &self.data
    }
}

#[cfg(test)]
mod tests {
    use super::{Page, Paginated};
    use crate::error::Error;

    #[derive(Debug, PartialEq, Eq)]
    struct TestPage {
        items: Vec<u8>,
        has_next_page: bool,
    }

    impl Page for TestPage {
        type Item = u8;

        fn items(&self) -> &[Self::Item] {
            &self.items
        }

        fn has_next_page(&self) -> bool {
            self.has_next_page
        }

        fn extend(&mut self, next: Self) {
            self.items.extend(next.items);
            self.has_next_page = next.has_next_page;
        }
    }

    #[test]
    fn loading_more_advances_only_after_success() {
        let mut pages = Paginated::default();
        pages.loaded(Ok(TestPage {
            items: vec![1],
            has_next_page: true,
        }));

        assert_eq!(pages.start_loading_more(), Some(2));
        pages.loaded(Ok(TestPage {
            items: vec![2],
            has_next_page: false,
        }));

        assert_eq!(pages.items().copied().collect::<Vec<_>>(), vec![1, 2]);
        assert!(!pages.can_load_more());
    }

    #[test]
    fn loading_more_failure_preserves_items_and_page() {
        let mut pages = Paginated::default();
        pages.loaded(Ok(TestPage {
            items: vec![1],
            has_next_page: true,
        }));

        assert_eq!(pages.start_loading_more(), Some(2));
        pages.loaded(Err(Error::Server("unavailable".into())));

        assert_eq!(pages.items().copied().collect::<Vec<_>>(), vec![1]);
        assert_eq!(pages.start_loading_more(), Some(2));
    }
}
