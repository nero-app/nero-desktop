use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Language {
    #[default]
    English,
}

impl Language {
    pub const ALL: &'static [Language] = &[Language::English];

    pub fn code(self) -> &'static str {
        match self {
            Language::English => "en",
        }
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Language::English => "English",
        }
        .fmt(f)
    }
}

pub fn set_language(language: Language) {
    rust_i18n::set_locale(language.code());
}
