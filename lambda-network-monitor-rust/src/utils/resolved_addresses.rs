use std::collections::HashMap;
use std::sync::Mutex;
use once_cell::sync::Lazy;

static MAP: Lazy<Mutex<HashMap<String, String>>> = Lazy::new(|| Mutex::new(HashMap::new()));

pub fn add(node: &str, ip: &str) {
    log::debug!("add node={node} ip={ip}");
    MAP.lock().unwrap().insert(ip.to_string(), node.to_string());
}

pub fn get_node_by_ip(ip: &str) -> String {
    match MAP.lock().unwrap().get(ip) {
        Some(node) => node.to_string(),
        None => "n/a".to_string(),
    }
}