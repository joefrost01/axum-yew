use axum::{
    extract::{Path, Query},
    routing::get,
    Json, Router,
};
use chrono::{Duration, Utc};
use rand::{seq::SliceRandom, Rng};
use uuid::Uuid;

use crate::models::{DAG, DAGGraph, DAGsQuery, DAGsResponse, Edge, Task, TaskStatus};

pub fn routes() -> Router {
    Router::new()
        .route("/dags", get(get_dags))
        .route("/dags/:dag_id/graph", get(get_dag_graph))
}

async fn get_dags(Query(params): Query<DAGsQuery>) -> Json<DAGsResponse> {
    let limit = params.limit.unwrap_or(25);
    let page = params.page.unwrap_or(1);
    let offset = (page - 1) * limit;

    // Generate sample DAG data
    let mut dags = generate_sample_dags();
    let total_count = dags.len();

    // Apply search filter if provided
    if let Some(search) = &params.search {
        dags.retain(|dag| {
            dag.dag_id.to_lowercase().contains(&search.to_lowercase())
                || dag.owner.to_lowercase().contains(&search.to_lowercase())
                || dag.description
                .as_ref()
                .map_or(false, |desc| desc.to_lowercase().contains(&search.to_lowercase()))
        });
    }

    // Apply status filter if provided
    if let Some(status) = &params.status {
        match status.as_str() {
            "active" => dags.retain(|dag| !dag.paused),
            "paused" => dags.retain(|dag| dag.paused),
            "success" => dags.retain(|dag| dag.success_count > 0 && dag.failed_count == 0),
            "failed" => dags.retain(|dag| dag.failed_count > 0),
            "running" => dags.retain(|dag| dag.running_count > 0),
            _ => {}
        }
    }

    // Apply tag filter if provided
    if let Some(tags) = &params.tags {
        let tag_list: Vec<&str> = tags.split(',').collect();
        dags.retain(|dag| {
            tag_list
                .iter()
                .any(|tag| dag.tags.iter().any(|t| t.eq_ignore_ascii_case(tag)))
        });
    }

    // Apply sorting if provided
    if let Some(sort_by) = &params.sort_by {
        let asc = params
            .sort_order
            .as_ref()
            .map_or(true, |order| order == "asc");

        match sort_by.as_str() {
            "dag_id" => {
                if asc {
                    dags.sort_by(|a, b| a.dag_id.cmp(&b.dag_id));
                } else {
                    dags.sort_by(|a, b| b.dag_id.cmp(&a.dag_id));
                }
            }
            "owner" => {
                if asc {
                    dags.sort_by(|a, b| a.owner.cmp(&b.owner));
                } else {
                    dags.sort_by(|a, b| b.owner.cmp(&a.owner));
                }
            }
            "last_run" => {
                if asc {
                    dags.sort_by(|a, b| a.last_run.cmp(&b.last_run));
                } else {
                    dags.sort_by(|a, b| b.last_run.cmp(&a.last_run));
                }
            }
            "next_run" => {
                if asc {
                    dags.sort_by(|a, b| a.next_run.cmp(&b.next_run));
                } else {
                    dags.sort_by(|a, b| b.next_run.cmp(&a.next_run));
                }
            }
            _ => {
                if asc {
                    dags.sort_by(|a, b| a.dag_id.cmp(&b.dag_id));
                } else {
                    dags.sort_by(|a, b| b.dag_id.cmp(&a.dag_id));
                }
            }
        }
    }

    // Apply pagination
    let dags = dags
        .into_iter()
        .skip(offset)
        .take(limit)
        .collect::<Vec<_>>();

    Json(DAGsResponse {
        dags,
        total_count,
    })
}

async fn get_dag_graph(Path(dag_id): Path<String>) -> Json<DAGGraph> {
    let graph = generate_sample_dag_graph(dag_id);
    Json(graph)
}

fn generate_sample_dag_graph(dag_id: String) -> DAGGraph {
    let mut rng = rand::thread_rng();
    let now = Utc::now();
    
    // Define the task types (operators) that can appear in the DAG
    let operators = [
        "PythonOperator", 
        "BashOperator", 
        "PostgresOperator", 
        "MySqlOperator", 
        "HttpSensor", 
        "S3KeySensor", 
        "EmailOperator",
        "SlackOperator",
        "SparkSubmitOperator", 
        "DockerOperator"
    ];
    
    // Special case handling for our test DAGs with known node counts
    let default_min_tasks = 5;
    let default_max_tasks = 20;
    
    // Check for special test DAGs first (exact matches)
    let num_tasks = match dag_id.as_str() {
        "tiny_dag_5" => 5,
        "small_dag_20" => 20,
        "medium_dag_100" => 100,
        "large_dag_500" => 500,
        "huge_dag_1000" => 1000,
        _ => {
            // For other DAGs, try to extract the number from the name
            if let Some(cap) = dag_id.find(|c: char| c.is_digit(10)) {
                // Try to extract number from the dag_id
                let end = dag_id[cap..].find(|c: char| !c.is_digit(10)).unwrap_or(dag_id[cap..].len());
                let num_str = &dag_id[cap..cap+end];
                
                if let Ok(num) = num_str.parse::<usize>() {
                    // Limit to reasonable range between 5-1000
                    num.clamp(5, 1000)
                } else {
                    // Fallback to random between 5-20
                    rng.gen_range(default_min_tasks..default_max_tasks)
                }
            } else {
                // No number in dag_id, use random between 5-50
                rng.gen_range(default_min_tasks..50)
            }
        }
    };
    
    println!("Generating DAG with {} tasks for {}", num_tasks, dag_id);
    
    // Create the tasks
    let mut tasks = Vec::with_capacity(num_tasks);
    for i in 0..num_tasks {
        // Generate a random status with weighted distribution
        let status = match rng.gen_range(0..100) {
            0..=10 => TaskStatus::PENDING,   // 10%
            11..=60 => TaskStatus::SUCCEEDED, // 50%
            61..=70 => TaskStatus::QUEUED,    // 10%
            71..=85 => TaskStatus::RUNNING,   // 15%
            86..=95 => TaskStatus::FAILED,    // 10%
            96..=98 => TaskStatus::SKIPPED,   // 3%
            _ => TaskStatus::PAUSED,          // 2%
        };
        
        // Generate random start and end times for completed tasks
        let (start_time, end_time, duration) = match status {
            TaskStatus::SUCCEEDED | TaskStatus::FAILED => {
                let start = now - Duration::minutes(rng.gen_range(30..120));
                let duration_secs = rng.gen_range(30.0..600.0);
                let end = start + Duration::seconds(duration_secs as i64);
                (Some(start), Some(end), Some(duration_secs))
            },
            TaskStatus::RUNNING => {
                let start = now - Duration::minutes(rng.gen_range(5..30));
                (Some(start), None, None)
            },
            _ => (None, None, None)
        };
        
        let operator = operators[rng.gen_range(0..operators.len())].to_string();
        let retries = if status == TaskStatus::FAILED {
            rng.gen_range(0..3)
        } else {
            0
        };
        
        tasks.push(Task {
            id: format!("task_{}", i),
            name: format!("task_{}_{}_{}", dag_id, operator.replace("Operator", ""), i),
            status,
            duration,
            start_time,
            end_time,
            operator,
            retries,
            max_retries: 3,
        });
    }
    
    // Create edges (connections between tasks)
    // We'll create a mostly linear pipeline with some branches
    let mut edges = Vec::new();
    
    // Ensure we always have a starting task that has no dependencies
    
    // For large graphs, we need a more structured approach
    if num_tasks <= 50 {
        // Small graphs: Add edges to create a DAG structure
        for i in 1..num_tasks {
            // Most tasks connect to the previous task
            if rng.gen_bool(0.7) || i == 1 {
                edges.push(Edge {
                    source: format!("task_{}", i-1),
                    target: format!("task_{}", i),
                });
            } else {
                // Sometimes connect to a task further back to create branches
                let source_idx = rng.gen_range(0..i-1);
                edges.push(Edge {
                    source: format!("task_{}", source_idx),
                    target: format!("task_{}", i),
                });
            }
            
            // Occasionally add an extra edge to create a more complex DAG
            if i > 2 && rng.gen_bool(0.3) {
                let source_idx = rng.gen_range(0..i-1);
                // Avoid duplicate edges
                let new_edge = Edge {
                    source: format!("task_{}", source_idx),
                    target: format!("task_{}", i),
                };
                if !edges.contains(&new_edge) {
                    edges.push(new_edge);
                }
            }
        }
    } else {
        // Large graphs: Create a more structured layout
        // First, create a primary chain to ensure connectivity
        for i in 1..num_tasks {
            if i % 10 == 0 {
                // Every 10th node connects to the node 10 positions before
                edges.push(Edge {
                    source: format!("task_{}", i-10),
                    target: format!("task_{}", i),
                });
            } else {
                // Regular progression
                edges.push(Edge {
                    source: format!("task_{}", i-1),
                    target: format!("task_{}", i),
                });
            }
        }
        
        // Add some parallel paths (for every 5th node, add 2-3 parallel branches)
        for i in (5..num_tasks).step_by(5) {
            let branches = rng.gen_range(2..=4);
            
            for _ in 0..branches {
                // Connect to a random later node within a reasonable range
                let target_distance = rng.gen_range(2..=10);
                if i + target_distance < num_tasks {
                    edges.push(Edge {
                        source: format!("task_{}", i),
                        target: format!("task_{}", i + target_distance),
                    });
                }
            }
        }
        
        // Add some cross-connections for every 20th node
        for i in (20..num_tasks).step_by(20) {
            // Find a distant node to connect to
            let target_distance = rng.gen_range(15..=30);
            if i + target_distance < num_tasks {
                edges.push(Edge {
                    source: format!("task_{}", i),
                    target: format!("task_{}", i + target_distance),
                });
            }
        }
    }
    
    DAGGraph {
        dag_id,
        tasks,
        edges,
    }
}

fn generate_sample_dags() -> Vec<DAG> {
    let mut rng = rand::thread_rng();
    let now = Utc::now();

    // Sample data for generating DAGs
    let dag_name_prefixes = [
        "etl_", "data_pipeline_", "process_", "transform_", "extract_", "load_", "sync_",
        "analytics_", "report_", "backup_", "cleanup_", "validate_", "monitor_", "alert_",
    ];

    let dag_name_suffixes = [
        "daily", "hourly", "weekly", "monthly", "sales", "inventory", "users", "events",
        "transactions", "logs", "metrics", "alerts", "notifications", "products", "orders",
        "shipments", "payments", "refunds", "customers", "suppliers",
    ];
    
    // Add some DAGs with specific node counts for testing
    let special_dags = [
        ("large_dag_500", 500),
        ("huge_dag_1000", 1000),
        ("medium_dag_100", 100),
        ("small_dag_20", 20),
        ("tiny_dag_5", 5),
    ];

    let owners = [
        "admin", "airflow", "john_doe", "jane_smith", "data_engineer", "data_scientist",
        "data_analyst", "system_admin", "devops", "sre", "developer",
    ];

    let schedule_intervals = [
        "* * * * *",           // Every minute
        "*/5 * * * *",         // Every 5 minutes
        "0 * * * *",           // Hourly
        "0 */2 * * *",         // Every 2 hours
        "0 0 * * *",           // Daily at midnight
        "0 8 * * *",           // Daily at 8am
        "0 0 * * 0",           // Weekly on Sunday
        "0 0 1 * *",           // Monthly on the 1st
        "0 0 1 1 *",           // Yearly on Jan 1st
        "@hourly", "@daily", "@weekly", "@monthly", "@yearly",
    ];

    let tags = [
        "production", "development", "staging", "testing", "data_warehouse", "data_lake",
        "batch", "streaming", "etl", "ml", "ai", "reporting", "monitoring", "cleanup",
        "validation", "transformation", "extraction", "loading", "high_priority", "low_priority",
    ];

    let create_dag = |i: usize| -> DAG {
        let dag_id_prefix = dag_name_prefixes[i % dag_name_prefixes.len()];
        let dag_id_suffix = dag_name_suffixes[i % dag_name_suffixes.len()];
        let dag_id = format!(
            "{}{}_{:03}",
            dag_id_prefix,
            dag_id_suffix,
            i
        );

        let file_path = format!("/home/airflow/dags/{}.py", dag_id);

        let owner = owners[i % owners.len()].to_string();
        let paused = rng.gen_bool(0.2); // 20% chance of being paused
        let created_at = now - Duration::hours(rng.gen_range(24..720));
        let updated_at = created_at + Duration::hours(rng.gen_range(1..24));

        let last_run = if rng.gen_bool(0.9) {
            // 90% chance of having a last run
            Some(now - Duration::hours(rng.gen_range(1..48)))
        } else {
            None
        };

        let next_run = if !paused {
            Some(now + Duration::hours(rng.gen_range(1..48)))
        } else {
            None
        };

        let runs_count = rng.gen_range(0..100);
        let success_rate = rng.gen_range(0.5..0.99);
        let success_count = (runs_count as f64 * success_rate) as usize;
        let failed_count = runs_count - success_count;
        let running_count = if rng.gen_bool(0.1) {
            // 10% chance of having running tasks
            rng.gen_range(1..5)
        } else {
            0
        };

        let schedule_interval = schedule_intervals[i % schedule_intervals.len()].to_string();

        // Assign 1-3 tags randomly
        let mut dag_tags = Vec::new();
        let num_tags = rng.gen_range(1..=3);
        let mut available_tags = tags.to_vec();
        available_tags.shuffle(&mut rng);
        for tag in available_tags.iter().take(num_tags) {
            dag_tags.push(tag.to_string());
        }

        let description = if rng.gen_bool(0.8) {
            // 80% chance of having a description
            Some(format!("DAG for processing {} data", dag_name_suffixes[i % dag_name_suffixes.len()]))
        } else {
            None
        };

        DAG {
            id: Uuid::new_v4(),
            dag_id,
            description,
            file_path,
            owner,
            paused,
            last_run,
            next_run,
            runs_count,
            success_count,
            failed_count,
            running_count,
            schedule_interval,
            tags: dag_tags,
            created_at,
            updated_at,
        }
    };

    // Create regular random DAGs
    let mut dags: Vec<DAG> = (0..45).map(create_dag).collect();
    
    // Add special test DAGs with specific node counts
    for (i, (name, node_count)) in special_dags.iter().enumerate() {
        let dag_id = name.to_string();
        let file_path = format!("/home/airflow/dags/{}.py", dag_id);
        let owner = owners[i % owners.len()].to_string();
        let paused = false; // Make these always active for testing
        
        let created_at = now - Duration::hours(rng.gen_range(24..720));
        let updated_at = created_at + Duration::hours(rng.gen_range(1..24));
        let last_run = Some(now - Duration::hours(rng.gen_range(1..24)));
        let next_run = Some(now + Duration::hours(rng.gen_range(1..24)));
        
        let runs_count = rng.gen_range(5..20);
        let success_count = runs_count - 1; // Almost always successful
        let failed_count = 1; // Just one failure for testing
        let running_count = 0; // Not running now
        
        let schedule_interval = "@daily".to_string();
        
        // Add specific tags for these test DAGs
        let mut dag_tags = vec!["test".to_string(), "performance".to_string()];
        dag_tags.push(format!("nodes_{}", name.split('_').last().unwrap_or("unknown")));
        
        let description = Some(format!("Test DAG with {} nodes", name.split('_').last().unwrap_or("many")));
        
        dags.push(DAG {
            id: Uuid::new_v4(),
            dag_id,
            description,
            file_path,
            owner,
            paused,
            last_run,
            next_run,
            runs_count,
            success_count,
            failed_count,
            running_count,
            schedule_interval,
            tags: dag_tags,
            created_at,
            updated_at,
        });
    }
    
    dags
}