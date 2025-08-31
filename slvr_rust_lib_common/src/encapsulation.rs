
pub trait Encapsulates<T> {
    fn encapsulated_get(&self) -> &T;
    fn encapsulated_get_mut(&mut self) -> &mut T;
    fn encapsulated_set(&mut self, value: T);
}
