
pub trait MarkdownHandler {
    fn parse_markdown(&self, input: &str) -> String;
    fn to_markdown(&self, input: &str) -> String;
    fn render_markdown(&self, input: &str) -> String;
    fn make_bold(&self, input: &str) -> String {
        format!("**{}**", input)
    }
    fn make_italic(&self, input: &str) -> String {
        format!("*{}*", input)
    }

    fn make_h1(&self, input: &str) -> String {
        format!("# {}", input)
    }

    fn make_h2(&self, input: &str) -> String {
        format!("## {}", input)
    }

    fn insert_link(&self, text: &str, url: &str) -> String {
        format!("[{}]({})", text, url)
    }
    fn make_quote(&self, input: &str) -> String {
        format!("> {}", input)
    }
    fn insert_image(&self, alt: &str, url: &str) -> String {
        format!("![{}]({})", alt, url)
    }
}