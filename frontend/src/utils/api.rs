use crate::models::dag::{DAGsQuery, DAGsResponse, DAGGraph};
use gloo::net::http::Request;
use serde::Serialize;

const API_BASE_URL: &str = "/api";

pub async fn fetch_dags(query: &DAGsQuery) -> Result<DAGsResponse, String> {
    let url = format!("{}/dags", API_BASE_URL);

    // Build the query params
    let mut params = vec![];

    if let Some(page) = query.page {
        params.push(format!("page={}", page));
    }

    if let Some(limit) = query.limit {
        params.push(format!("limit={}", limit));
    }

    if let Some(search) = &query.search {
        params.push(format!("search={}", search));
    }

    if let Some(status) = &query.status {
        params.push(format!("status={}", status));
    }

    if let Some(tags) = &query.tags {
        params.push(format!("tags={}", tags));
    }

    if let Some(sort_by) = &query.sort_by {
        params.push(format!("sort_by={}", sort_by));
    }

    if let Some(sort_order) = &query.sort_order {
        params.push(format!("sort_order={}", sort_order));
    }

    let url = if params.is_empty() {
        url
    } else {
        format!("{}?{}", url, params.join("&"))
    };

    let response = Request::get(&url)
        .send()
        .await
        .map_err(|e| format!("Failed to fetch DAGs: {:?}", e))?;

    if !response.ok() {
        return Err(format!("API error: {}", response.status()));
    }

    response
        .json::<DAGsResponse>()
        .await
        .map_err(|e| format!("Failed to parse response: {:?}", e))
}

pub async fn toggle_dag_paused(dag_id: &str, paused: bool) -> Result<(), String> {
    let _url = format!("{}/dags/{}/paused", API_BASE_URL, dag_id);

    #[derive(Serialize)]
    struct PausedPayload {
        is_paused: bool,
    }

    let _payload = PausedPayload { is_paused: paused };

    // In a real app, we'd send a POST request here
    // For this demo, we'll just return Ok since our backend doesn't actually support this endpoint
    Ok(())
}

pub fn format_datetime(dt: Option<chrono::DateTime<chrono::Utc>>) -> String {
    match dt {
        Some(dt) => {
            // Format as "YYYY-MM-DD HH:MM:SS"
            dt.format("%Y-%m-%d %H:%M:%S").to_string()
        }
        None => "N/A".to_string(),
    }
}

pub async fn fetch_dag_graph(dag_id: &str) -> Result<DAGGraph, String> {
    let url = format!("{}/dags/{}/graph", API_BASE_URL, dag_id);
    
    let response = Request::get(&url)
        .send()
        .await
        .map_err(|e| format!("Failed to fetch DAG graph: {:?}", e))?;
        
    if !response.ok() {
        return Err(format!("API error: {}", response.status()));
    }
    
    response
        .json::<DAGGraph>()
        .await
        .map_err(|e| format!("Failed to parse DAG graph response: {:?}", e))
}

pub async fn update_task_status(dag_id: &str, task_id: &str, status: &str) -> Result<(), String> {
    let _url = format!("{}/dags/{}/tasks/{}/status", API_BASE_URL, dag_id, task_id);
    
    #[derive(Serialize)]
    struct StatusPayload {
        status: String,
    }
    
    let _payload = StatusPayload { status: status.to_string() };
    
    // In a real app, we'd send a PATCH request here
    // For this demo, we'll just return Ok since our backend doesn't actually support this endpoint
    Ok(())
}