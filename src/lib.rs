pub mod xthreadstore_impl; 

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

use std::{thread::{self, sleep, JoinHandle}, time::Duration};

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use xthreadstore_impl::{XThreadStore, new_arc_singleton};
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_example_atomicity() {
        let x_thread_store_ptr = new_arc_singleton();;
        let mut threads: Vec<JoinHandle<()>> = Vec::new();
        // create a vector of values that we expect each thread to have
        let vals: Vec<i32> = vec![1, 2, 3, 4, 5];
        let expected_example_vals: Vec<i32> = vec![0, 1, 2, 3, 4];
        let vals_rc = Arc::new(vals);
        let expected_example_vals_rc = Arc::new(expected_example_vals);
        for i in 0..5 {
            // create a pointer here that will be moved to the thread, since the
            // original pointer does not implement Copy (and shouldn't, XThreadstore
            // should be a singleton)
            let my_ptr = x_thread_store_ptr.clone();
            let vrc = vals_rc.clone();
            let eevrc = expected_example_vals_rc.clone();
            let index = i as usize;
            let handle = thread::spawn(move|| {
                // thread i sleeps for i+1 seconds
                let sleep_time = i+1;
                sleep(Duration::from_secs(sleep_time));
                // get example var, and assert that it is the expected value
                let example_var = *my_ptr.get_example().lock().unwrap();
                
                let expected_val = eevrc[index];
                assert_eq!(example_var, expected_val);
                // Set a new value to example
                let new_val = vrc[index];
                my_ptr.set_example(new_val);
            });
            threads.push(handle);
        }
        for t in threads {
            match t.join() {
                Ok(_) => {}
                Err(e) => {assert!(false, "Unexpected error: {e:?}")}
            };
        }
        let final_example_val = *x_thread_store_ptr.get_example().lock().unwrap();
        assert_eq!(final_example_val, vals_rc[4]);
    }

    #[test]
    fn test_singleton() {
        new_arc_singleton();
        match XThreadStore::new_arc() {
            Ok(_) => assert!(false, "Singleton breached!"),
            Err(_) => {}
        };
    }
}
