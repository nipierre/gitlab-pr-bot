use super::pr::{Author, Pr};
use regex::Regex;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Facts {
    pub name: String,
    pub value: String
}


#[derive(Deserialize, Serialize, Debug)]
pub struct Sections {
    pub activityTitle: String,
    pub activitySubtitle: String,
    pub activityText: String,
    pub activityImage: String,
    pub facts: Vec<Facts>,
    pub markdown: bool
}


#[derive(Deserialize, Serialize, Debug)]
pub struct Choices {
    pub display: String,
    pub value: String
}


#[derive(Deserialize, Serialize, Debug)]
pub struct Inputs {
    pub attype: String,
    pub id: String,
    pub isMultiline: bool,
    pub isMultiSelect: bool,
    pub choices: Vec<Choices>,
    pub title: String
}


#[derive(Deserialize, Serialize, Debug)]
pub struct Actions {
    pub attype: String,
    pub name: String,
    pub target: String,
}


#[derive(Deserialize, Serialize, Debug)]
pub struct PotentialAction {
    pub attype: String,
    pub name: String,
    pub inputs: Vec<Inputs>,
    pub target: Vec<String>,
    pub actions: Vec<Actions>
}


#[derive(Deserialize, Serialize, Debug)]
pub struct Card {
    pub attype: String,
    pub atcontext: String,
    pub themeColor: String,
    pub summary: String,
    pub sections: Vec<Sections>,
    pub potentialAction: Vec<PotentialAction>,
}

impl Facts {
    pub fn new(pr: Pr, required_reviewer: usize) -> Vec<Facts> {
        vec![
            Facts {
                name: "Missing Reviewer(s)".into(),
                value: (required_reviewer-pr.assignees.len()).to_string()
            },
            Facts {
                name: "Assigned to".into(),
                value: get_assignees(pr.assignees)
            },
            Facts {
                name: "Source Branch".into(),
                value: pr.source_branch
            },
            Facts {
                name: "Target Branch".into(),
                value: pr.target_branch
            },
            Facts {
                name: "Status".into(),
                value: format!("*{}*",pr.merge_status)
            }
        ]
    }
}


impl Sections {
    pub fn new(pr: Pr, required_reviewer: usize) -> Vec<Sections> {
        vec![
            Sections {
                activityTitle: format!("{owner} ({username}) needs reviewer(s) for this PR : *{title}*",
                owner=pr.author.name,
                username=pr.author.username,
                title=pr.title),
                activitySubtitle: format!("On *{}*", extract_project_name(pr.references.full.clone())),
                activityText: format!("{}", pr.description),
                activityImage: format!("{}", pr.author.avatar_url),
                facts: Facts::new(pr, required_reviewer),
                markdown: true
            }
        ]
    }
}

impl PotentialAction {
    pub fn new(url: String) -> Vec<PotentialAction> {
        vec![
            PotentialAction {
                attype: "ViewAction".into(),
                name: "View PR".into(),
                inputs: vec![],
                target: vec![
                    url
                ],
                actions: vec![]
            }
        ]
    }
}


impl Card {
    pub fn new(pr: Pr, required_reviewer: usize) -> Card {
        Card {
            attype: "MessageCard".into(),
            atcontext: "http://schema.org/extensions".into(),
            themeColor: "0076D7".into(),
            summary: "This PR misses Reviewer(s) !".into(),
            potentialAction: PotentialAction::new(pr.web_url.clone()),
            sections: Sections::new(pr, required_reviewer)
        }
    }
}

pub fn extract_project_name(name: String) -> String {
    let re = Regex::new(r"(?P<name>.*)![0-9]*").unwrap();
    let project_name = re.captures(&name[..])
                         .unwrap()["name"]
                         .into();
    return project_name
}

pub fn get_assignees(assignees: Vec<Author>) -> String {
    let message: String = match assignees.is_empty() {
        false => {
            let mut s: String = "".into();
            for assignee in assignees {
                s = format!("{}{} ({})", s, assignee.name, assignee.username);
            }
            s
        },
        true => "Unassigned".into()
    };
    return message
}
