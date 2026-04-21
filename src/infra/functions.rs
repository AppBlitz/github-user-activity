use genpdf::{
    Document,
    elements::{Paragraph, TableLayout},
    fonts, style,
};

use crate::model::{format_output::FormatEvents, github_events::Events};

pub fn count_push(name_repository: &String, events: &Vec<Events>) -> i64 {
    let mut count_pus: i64 = 0;
    let name_repo: String = format!("{}/{}", events[0].actor.login, name_repository);
    for elements in events {
        if elements.repo.name == name_repo {
            count_pus += 1;
        }
    }
    count_pus
}
pub fn run_array(name_repository: Vec<String>, events: &Vec<Events>) -> Vec<FormatEvents> {
    let mut events_format: Vec<FormatEvents> = Vec::new();
    for names in name_repository {
        let structure_repository = FormatEvents {
            amount_push: count_push(&names, events),
            name: names,
        };
        events_format.push(structure_repository);
    }
    events_format
}
pub fn generate_pdf(names_repository: Vec<String>, events: &Vec<Events>) {
    let mut document = create_document();
    let table = create_table(names_repository, events);
    document.push(table);
    document
        .render_to_file("github_activity.pdf")
        .expect("Failed write content pdf ")
}

fn create_document() -> Document {
    let font_family = fonts::from_files("./fonts/", "LiberationSans", None).unwrap();
    let mut doc: Document = Document::new(font_family);
    doc.set_title("github activity");
    doc
}
fn create_table(names_repository: Vec<String>, events: &Vec<Events>) -> TableLayout {
    let mut table = TableLayout::new(vec![1, 1]);
    let format_events: Vec<FormatEvents> = run_array(names_repository, events);
    for element in format_events {
        let mut row = table.row();
        row.push_element(create_paragraph(element.name));
        row.push_element(create_paragraph(element.amount_push.to_string()));
        row.push().expect("Invalid elements");
    }
    table
}

fn create_paragraph(name: String) -> Paragraph {
    Paragraph::default().styled_string(name, style::Color::Rgb(0, 0, 0))
}
