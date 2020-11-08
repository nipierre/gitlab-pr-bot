use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Author {
    pub id: u32,
    pub name: String,
    pub username: String,
    pub state: String,
    pub avatar_url: String,
    pub web_url: String
}


#[derive(Deserialize, Serialize, Debug)]
pub struct References {
    pub short: String,
    pub relative: String,
    pub full: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TimeStats {
    pub time_estimate: f32,
    pub total_time_spent: f32,
    pub human_time_estimate: Option<f32>,
    pub human_total_time_spent: Option<f32>
}


#[derive(Deserialize, Serialize, Debug)]
pub struct TaskCompletionStatus {
    pub count: usize,
    pub completed_count: usize
}


#[derive(Deserialize, Serialize, Debug)]
pub struct Pr {
    pub id: u32,
    pub iid: u32,
    pub project_id: u32,
    pub title: String,
    pub description: String,
    pub state: String,
    pub created_at: String,
    pub updated_at: String,
    pub merged_by: Option<String>,
    pub merged_at: Option<String>,
    pub closed_by: Option<String>,
    pub closed_at: Option<String>,
    pub target_branch: String,
    pub source_branch: String,
    pub user_notes_count: u32,
    pub upvotes: u32,
    pub downvotes: u32,
    pub author: Author,
    pub assignees: Vec<Author>,
    pub assignee: Option<Author>,
    pub source_project_id: u32,
    pub target_project_id: u32,
    pub labels: Vec<String>,
    pub work_in_progress: Option<bool>,
    pub milestone: Option<String>,
    pub merge_when_pipeline_succeeds: Option<bool>,
    pub merge_status: String,
    pub sha: String,
    pub merge_commit_sha: Option<String>,
    pub squash_commit_sha: Option<String>,
    pub discussion_locked: Option<String>,
    pub should_remove_source_branch: Option<String>,
    pub force_remove_source_branch: Option<bool>,
    pub reference: String,
    pub references: References,
    pub web_url: String,
    pub time_stats: TimeStats,
    pub squash: Option<bool>,
    pub task_completion_status: Option<TaskCompletionStatus>,
    pub has_conflicts: Option<bool>,
    pub blocking_discussions_resolved: Option<bool>,
    pub approvals_before_merge: Option<String>
}
