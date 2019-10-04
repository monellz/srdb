pub mod constants;
pub mod list;



#[cfg(test)]
mod tests {
    use super::list::MultiList;
    #[test]
    fn list_test() {
        let mut li = MultiList::new(10, 1);
        li.insert_as_first(0, 1);
        assert_eq!(1, li.get_first(0));
        li.insert(0, 2);
        assert_eq!(1, li.get_first(0));
        li.insert_as_first(0, 2);
        assert_eq!(2, li.get_first(0));

        let f = li.get_first(0);
        assert_eq!(f, 2);
        let f = li.next(f);
        assert_eq!(f, 1);
        let f = li.next(f);
        assert!(li.is_head(f));
    }
}
