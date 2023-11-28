pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(v) => {
                self.update_average();
                Some(v)
            },
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

pub fn run() {
    let mut collection = AveragedCollection {
        list: vec![],
        average: 0.0,
    };

    collection.add(10);
    collection.add(20);
    collection.add(30);

    if let Some(x) = collection.remove() {
        println!("last item {}", x);
    }

    // you still can access private virable in same mod
    println!("average: {}", collection.average);
    println!("average: {}", collection.average());
}