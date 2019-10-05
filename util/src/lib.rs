pub mod constants;
pub mod list;

pub unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    std::slice::from_raw_parts((p as *const T) as *const u8, std::mem::size_of::<T>())
}

pub unsafe fn read_from_u8<T: Sized>(p: &[u8]) -> T {
    std::ptr::read(p.as_ptr() as *const _)
}

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
