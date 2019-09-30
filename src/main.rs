use regex::Regex;
use std::env;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;

extern crate chrono;
extern crate regex;

struct Items {
    list: Vec<String>,
    recent_start: usize,
}

struct Task {
    status: String,
    category: String,
    title: String,
    //date: chrono::NaiveDate,
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "- [{}] [{}] {}", self.status, self.category, self.title)
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let lines: Vec<&str> = contents.lines().collect();

    /*
    let milestones = parse_section(&lines, &is_milestone_header);
    let backlog = parse_section(&lines, &is_backlog_header);
    let goals = parse_repeating_section(&lines, &is_week_header);
    let tasks = parse_repeating_section(&lines, &is_day_header);
    */
    let tasks = parse_day_sections(&lines, 2019, 9);
    for t in tasks {
        println!("{}", t);
    }

    /*
    println!("Milestones: \n{}\n", milestones.join("\n"));
    println!("Backlog: \n{}\n", backlog.join("\n"));
    println!(
        "Recent goals (of {}): \n{}\n",
        goals.list.len(),
        goals.list[goals.recent_start..].join("\n")
    );
    println!(
        "Recent tasks (of {}): \n{}\n",
        tasks.list.len(),
        tasks.list[tasks.recent_start..].join("\n")
    );
    */
    Ok(())
}

fn parse_section(lines: &Vec<&str>, is_section_header: impl Fn(&str) -> bool) -> Vec<String> {
    let mut items = vec![];
    let mut in_backlog = false;
    for line in lines {
        if is_section_header(line) {
            in_backlog = true;
        } else if in_backlog && is_item_entry(line) {
            items.push(line.to_string());
        } else {
            in_backlog = false;
        }
    }
    items
}

fn parse_day_sections(lines: &Vec<&str>, year: i32, month: i32) -> Vec<Task> {
    let mut tasks = vec![];
    let mut in_section = false;
    //let mut section_indices = vec![];

    // The format of a task is:
    // - [Status] [Category] Task description
    // - [Status] [Category] Task description 2
    // etc.
    let task_re = Regex::new(r"- \[([A-Za-z]+)\] \[([A-Za-z]+)\] (.+)$").unwrap();

    for line in lines {
        if is_day_header(line) {
            in_section = true;
        //section_indices.push(items.len());
        } else if in_section && is_item_entry(line) {
            let caps = task_re.captures(line).unwrap();
            // TODO: Don't just assume it's formatted correctly. Check for issues
            tasks.push(Task {
                status: caps[1].to_string(),
                category: caps[2].to_string(),
                title: caps[3].to_string(),
            });
        } else {
            in_section = false;
        }
    }
    tasks
}

fn parse_repeating_section(lines: &Vec<&str>, is_section_header: impl Fn(&str) -> bool) -> Items {
    let mut items = vec![];
    let mut in_section = false;
    let mut section_indices = vec![];
    for line in lines {
        if is_section_header(line) {
            in_section = true;
            section_indices.push(items.len());
        } else if in_section && is_item_entry(line) {
            items.push(line.to_string());
        } else {
            in_section = false;
        }
    }

    // It's possible for sections to exist with no items. If this happens,
    // there could be one or more indices on the section_indices stack that have indices
    // past the end of the items. For example:
    // ## Monday, 1st
    // - Task 1 (day_index gets pushed as 0 as the length before this was 0)
    // ## Tuesday, 2nd
    // <eof>
    // As Tuesday is a new day, the length of 1 would get pushed onto the day_indices stack).
    // So we need to continue popping until we have an index that reflects the last day entry with
    // task entries in it (0 in the above case).
    let mut recent_start = 0;
    while let Some(idx) = section_indices.pop() {
        recent_start = idx;
        if recent_start < items.len() {
            break;
        }
    }

    Items {
        list: items,
        recent_start,
    }
}

fn is_item_entry(line: &str) -> bool {
    line.starts_with("- ")
}

fn is_milestone_header(line: &str) -> bool {
    line.eq("# Milestones")
}

fn is_backlog_header(line: &str) -> bool {
    line.eq("# Backlog")
}

fn is_week_header(line: &str) -> bool {
    line.starts_with("# Weekly")
}

fn is_day_header(line: &str) -> bool {
    line.starts_with("## Monday")
        || line.starts_with("## Tuesday")
        || line.starts_with("## Wednesday")
        || line.starts_with("## Thursday")
        || line.starts_with("## Friday")
        || line.starts_with("## Saturday")
        || line.starts_with("## Sunday")
}
