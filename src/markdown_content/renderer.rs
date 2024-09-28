use markdown::to_html;

pub fn render_markdown(content: String) -> String {
    to_html(&content)
}
