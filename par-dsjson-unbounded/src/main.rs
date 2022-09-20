use std::cell::UnsafeCell;

use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{self, BufRead};
use std::sync::{Condvar, Mutex};

// 1.5.1
use rayon::prelude::*;

use vowpalwabbit::error::VWError;
use vowpalwabbit::multi_example::MultiExample;
use vowpalwabbit::pool::ExamplePool;
use vowpalwabbit::workspace::Workspace;

struct Item<T> {
    index: usize,
    value: T,
}

impl<T> Ord for Item<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.index).cmp(&(other.index))
    }
}

impl<T> PartialOrd for Item<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> PartialEq for Item<T> {
    fn eq(&self, other: &Self) -> bool {
        (self.index) == (other.index)
    }
}

impl<T> Eq for Item<T> {}

struct OrderedParallelQueueState<T> {
    storage: BinaryHeap<Reverse<Item<T>>>,
    next_index: usize,
    is_done: bool,
}

impl<T> OrderedParallelQueueState<T> {
    fn new() -> OrderedParallelQueueState<T> {
        OrderedParallelQueueState {
            storage: BinaryHeap::new(),
            next_index: 0,
            is_done: false,
        }
    }
}

struct OrderedParallelQueue<T> {
    storage_lock: Mutex<OrderedParallelQueueState<T>>,
    cv: Condvar,
}

impl<T> OrderedParallelQueue<T> {
    fn new() -> OrderedParallelQueue<T> {
        OrderedParallelQueue {
            storage_lock: Mutex::new(OrderedParallelQueueState::new()),
            cv: Condvar::new(),
        }
    }

    fn push(&self, index: usize, value: T) {
        let mut guard = self.storage_lock.lock().unwrap();
        guard.storage.push(Reverse(Item { index, value }));

        if index == guard.next_index {
            self.cv.notify_one()
        }
    }

    fn done(&self) {
        let guard = &mut self.storage_lock.lock().unwrap();
        guard.is_done = true;
        self.cv.notify_all()
    }

    fn is_finished(&self) -> bool {
        let guard = self.storage_lock.lock().unwrap();
        guard.is_done && guard.storage.is_empty()
    }

    fn pop(&self) -> Option<T> {
        let mut guard = self.storage_lock.lock().unwrap();

        loop {
            let storage = &guard.storage;

            let is_done = guard.is_done;
            if storage.is_empty() && is_done {
                return None;
            }

            if !storage.is_empty() {
                let top_index = storage.peek().unwrap().0.index;
                let next_index = guard.next_index;

                if top_index == next_index {
                    break;
                }
            }

            guard = self.cv.wait(guard).unwrap();
        }
        let next_index = &mut guard.next_index;
        *next_index += 1;
        Some(
            guard
                .storage
                .pop()
                .expect("Heap should not be empty here")
                .0
                .value,
        )
    }
}

fn main() {
    let queue: OrderedParallelQueue<Result<MultiExample, VWError>> = OrderedParallelQueue::new();
    let args: Vec<String> = vec!["--cb_adf".to_owned(), "--quiet".to_owned()];
    let pool = ExamplePool::new();
    let workspace: UnsafeCell<Workspace> = Workspace::new(&args).unwrap().into();
    std::thread::scope(|s| unsafe {
        let ws_ref = workspace.get().as_ref().unwrap();
        s.spawn(|| {
            let file = File::open("example_datafile.json").unwrap();
            io::BufReader::new(file)
                .lines()
                .enumerate()
                .par_bridge()
                .for_each(|(line_number, line)| {
                    let line = line.unwrap();
                    queue.push(
                        line_number,
                        ws_ref.parse_decision_service_json(&line, &pool),
                    );
                });
            queue.done();
        });
        while !queue.is_finished() {
            let ex_opt = queue.pop();
            match ex_opt {
                Some(ex) => {
                    let ws = workspace.get().as_mut().unwrap();
                    let mut e = ex.unwrap();
                    ws.setup_multi_ex(&mut e).unwrap();
                    ws.learn_multi_example(&mut e).unwrap();
                    pool.return_multi_example(e);
                }
                None => break,
            }
        }
    });
}
