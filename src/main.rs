use clap::{App, Arg};
use dotenv::dotenv;
use log::{debug, info};
use reqwest::Error;
use reqwest::header::CONTENT_TYPE;
use gitlab::GitLab;
use pr::Pr;
use repo::Repo;
use std::env;
use teams::Card;

mod gitlab;
mod pr;
mod repo;
mod teams;



#[tokio::main]
async fn main() -> Result<(), Error> {
    let matches = App::new("gitlab-pr-bot")
    .version("0.0.1")
    .about("Bot alerting about pending PRs")
    .arg(
      Arg::with_name("date")
        .short("d")
        .long("date")
        .help("Get PR after this date")
        .default_value("1970-01-01T00:00:00.000Z")
        .takes_value(true)
    )
    .arg(
      Arg::with_name("dotenv")
        .long("dotenv")
        .help("Get GitLab infos from .env")
    )
    .arg(
      Arg::with_name("number-reviewers")
        .long("number-reviewers")
        .value_name("NUMBER-REVIEWER")
        .help("Sets the required number of reviewers")
        .default_value("1")
        .takes_value(true),
    )
    .arg(
      Arg::with_name("token")
        .long("token")
        .value_name("TOKEN")
        .help("Sets the GitLab token")
        .takes_value(true),
    )
    .arg(
      Arg::with_name("url")
        .long("url")
        .value_name("URL")
        .help("Sets the GitLab url (default: https://gitlab.com)")
        .default_value("https://gitlab.com")
        .takes_value(true),
    )
    .arg(
      Arg::with_name("v")
        .short("v")
        .multiple(true)
        .help("Sets the level of verbosity"),
    )
    // TODO: Find crate for Vault
    .arg(
      Arg::with_name("vault")
        .long("vault")
        .help("Get GitLab infos with Vault"),
    )
    .get_matches();

    match matches.occurrences_of("v") {
        0 => simple_logger::init_with_level(log::Level::Error).unwrap(),
        1 => simple_logger::init_with_level(log::Level::Info).unwrap(),
        _ => simple_logger::init_with_level(log::Level::Debug).unwrap(),
    }

    let gitlabs: Vec<GitLab> = if matches.occurrences_of("vault") == 1 {
        let vault_token = env::var("VAULT_TOKEN").unwrap();
        let vec_gitlab: Vec<GitLab> = vec![];
        // TODO Need good Vault API crate
        vec_gitlab
    }
    else if matches.occurrences_of("dotenv") == 1 {
        dotenv().ok();
        let urls: Vec<String> = env::var("GITLAB_URL").unwrap().split(':')
                                                     .map(String::from)
                                                     .collect();
        let mut tokens: Vec<String> = env::var("GITLAB_TOKEN").unwrap().split(':')
                                                             .map(String::from)
                                                             .collect();
        let mut vec_gitlab: Vec<GitLab> = vec![];
        for (url, token) in urls.iter().zip(tokens.iter_mut()) {

            vec_gitlab.push(GitLab {
                    url: url.clone(),
                    token: token.clone()
                });
        }
        vec_gitlab
    }
    else {
        let url = matches.value_of("url")
            .unwrap()
            .parse::<String>()
            .expect("Unable to parse url");
        let token = matches.value_of("token")
            .unwrap()
            .parse::<String>()
            .expect("Unable to parse token");
        vec![
            GitLab {
                url,
                token
            }
        ]
    };

    let required_reviewer = matches.value_of("number-reviewers")
                                   .unwrap()
                                   .parse::<usize>()
                                   .expect("Unable to parse number-reviewers");

    info!("Number of GitLab projects : {}", gitlabs.len());

    let client = reqwest::Client::new();

    let date = matches.value_of("date")
                      .unwrap()
                      .parse::<String>()
                      .expect("Unable to parse url");

    for gitlab in gitlabs {

        info!("Projects @ {} processing..", gitlab.url);

        let repo_url = format!("https://{}/api/v4/projects?", gitlab.url);

        let repo_response = client
            .get(&repo_url)
            .query(&[("per_page","100"),
                     ("membership","true"),
                     ("simple","true"),
                     ("last_activity_after",&date)])
            .bearer_auth(gitlab.token.clone())
            .send()
            .await?;

        debug!("{:?}", repo_response);

        let repos: Vec<Repo> = repo_response.json().await?;
        debug!("{:?}", repos);

        for repo in repos {

            let repo_url_pr = format!("https://{gitlab}/api/v4/projects/{repo_id}/merge_requests?",
                                gitlab=gitlab.url,
                                repo_id=repo.id);

            let pr_response = client
                                .get(&repo_url_pr)
                                .query(&[("state","opened")])
                                .bearer_auth(gitlab.token.clone())
                                .send()
                                .await?;
            let prs: Vec<Pr> = pr_response.json().await?;

            for pr in prs {

                let card = Card::new(pr, required_reviewer);

                let json = serde_json::to_string(&card).unwrap().replace("attype","@type").replace("atcontext","@context");

                debug!("{:?}", json);

                let teams_response = client
                                        .post(&env::var("TEAMS_WEBHOOK").unwrap())
                                        .header(CONTENT_TYPE, "application/json")
                                        .body(json)
                                        .send()
                                        .await?;

                debug!("{:?}", teams_response);
            }
        }

        info!("Projects @ {} done.", gitlab.url);

    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use pr::{Author,Pr,References,TimeStats};
    use teams::{Card,extract_project_name,
                get_assignees};

    #[test]
    fn test_extract_project_name() {
        let mock_project: String = "myproject/myrepo!1234".into();

        let result = extract_project_name(mock_project);

        assert_eq!(result,"myproject/myrepo");
    }

    #[test]
    fn test_get_assignees() {
        let mock_assignees: Vec<Author> = vec![
            Author {
                id: 12345,
                name: "Toto Tata".into(),
                username: "toto123".into(),
                state: "Active".into(),
                avatar_url: "greatURL".into(),
                web_url: "greatURL".into()
            }
        ];

        let result = get_assignees(mock_assignees);

        assert_eq!(result,"Toto Tata (toto123)");
    }

    #[test]
    fn test_card_creation() {
        let author: Author = Author {
            id: 12345,
            name: "Toto Tata".into(),
            username: "toto123".into(),
            state: "Active".into(),
            avatar_url: "greatURL".into(),
            web_url: "greatURL".into()
        };

        let assignee1: Author = Author {
            id: 54321,
            name: "Titi Tata".into(),
            username: "titi123".into(),
            state: "Active".into(),
            avatar_url: "greatURL".into(),
            web_url: "greatURL".into()
        };

        let assignee2: Author = Author {
            id: 54321,
            name: "Titi Tata".into(),
            username: "titi123".into(),
            state: "Active".into(),
            avatar_url: "greatURL".into(),
            web_url: "greatURL".into()
        };

        let pr: Pr = Pr {
            id: 1234,
            iid: 1234,
            project_id: 1234,
            title: "Title".into(),
            description: "This is a description".into(),
            state: "State".into(),
            created_at: "Created".into(),
            updated_at: "Updated".into(),
            merged_by: None,
            merged_at: None,
            closed_by: None,
            closed_at: None,
            target_branch: "Target".into(),
            source_branch: "Source".into(),
            user_notes_count: 0,
            upvotes: 1,
            downvotes: 2,
            author,
            assignees: vec![assignee1],
            assignee: Some(assignee2),
            source_project_id: 0,
            target_project_id: 1,
            labels: vec![],
            work_in_progress: None,
            milestone: None,
            merge_when_pipeline_succeeds: None,
            merge_status: "not_merged".into(),
            sha: "SHASHA".into(),
            merge_commit_sha: None,
            squash_commit_sha: None,
            discussion_locked: None,
            should_remove_source_branch: None,
            force_remove_source_branch: None,
            reference: "ref".into(),
            references: References {
                short: "short".into(),
                relative: "relative".into(),
                full: "myproject/myrepo!1234".into()
            },
            web_url: "greatURL".into(),
            time_stats: TimeStats {
                time_estimate: 0.0,
                total_time_spent: 0.0,
                human_time_estimate: None,
                human_total_time_spent: None
            },
            squash: None,
            task_completion_status: None,
            has_conflicts: None,
            blocking_discussions_resolved: None,
            approvals_before_merge: None
        };

        let card: Card = Card::new(pr, 1);

        assert_eq!(card.potentialAction[0].target[0],"greatURL");
        assert_eq!(card.sections[0].activityTitle,format!("Toto Tata (toto123) needs reviewer(s) for this PR : *Title*"));
        assert_eq!(card.sections[0].activitySubtitle,"On *myproject/myrepo*");
        assert_eq!(card.sections[0].activityText,"This is a description");
        assert_eq!(card.sections[0].activityImage,"greatURL");
        assert_eq!(card.sections[0].facts[0].value,"1");
        assert_eq!(card.sections[0].facts[1].value,"Titi Tata (titi123)");
        assert_eq!(card.sections[0].facts[2].value,"Source");
        assert_eq!(card.sections[0].facts[3].value,"Target");
        assert_eq!(card.sections[0].facts[4].value,"*not_merged*");
    }
}
