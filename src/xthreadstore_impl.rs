// This is the entry point for any xthread access. This is a store for all data
// members that we expect to be available across threads, referenced by means
// of an Arc.
// Good practice here would be for individual accessors to access the value
// during initialization of task, copy the value and update only at the end of
// the task, discouraging frequent accesses of the Arc pointer. This means that
// it is generally good practice to ensure that an entity from XThreadStore is
// copied for any mutations, allowing the thread to update the shared entry
// once and for all.
// TODO(@Skeletrox) What if we make each distributed component use an extension
// of the struct in order to store only the variables that it cares about? For
// example, the storage component must not contain a XThreadStore that contains
// variables pertaining to query processing or whatever.
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

pub struct XThreadStore {
    // Example value.
    example: Arc<Mutex<i32>>
}

impl XThreadStore {
    // Default constructor. All values get initialized here.
    fn new() -> XThreadStore {
        XThreadStore {
            example: Arc::new(Mutex::new(0))
        }
    }

    pub fn new_arc() -> Result<Arc<XThreadStore>, String> {
        if X_THREAD_STORE_EXISTS.load(Ordering::Relaxed) {
            return Err(String::from("XThreadStore already exists!"));
        } else {
            // By design, the first call is made by main(), so the possibility
            // of another thread concurrently writing this value is zero.
            X_THREAD_STORE_EXISTS.store(true, Ordering::Relaxed);
            return Ok(Arc::new(XThreadStore::new()));
        }
    }

    // Getter methods
    pub fn get_example(&self) -> Arc<Mutex<i32>> {
        self.example.clone()
    }

    // Setter methods
    pub fn set_example(&self, value: i32) {
        let mut val = self.example.lock().unwrap();
        *val = value;
    }
}

#[cfg(test)]
// Test only function that returns an Arc to a singleton XThreadStore.
pub fn new_arc_singleton() -> Arc<XThreadStore> {
    X_THREAD_STORE_EXISTS.store(true, Ordering::Relaxed);
    return Arc::new(XThreadStore::new());
}


static X_THREAD_STORE_EXISTS: AtomicBool = AtomicBool::new(false);