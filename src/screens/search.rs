use iced::task::Handle;
use iced::widget::{button, checkbox, column, container, grid, scrollable, sensor, space, text};
use iced::{padding, Element, Fill, Task};
use nero_extensions::types::{FilterCategory, SearchFilter, SeriesPage};
use nero_extensions::Extension;
use rust_i18n::t;
use std::collections::HashSet;

use crate::components::layout::sidebar_layout;
use crate::components::styles;
use crate::components::typography::{self, TextExt};
use crate::error::{Error, Result};
use crate::extensions::{ExtensionId, LoadedExtension, Registry};
use crate::fetch::Fetch;
use crate::images::Images;
use crate::pagination::{Paginated, LOAD_MORE_MARGIN};
use crate::screens::{Action, Route};
use crate::theme::PALETTE;
use crate::widgets::card::{series_card, SERIES_CARD_RATIO, SERIES_GRID_SPACING};
use crate::widgets::extension;

struct ExtensionSearch {
    extension: LoadedExtension,
    results: Paginated<SeriesPage>,
    categories: Fetch<Vec<FilterCategory>>,
    search_task: Option<Handle>,
    categories_task: Option<Handle>,
    selected: Vec<SearchFilter>,
    collapsed: HashSet<String>,
}

impl ExtensionSearch {
    fn new(extension: LoadedExtension) -> Self {
        Self {
            extension,
            results: Paginated::default(),
            categories: Fetch::Loading,
            search_task: None,
            categories_task: None,
            selected: Vec::new(),
            collapsed: HashSet::new(),
        }
    }

    fn is_selected(&self, category: &str, filter: &str) -> bool {
        self.selected.iter().any(|selected| {
            selected.id == category && selected.values.iter().any(|value| value == filter)
        })
    }

    fn toggle_filter(&mut self, category: String, filter: String) {
        let Some(selected) = self
            .selected
            .iter_mut()
            .find(|selected| selected.id == category)
        else {
            self.selected.push(SearchFilter {
                id: category,
                values: vec![filter],
            });

            return;
        };

        match selected.values.iter().position(|value| *value == filter) {
            Some(index) => {
                selected.values.remove(index);
            }
            None => selected.values.push(filter),
        }

        self.selected.retain(|selected| !selected.values.is_empty());
    }

    fn load(&mut self, query: &str, page: u16) -> Task<Message> {
        let extension = self.extension.extension.clone();
        let id = self.extension.id.clone();
        let query = query.to_owned();
        let filters = self.selected.clone();

        let task = Task::perform(
            async move {
                extension
                    .search(&query, Some(page), filters)
                    .await
                    .map_err(|error| Error::extension("search", error))
            },
            move |result| Message::ResultsLoaded(id, result),
        );
        let (task, handle) = task.abortable();
        self.search_task = Some(handle.abort_on_drop());

        task
    }

    fn load_categories(&mut self) -> Task<Message> {
        let extension = self.extension.extension.clone();
        let id = self.extension.id.clone();

        let task = Task::perform(
            async move {
                extension
                    .filters()
                    .await
                    .map_err(|error| Error::extension("load filters", error))
            },
            move |result| Message::CategoriesLoaded(id, result),
        );
        let (task, handle) = task.abortable();
        self.categories_task = Some(handle.abort_on_drop());

        task
    }
}

pub struct Search {
    query: String,
    extensions: Vec<ExtensionSearch>,
    images: Images,
}

#[derive(Clone)]
pub enum Message {
    LoadMore,

    ResultsLoaded(ExtensionId, Result<SeriesPage>),
    CategoriesLoaded(ExtensionId, Result<Vec<FilterCategory>>),
    ImagesLoaded,

    ToggleFilter(ExtensionId, String, String),
    ToggleCategory(ExtensionId, String),

    Open(LoadedExtension, String),
}

impl Search {
    pub fn new(extensions: &Registry, images: Images, query: String) -> (Self, Task<Message>) {
        let mut search = Self {
            query,
            extensions: extensions
                .values()
                .into_iter()
                .map(ExtensionSearch::new)
                .collect(),
            images,
        };

        let load = Task::batch(search.extensions.iter_mut().flat_map(|extension| {
            [
                extension.load(&search.query, 1),
                extension.load_categories(),
            ]
        }));

        (search, load)
    }

    pub fn update(&mut self, message: Message) -> Action<Message> {
        match message {
            Message::LoadMore => {
                let query = self.query.clone();

                Action::run(Task::batch(self.extensions.iter_mut().filter_map(
                    |extension| {
                        let page = extension.results.start_loading_more()?;

                        Some(extension.load(&query, page))
                    },
                )))
            }
            Message::ResultsLoaded(id, result) => {
                let load_images = self.images.load_all(
                    result
                        .iter()
                        .flat_map(|page| page.items.iter())
                        .filter_map(|series| series.poster_resource.clone()),
                    Message::ImagesLoaded,
                );

                if let Some(extension) = self.extension_mut(&id) {
                    extension.search_task = None;
                    extension.results.loaded(result);
                }

                Action::run(load_images)
            }
            Message::CategoriesLoaded(id, result) => {
                if let Some(extension) = self.extension_mut(&id) {
                    extension.categories_task = None;
                    extension.categories = result.into();
                }

                Action::None
            }
            Message::ImagesLoaded => Action::None,
            Message::ToggleFilter(id, category, filter) => {
                if let Some(extension) = self.extension_mut(&id) {
                    extension.toggle_filter(category, filter);
                }

                Action::None
            }
            Message::ToggleCategory(id, category) => {
                if let Some(extension) = self.extension_mut(&id) {
                    if !extension.collapsed.remove(&category) {
                        extension.collapsed.insert(category);
                    }
                }

                Action::None
            }
            Message::Open(extension, series_id) => Action::Navigate(Route::Series {
                extension,
                series_id,
            }),
        }
    }

    pub fn search(&mut self, query: String) -> Task<Message> {
        self.query = query;
        Task::batch(self.extensions.iter_mut().map(|extension| {
            extension.results.reset();
            extension.load(&self.query, 1)
        }))
    }

    fn extension_mut(&mut self, id: &ExtensionId) -> Option<&mut ExtensionSearch> {
        self.extensions
            .iter_mut()
            .find(|extension| &extension.extension.id == id)
    }

    pub fn view(&self) -> Element<'_, Message> {
        sidebar_layout(self.results_view(), self.sidebar()).into()
    }

    fn results_view(&self) -> Element<'_, Message> {
        if self.extensions.is_empty() {
            return container(text(t!("media.no_extensions")).body())
                .center(Fill)
                .into();
        }

        let anything_loading = self
            .extensions
            .iter()
            .any(|extension| extension.results.is_loading());
        let empty = self
            .extensions
            .iter()
            .all(|extension| extension.results.items().next().is_none());

        if empty {
            let message = if anything_loading {
                t!("common.loading")
            } else {
                match self.problem() {
                    Some(error) => error.to_string().into(),
                    None => t!("media.no_results"),
                }
            };

            return container(text(message).body()).center(Fill).into();
        }

        let cards = grid(self.extensions.iter().flat_map(|extension| {
            extension.results.items().map(move |series| {
                series_card(
                    series,
                    series
                        .poster_resource
                        .as_ref()
                        .and_then(|resource| self.images.handle(resource)),
                    Message::Open(extension.extension.clone(), series.id.clone()),
                )
            })
        }))
        .columns(4)
        .spacing(SERIES_GRID_SPACING)
        .height(SERIES_CARD_RATIO);

        let can_load_more = self
            .extensions
            .iter()
            .any(|extension| extension.results.can_load_more());
        let item_count = self
            .extensions
            .iter()
            .map(|extension| extension.results.items().count())
            .sum::<usize>();
        let footer = (anything_loading || can_load_more).then(|| {
            let content = if anything_loading {
                Element::from(
                    container(
                        text(t!("common.loading"))
                            .label()
                            .color(PALETTE.text_muted)
                            .center()
                            .width(Fill),
                    )
                    .width(Fill)
                    .padding([8, 0]),
                )
            } else {
                Element::from(space().height(1))
            };

            sensor(content)
                .key((item_count, anything_loading))
                .anticipate(LOAD_MORE_MARGIN)
                .on_show(|_| Message::LoadMore)
        });

        scrollable(column![cards, footer].spacing(8))
            .width(Fill)
            .height(Fill)
            .into()
    }

    fn problem(&self) -> Option<&Error> {
        self.extensions
            .iter()
            .find_map(|extension| match extension.results.data() {
                Fetch::Failed(error) => Some(error),
                _ => None,
            })
    }

    fn sidebar(&self) -> Element<'_, Message> {
        let named = self.extensions.len() > 1;

        let panels = self
            .extensions
            .iter()
            .map(|extension| filters_view(extension, named));

        let panel = column(panels)
            .spacing(16)
            .padding(padding::right(32).bottom(32).left(16));

        scrollable(panel).width(Fill).height(Fill).into()
    }
}

fn filters_view(extension: &ExtensionSearch, named: bool) -> Element<'_, Message> {
    let title =
        named.then(|| Element::from(text(extension::title(&extension.extension)).heading()));

    let categories = match &extension.categories {
        Fetch::Loading => column![text(t!("common.loading")).label().color(PALETTE.text_muted)],
        Fetch::Failed(error) => {
            column![text(error.to_string()).label().color(PALETTE.text_muted)]
        }
        Fetch::Loaded(categories) if categories.is_empty() => {
            column![text(t!("filters.empty")).label()]
        }
        Fetch::Loaded(categories) => column(
            categories
                .iter()
                .map(|category| category_view(extension, category)),
        ),
    }
    .spacing(12);

    column![title, categories].spacing(12).into()
}

fn category_view<'a>(
    extension: &'a ExtensionSearch,
    category: &'a FilterCategory,
) -> Element<'a, Message> {
    let id = &extension.extension.id;

    let header = button(text(&category.display_name).body().width(Fill))
        .on_press(Message::ToggleCategory(id.clone(), category.id.clone()))
        .padding(4)
        .width(Fill)
        .style(styles::link_button);

    let filters = (!extension.collapsed.contains(&category.id))
        .then(|| category.filters.iter())
        .into_iter()
        .flatten()
        .map(move |filter| {
            let key = (id.clone(), category.id.clone(), filter.id.clone());

            checkbox(extension.is_selected(&category.id, &filter.id))
                .label(&filter.display_name)
                .size(18.0)
                .text_size(typography::LABEL)
                .on_toggle(move |_| {
                    let (id, category, filter) = key.clone();

                    Message::ToggleFilter(id, category, filter)
                })
                .into()
        });

    column![header].extend(filters).spacing(6).into()
}
