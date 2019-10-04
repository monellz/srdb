pub mod fm;
pub mod bm;


#[cfg(test)]
mod tests {
    use super::fm::FileManager;
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
}
