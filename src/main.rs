use chrono::NaiveDate;
use regex::Regex;
use std::env;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;

extern crate chrono;
extern crate regex;

struct Journal {
    milestones: Vec<String>,
    backlog: Vec<Task>,
    goals: Vec<Task>,
    tasks: Vec<Task>,
}

struct Task {
    status: String,
    category: String,
    title: String,
    #[allow(dead_code)]
    date: chrono::NaiveDate,
}

enum FileSection {
    Milestones(Vec<String>),
    Backlog(Vec<Task>),
    Week(Vec<Task>),
    Day(Vec<Task>),
    Unrecognized,
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
    let journal = parse_month(&lines, 2019, 9);

    println!("Found {} milestones", journal.milestones.len());
    println!("Found {} tasks in backlog", journal.backlog.len());
    println!("Found {} goals in week", journal.goals.len());
    println!("Found {} tasks in day", journal.tasks.len());

    Ok(())
}

fn parse_month(lines: &[&str], year: i32, month: u32) -> Journal {
    let mut journal = Journal {
        milestones: Vec::new(),
        backlog: Vec::new(),
        goals: Vec::new(),
        tasks: Vec::new(),
    };

    let sections = find_sections(&lines);
    for (begin, end) in sections {
        let section_lines = &lines[begin..end];
        match parse_section(section_lines, year, month) {
            FileSection::Milestones(mut entries) => {
                journal.milestones.append(&mut entries);
            }
            FileSection::Backlog(mut tasks) => {
                journal.backlog.append(&mut tasks);
            }
            FileSection::Week(mut goals) => {
                journal.goals.append(&mut goals);
            }
            FileSection::Day(mut tasks) => {
                journal.tasks.append(&mut tasks);
            }
            _ => {
                // Unrecognized, do nothing
            }
        }
    }
    journal
}

/// Captures pairs of indices that determine the sections that are within the document.
/// A "section" is a section that begins with a header ("#") and continues until the next
/// header.
fn find_sections(lines: &[&str]) -> Vec<(usize, usize)> {
    let mut sections = vec![];
    let mut prev = None;

    for i in 0..lines.len() {
        let line = lines[i];
        if line.starts_with('#') {
            match prev {
                Some(idx) => {
                    sections.push((idx, i));
                }
                None => {
                    // This is the start of the first section, so we don't have something to push
                }
            }
            prev = Some(i);
        }
    }

    assert!(prev.is_some()); // We shouldn't be trying to parse a file w/ just a single header
    sections.push((prev.unwrap(), lines.len()));
    sections
}

fn parse_section(lines: &[&str], year: i32, month: u32) -> FileSection {
    let header = lines[0];
    if is_milestone_header(header) {
        parse_milestones(lines)
    } else if is_backlog_header(header) {
        parse_backlog(lines, year, month)
    } else if is_week_header(header) {
        parse_week(lines, year, month)
    } else if is_day_header(header) {
        parse_day(lines, year, month)
    } else {
        FileSection::Unrecognized
    }
}

fn parse_milestones(lines: &[&str]) -> FileSection {
    let mut milestones = vec![];
    for line in lines {
        if is_item_entry(line) {
            milestones.push(line.to_string());
        }
    }
    FileSection::Milestones(milestones)
}

fn parse_backlog(lines: &[&str], year: i32, month: u32) -> FileSection {
    let mut tasks = vec![];
    for line in lines {
        if is_item_entry(line) {
            tasks.push(parse_task(line, year, month));
        }
    }
    FileSection::Backlog(tasks)
}

fn parse_week(lines: &[&str], year: i32, month: u32) -> FileSection {
    let mut tasks = vec![];
    for line in lines {
        if is_item_entry(line) {
            tasks.push(parse_task(line, year, month));
        }
    }
    FileSection::Week(tasks)
}

fn parse_day(lines: &[&str], year: i32, month: u32) -> FileSection {
    // The day header is in format "## Day, Number", so get final two digits of string
    let header = lines[0];
    let day = header[header.len() - 2..].trim().parse::<u32>().unwrap();

    let mut tasks = vec![];
    for line in lines {
        if is_item_entry(line) {
            tasks.push(parse_task_with_day(line, year, month, day));
        }
    }
    FileSection::Day(tasks)
}

fn parse_task(line: &str, year: i32, month: u32) -> Task {
    parse_task_with_day(line, year, month, 1)
}

fn parse_task_with_day(line: &str, year: i32, month: u32, day: u32) -> Task {
    // The format of a task is:
    // - [Status] [Category] Task description
    // - [Status] [Category] Task description 2
    // etc.
    let task_re = Regex::new(r"- \[([A-Za-z]+)\] \[([A-Za-z]+)\] (.+)$").unwrap();
    let caps = task_re.captures(line).unwrap();
    Task {
        status: caps[1].to_string(),
        category: caps[2].to_string(),
        title: caps[3].to_string(),
        date: NaiveDate::from_ymd(year, month, day),
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
