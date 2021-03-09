pub struct Pip;

impl Component for Pip {
    type Storage = DenseVecStorage<Self>;
}

#[cfg(test)]
mod tests {
    use super::Pip;
 
    #[test]
    pub fn new_snake() {
        let s = Pip::new();
        println!("{}", s.length);

        assert!(true);
    }
}