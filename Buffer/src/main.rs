fn main() {
    let container1 = vec![1, 2, 3];
    let container2 = vec![1.1, 2.2, 3.3];
    let Buffer1 = Buffer{container: container1};
    let Buffer2 = Buffer{container: container2};
    println!("{}", Buffer1.Sum());
    println!("{}", Buffer2.Sum());
}
struct Buffer<T> {
    container: Vec<T>
}
impl<T: for<'a> std::ops::AddAssign<&'a T> + std::marker::Copy> Buffer<T> {
    fn Sum(&self) -> T {
        let mut sum = self.container[0];
        for number in &self.container {
            sum += number;
        }
        sum
    }
}
