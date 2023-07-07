use std::str::FromStr;

use chrono::{DateTime, Days, Utc};

use thiserror::Error;

type Date = DateTime<Utc>;

#[derive(Clone)]
struct Task {
    desc: String,
    priority: Priority,
    due: Option<Date>,
}

// fn print_task(task: Task) {
//     let urgency_string = if task.is_urgent() { "(!)" } else { " - " };
//     println!(
//         // print!("{*>10}", valeur) signifie :
//         // affiche la 'valeur' justifiée à droite ('>') sur
//         // au moins '10' colonnes et ajoute du padding
//         // si necessaire avec des '*'"
//         "{} {: <10} {: <40} {: <10} ",
//         urgency_string,
//         format!("{:?}", task.priority), // dirty trick pour afficher un type Debug mais pas display
//         task.desc,
//         format!("{:?}", task.due),
//     );
// }

struct TaskBuilder {
    desc: Option<String>,
    prio: Option<Priority>,
    due: Option<DateTime<Utc>>,
}

impl TaskBuilder {
    fn new() -> Self {
        Self {
            desc: None,
            prio: None,
            due: None,
        }
    }

    fn desc(mut self, desc: String) -> Self {
        self.desc = Some(desc);
        self
    }

    fn priority(mut self, prio: &str) -> Self {
        self.prio = Some(Priority::from_str(prio).unwrap_or_else(|e| {
            // log "non valid value, using default priority"
            Priority::default()
        }));
        self
    }

    fn deadline(mut self, due: &str) -> Self {
        // FIXME
        let dt = DateTime::parse_from_str(due, "%Y").expect("tryagain");
        self.due = Some(dt.naive_utc().and_utc());
        self
    }

    fn build(self) -> Task {
        Task {
            desc: self.desc.unwrap(),
            priority: self.prio.unwrap_or_default(),
            due: self.due,
        }
    }
}

impl Task {
    #[cfg(test)]
    fn new(desc: String, priority: Priority, due: Option<Date>) -> Self {
        Task {
            desc,
            priority,
            due,
        }
    }

    fn restitute_task(&self) -> &Self {
        self
    }

    fn is_urgent(&self) -> bool {
        let urgent_priority = match self.priority {
            Priority::Low => false,
            Priority::Normal => false,
            Priority::High => true,
        };
        let urgent_deadline = match self.due {
            None => false,
            Some(due) => Utc::now() + Days::new(1) >= due,
        };

        urgent_priority || urgent_deadline
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone, Copy, Default)]
enum Priority {
    Low,
    #[default]
    Normal,
    High,
}

impl FromStr for Priority {
    type Err = PriorityError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_lowercase().as_str() {
            "low" => Ok(Priority::Low),
            "normal" => Ok(Priority::Normal),
            "high" => Ok(Priority::High),
            input @ _ => Err(PriorityError::NonParsablePriority(input.to_string())),
        }
    }
}

#[derive(Error, Debug)]
pub enum PriorityError {
    #[error("the value {0} cannot be parsed as a priority")]
    NonParsablePriority(String),
}

fn main() {
    use structopt::StructOpt;

    #[derive(Debug, StructOpt)]
    #[structopt(name = "tasker", about = "Manage your tasks")]
    struct Args {
        #[structopt(subcommand)]
        cmd: Cmd,
    }

    #[derive(Debug, StructOpt)]
    enum Cmd {
        Add(Desc),
    }

    #[derive(Debug, StructOpt)]
    struct Desc {
        desc: String,
    }

    let Args {
        cmd: Cmd::Add(Desc { desc }),
    } = Args::from_args();

    let task = TaskBuilder::new().desc(desc).build();

    task.is_urgent();
    let _ = task.restitute_task();

    println!(
        "task description: {} {:?} {:?}",
        task.desc, task.priority, task.due
    );
}

#[cfg(test)]
mod tests {

    use super::*;
    use chrono::{Days, TimeZone, Utc};

    #[test]
    fn build_task_with_label_and_priority_high_priority() {
        assert_eq!(
            Task::new(String::from("description"), Priority::High, None).priority,
            Priority::High
        );
    }

    #[test]
    fn task_deadline_is_not_none_when_set_with_date() {
        assert_ne!(
            Task::new(String::from(""), Priority::High, Some(chrono::Utc::now())).due,
            None
        );
    }

    #[test]
    fn task_is_urgent_when_priority_is_high_whatever_the_deadline() {
        assert_eq!(Task::new("".into(), Priority::High, None).is_urgent(), true);
        assert_eq!(
            Task::new("".into(), Priority::High, Some(Utc::now())).is_urgent(),
            true
        );
        assert_eq!(
            Task::new(
                "".into(),
                Priority::High,
                Some(Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap())
            )
            .is_urgent(),
            true
        );
    }

    #[test]
    fn task_is_urgent_when_date_is_next24h_whatever_the_priority() {
        assert_eq!(
            Task::new("".into(), Priority::Low, Some(Utc::now())).is_urgent(),
            true
        );
        assert_eq!(
            Task::new("".into(), Priority::High, Some(Utc::now() + Days::new(1))).is_urgent(),
            true
        );
    }

    #[test]
    fn task_is_not_urgent_when_date_is_not_next24h_and_priority_is_not_high() {
        assert_eq!(
            Task::new("".into(), Priority::Normal, None).is_urgent(),
            false
        );
        assert_eq!(
            Task::new("".into(), Priority::Low, Some(Utc::now() + Days::new(3))).is_urgent(),
            false
        );
    }
}
