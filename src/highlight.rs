use syntect::highlighting::{Theme, ThemeSet};
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

pub struct CodeHighlighter {
    syntax_set: SyntaxSet,
    theme: Theme,
}

impl CodeHighlighter {
    pub fn new() -> Self {
        let syntax_set = SyntaxSet::load_defaults_newlines();
        let theme_set = ThemeSet::load_defaults();
        let theme = theme_set
            .themes
            .get("base16-ocean.dark")
            .expect("No theme defined")
            .clone();

        CodeHighlighter { syntax_set, theme }
    }

    pub fn highligt_code(&self, lang: &str, code: &str) -> Option<String> {
        let syntax = self.syntax_set.find_syntax_by_extension(lang)?;
        let output = highlighted_html_for_string(code, &self.syntax_set, syntax, &self.theme);

        Some(output)
    }
}
