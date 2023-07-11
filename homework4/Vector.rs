struct Vector<T> {
    data: Vec<T>,
}

impl<T> Vector<T> {
    fn new() -> Self {
        Vector { data: Vec::new() }
    }

    fn with_capacity(capacity: usize) -> Self {
        Vector {
            data: Vec::with_capacity(capacity),
        }
    }

    fn push(&mut self, value: T) {
        self.data.push(value);
    }

    fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    fn remove(&mut self, index: usize) -> Option<T> {
        if index < self.data.len() {
            Some(self.data.remove(index))
        } else {
            None
        }
    }

    fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }

    fn resize(&mut self, new_size: usize) {
        self.data.resize(new_size, Default::default());
    }
}

fn main() {
    let mut vector: Vector<i32> = Vector::new();
    vector.push(1);
    vector.push(2);
    vector.push(3);

    println!("{:?}", vector.pop()); 

    println!("{:?}", vector.remove(0));

    println!("{:?}", vector.get(0)); 

    vector.resize(5);
}
