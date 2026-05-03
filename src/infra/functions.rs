use crate::model::{format_output::FormatEvents, github_events::Events};

pub fn count_push(name_repository: &String, events: &Vec<Events>) -> i64 {
    let mut count_pus: i64 = 0;
    // let name_repo: String = format!("{}/{}", events[0].actor.login, name_repository);
    for elements in events {
        if elements.repo.name == name_repository.to_string() {
            count_pus += 1;
        }
    }
    count_pus
}
pub fn run_array(name_repository: &Vec<String>, events: &Vec<Events>) -> Vec<FormatEvents> {
    let mut events_format: Vec<FormatEvents> = Vec::new();
    for names in name_repository {
        let structure_repository = FormatEvents {
            amount_push: count_push(names, events),
            name_repository: names.to_string(),
        };
        events_format.push(structure_repository);
    }
    events_format
}
