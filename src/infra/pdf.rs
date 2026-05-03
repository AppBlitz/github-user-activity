use genpdf::{
    Document, Element, SimplePageDecorator,
    elements::{self, FrameCellDecorator, LinearLayout, Paragraph, TableLayout},
    fonts, style,
};
use std::result::Result;

use crate::{
    infra::functions::{count_push, run_array},
    model::{format_output::FormatEvents, github_events::Events},
};
fn decorator_pdf(file_decorator: &mut Document) {
    let mut decorator = SimplePageDecorator::new();
    decorator.set_margins(10);
    decorator.set_header(|_page_number| {
        let mut layout = LinearLayout::vertical();
        let bre = elements::Break::new(2);
        layout.push(
            elements::Paragraph::new("Github user activity")
                .styled(style::Style::new().bold().with_font_size(18)),
        );
        layout.push(bre);
        layout
    });
    file_decorator.set_page_decorator(decorator);
}

fn create_document() -> Document {
    let font_family = fonts::from_files("./fonts/", "LiberationSans", None).unwrap();
    let mut doc: Document = Document::new(font_family);
    doc.set_title("github activity");
    doc
}
fn create_table(names_repository: &Vec<String>, events: &Vec<Events>) -> TableLayout {
    let mut table = TableLayout::new(vec![1, 1]);
    table.set_cell_decorator(FrameCellDecorator::new(true, true, false));
    let format_events: Vec<FormatEvents> = run_array(names_repository, events);
    for element in format_events {
        let mut row = table.row();
        row.push_element(create_paragraph(element.name_repository));
        row.push_element(create_paragraph(element.amount_push.to_string()));
        row.push().expect("Invalid elements");
    }
    table
}

fn create_paragraph(name: String) -> Paragraph {
    Paragraph::default().styled_string(name, style::Color::Rgb(0, 0, 0))
}
pub fn generate_pdf(
    names_repository: &Vec<String>,
    events: &Vec<Events>,
) -> Result<&'static str, Box<dyn std::error::Error>> {
    let mut document = create_document();
    let table = create_table(names_repository, events);
    document.push(table);
    decorator_pdf(&mut document);
    document.render_to_file("github_activity.pdf")?;
    Ok("Pdf created succeed")
}

pub fn response_pdf(name_repository: &Vec<String>, events: &Vec<Events>) {
    match generate_pdf(name_repository, events) {
        Ok(value) => println!("{value:?}"),
        Err(er) => eprintln!("{er:?}"),
    }
}
pub fn list_repository(events: &Vec<Events>) -> std::collections::HashMap<&String, i64> {
    let mut hasmap_user: std::collections::HashMap<&String, i64> = std::collections::HashMap::new();
    for event in events {
        hasmap_user.insert(&event.repo.name, count_push(&event.repo.name, events));
    }
    hasmap_user
}
