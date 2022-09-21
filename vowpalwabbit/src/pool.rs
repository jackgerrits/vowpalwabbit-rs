use object_pool::Pool;

use crate::{
    example::{Example, RawExample},
    multi_example::{MultiExample, RawMultiExample},
};

pub struct ExamplePool {
    example_pool: Pool<RawExample>,
    multi_example_pool: Pool<RawMultiExample>,
}

pub trait ReturnToPool<T> {
    // Requires interior mutability
    fn return_example(&self, object: T);
}

impl ReturnToPool<Example> for ExamplePool {
    fn return_example(&self, object: Example) {
        self.example_pool.attach(object.clear())
    }
}

impl ReturnToPool<MultiExample> for ExamplePool {
    fn return_example(&self, mut object: MultiExample) {
        while !object.is_empty() {
            let ex = object.extract_at(object.len() - 1).unwrap();
            self.return_example(ex.clear());
        }
        assert!(object.is_empty());
        let ptr = object.get_mut_ptr();
        std::mem::forget(object);
        self.multi_example_pool
            .attach(RawMultiExample { multi_example: ptr });
    }
}

impl ReturnToPool<RawExample> for ExamplePool {
    fn return_example(&self, mut object: RawExample) {
        object.clear();
        self.example_pool.attach(object);
    }
}

impl ReturnToPool<RawMultiExample> for ExamplePool {
    fn return_example(&self, mut object: RawMultiExample) {
        while !object.is_empty() {
            let mut ex = object.extract_at(object.len() - 1).unwrap();
            ex.clear();
            self.return_example(ex);
        }
        assert!(object.is_empty());
        self.multi_example_pool.attach(object);
    }
}

impl ExamplePool {
    pub fn new() -> ExamplePool {
        // Initial size is zero.
        ExamplePool {
            example_pool: Pool::new(0, RawExample::new),
            multi_example_pool: Pool::new(0, RawMultiExample::new),
        }
    }

    pub fn get_example(&self) -> RawExample {
        self.example_pool.pull(RawExample::new).detach().1
    }

    pub fn get_multi_example(&self) -> RawMultiExample {
        self.multi_example_pool
            .pull(RawMultiExample::new)
            .detach()
            .1
    }
}

impl Default for ExamplePool {
    fn default() -> Self {
        Self::new()
    }
}
