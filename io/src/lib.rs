pub mod fm;
pub mod bm;


#[cfg(test)]
mod tests {
    use super::fm::FileManager;
    use super::bm::{CustomHashMap, BufManager};
    use util::constants::*;
    #[test]
    fn fm_test() {
        let mut fm = FileManager::new();
        fm.create_db("tmp_db");
        fm.use_db("tmp_db");
        fm.create_file("tmp.table");
        let a: [u8;PAGE_SIZE] = [b'a'; PAGE_SIZE];
        fm.write_page("tmp.table", 0, &a);

        let mut b: [u8;PAGE_SIZE] = [b'b'; PAGE_SIZE];
        fm.read_page("tmp.table", 0, &mut b);
        
        for i in 0..PAGE_SIZE {
            assert_eq!(a[i], b[i], "err in index {}", i);
        }

        std::mem::drop(fm);
        std::fs::remove_dir_all("./../data/tmp_db").expect("remove tmp_db/");
    }

    #[test]
    fn str_test() {
        assert_eq!("asdasd", "asdasd");
        let a = "abcde";
        let bb = String::from("abcde");
        let b = &bb;
        assert_eq!(a, b);
    }

    #[test]
    fn customhashmap_test() {
        let mut hash = CustomHashMap::new();
        hash.update(0, "test.table", 1);
        assert_eq!(hash.find_page(0), Some(&("test.table".to_string(), 1)));
        assert_ne!(hash.find_page(0), Some(&("test.tabll".to_string(), 1)));
        
        assert_eq!(hash.find_idx("test.table", 1), Some(&0));
        assert_ne!(hash.find_idx("test.table", 1), Some(&1));

        hash.update(0, "test.table", 0);
        println!("{:?}", hash);
    }

    #[test]
    fn bm_test() {

    }
}
