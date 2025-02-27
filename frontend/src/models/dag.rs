use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DAG {
    pub id: Uuid,
    pub dag_id: String,
    pub description: Option<String>,
    pub file_path: String,
    pub owner: String,
    pub paused: bool,
    pub last_run: Option<DateTime<Utc>>,
    pub next_run: Option<DateTime<Utc>>,
    pub runs_count: usize,
    pub success_count: usize,
    pub failed_count: usize,
    pub running_count: usize,
    pub schedule_interval: String,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DAGsResponse {
    pub dags: Vec<DAG>,
    pub total_count: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct DAGsQuery {
    pub page: Option<usize>,
    pub limit: Option<usize>,
    pub search: Option<String>,
    pub status: Option<String>,
    pub tags: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskStatus {
    PENDING,
    SUCCEEDED,
    QUEUED,
    RUNNING,
    FAILED,
    SKIPPED,
    PAUSED,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub name: String,
    pub status: TaskStatus,
    pub duration: Option<f64>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub operator: String,
    pub retries: usize,
    pub max_retries: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge {
    pub source: String,
    pub target: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DAGGraph {
    pub dag_id: String,
    pub tasks: Vec<Task>,
    pub edges: Vec<Edge>,
}

impl DAG {
    pub fn status(&self) -> &'static str {
        if self.paused {
            "paused"
        } else if self.running_count > 0 {
            "running"
        } else if self.failed_count > 0 {
            "failed"
        } else if self.success_count > 0 {
            "success"
        } else {
            "none"
        }
    }
}

impl TaskStatus {
    pub fn color(&self) -> &'static str {
        match self {
            TaskStatus::SUCCEEDED => "#4caf50",  // Green
            TaskStatus::FAILED => "#f44336",     // Red
            TaskStatus::RUNNING => "#2196f3",    // Blue
            TaskStatus::PENDING => "#9e9e9e",    // Gray
            TaskStatus::QUEUED => "#ff9800",     // Orange
            TaskStatus::SKIPPED => "#673ab7",    // Purple
            TaskStatus::PAUSED => "#795548",     // Brown
        }
    }
    
    pub fn label(&self) -> &'static str {
        match self {
            TaskStatus::SUCCEEDED => "Succeeded",
            TaskStatus::FAILED => "Failed",
            TaskStatus::RUNNING => "Running",
            TaskStatus::PENDING => "Pending",
            TaskStatus::QUEUED => "Queued",
            TaskStatus::SKIPPED => "Skipped",
            TaskStatus::PAUSED => "Paused",
        }
    }
}