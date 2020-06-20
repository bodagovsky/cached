pub mod lru;

#[cfg(test)]
pub mod tests {
    use super::lru::source;
    #[test]
    fn case_1() {
       let mut lru = source::LRUCache::new(1);
       lru.put(1,1);
       assert_eq!(lru.get(1), 1)
    }

    #[test] 
    fn case_2() {
        let mut lru = source::LRUCache::new(1);
        lru.put(1,1);
        assert_eq!(lru.get(2), -1)
    }

    #[test]
    fn case_3() {
        let mut lru = source::LRUCache::new(1);
        lru.put(1,1);
        lru.put(2,2);
        assert_eq!(lru.get(1), -1)
    }
}