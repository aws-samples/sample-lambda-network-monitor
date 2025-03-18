use std::collections::HashMap;
use once_cell::sync::Lazy;
use std::ffi::c_int;
use std::sync::Mutex;

/// Map file descriptor to address string for reporting
static MAP: Lazy<Mutex<HashMap<c_int, String>>> = Lazy::new(|| Mutex::new(HashMap::new()));

pub fn add(fd: c_int){
    log::debug!("> add fd={fd}");
    if !contains(fd) {
        MAP.lock().unwrap().insert(fd, String::from(""));
        log::debug!("added fd={fd}");
    }
}

pub fn contains(fd: c_int) -> bool {
    MAP.lock().unwrap().contains_key(&fd)
}

pub fn set_socket_addr(fd: c_int, addr: &str){
    log::debug!("> set_socket_addr fd={fd} addr={addr}");
    if contains(fd){
        remove(fd);
        MAP.lock().unwrap().insert(fd, addr.to_string());
        log::debug!("set_socket_addr fd={fd} addr={addr}, done");
    } else {
        log::debug!("set_socket_addr fd={fd} addr={addr}, not found");
    }
}

// pub fn get_socket_addr(fd: c_int) -> String {
//     let map = MAP.lock().unwrap();
//     map.get(&fd).unwrap().clone()
// }

pub fn remove(fd: c_int) {
    if contains(fd){
        MAP.lock().unwrap().remove(&fd);
        log::debug!("remove fd={}, done", fd);
    } else {
        log::debug!("remove fd={}, not found", fd);

    }
}
