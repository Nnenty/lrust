#[derive(Debug)]
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f32,
}
impl AveragedCollection {
    pub fn new() -> Self {
        Self {
            list: vec![],
            average: 0f32,
        }
    }
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }
    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();

        self.average = total as f32 / self.list.len() as f32;
    }
    pub fn delete(&mut self) -> Option<i32> {
        match self.list.pop() {
            Some(v) => {
                self.update_average();
                Some(v)
            }
            None => None,
        }
    }
    pub fn average(&self) -> f32 {
        self.average
    }
}
