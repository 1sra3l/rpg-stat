pub mod stats;

#[cfg(test)]
mod tests {
    use crate::stats::Basic;
    #[test]
    fn test_basic() {
        let b:Basic = Basic::default();
        println!("hp={}",b.hp);
    }
}
