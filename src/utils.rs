pub trait InsertOrPush<T> {
	fn insert_or_push(&mut self, index: usize, item: T);
}

impl<T> InsertOrPush<T> for Vec<T> {
	fn insert_or_push(&mut self, index: usize, item: T) {
		if index >= self.len() {
			self.push(item);
		} else {
			self.insert(index, item);
		}
	}
}
