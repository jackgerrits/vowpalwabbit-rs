use object_pool::Pool;

use crate::{example::Example, multi_example::MultiExample};

pub struct ExamplePool {
    example_pool: Pool<Example>,
    multi_example_pool: Pool<MultiExample>,
}

impl ExamplePool {
    pub fn new() -> ExamplePool {
        // Initial size is zero.
        ExamplePool {
            example_pool: Pool::new(0, Example::new),
            multi_example_pool: Pool::new(0, MultiExample::new),
        }
    }

    pub fn get_example(&self) -> Example {
        self.example_pool.pull(Example::new).detach().1
    }

    pub fn return_example(&self, mut example: Example) {
        example.clear();
        self.example_pool.attach(example)
    }

    pub fn get_multi_example(&self) -> MultiExample {
        self.multi_example_pool.pull(MultiExample::new).detach().1
    }

    pub fn return_multi_example(&self, mut example: MultiExample) {
        while !example.is_empty() {
            let mut ex = example.extract_at(example.len() - 1).unwrap();
            ex.clear();
            self.return_example(ex);
        }
        assert!(example.is_empty());
        self.multi_example_pool.attach(example);
    }
}

impl Default for ExamplePool {
    fn default() -> Self {
        Self::new()
    }
}
