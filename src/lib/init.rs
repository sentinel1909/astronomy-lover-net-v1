// src/lib/init.rs

// dependencies
use crate::routes::handlers::{
    handle_count, handle_echo, handle_fetch_nasa_data, handle_health_check, handle_metrics, handle_ping, handle_reset_metrics, handle_static_files
};
use crate::routes::router_table::RouteTable;
use crate::types::{HandlerFn, HyperMethod};

// function which builds and returns routes, their associated methods, and a handler function
// the HandlerFn type is a function pointer
pub fn build_route_table() -> RouteTable {
    let mut table = RouteTable::new();

    table.insert(
        HyperMethod::GET,
        "/_health",
        handle_health_check as HandlerFn,
    );
    table.insert(HyperMethod::GET, "/", handle_static_files as HandlerFn);
    table.insert(HyperMethod::GET, "/count", handle_count as HandlerFn);
    table.insert(HyperMethod::GET, "/echo", handle_echo as HandlerFn);
    table.insert(HyperMethod::GET, "/fetch", handle_fetch_nasa_data as HandlerFn);
    table.insert(HyperMethod::GET, "/metrics", handle_metrics as HandlerFn);
    table.insert(HyperMethod::POST, "/metrics/reset", handle_reset_metrics);

    table.insert(HyperMethod::GET, "/ping", handle_ping as HandlerFn);
    table.insert(
        HyperMethod::GET,
        "/{*path}",
        handle_static_files as HandlerFn,
    );

    table
}
