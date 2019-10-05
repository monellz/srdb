pub mod fm;
pub mod bm;


#[cfg(test)]
mod tests {
    use super::fm::FileManager;
    use super::bm::BufManager;
    use bimap::BiMap;
    use util::constants::*;
    #[ignore]
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
    fn bimap_test() {
        let mut hash: BiMap<usize, (usize, usize)> = BiMap::new();
        hash.insert(0, (0, 1));
        assert_eq!(hash.get_by_left(&0), Some(&(0, 1)));
        assert_ne!(hash.get_by_left(&0), Some(&(1, 1)));
        
        assert_eq!(hash.get_by_right(&(0, 1)), Some(&0));
        assert_eq!(hash.get_by_right(&(0, 0)), None);
        assert_ne!(hash.get_by_right(&(0, 1)), Some(&1));

        hash.insert(0, (1, 1));
        println!("{:?}", hash);
    }

    //#[ignore]
    #[test]
    fn bm_test() {
        let mut bm = BufManager::new();
        println!("bm created");
        bm.fm.create_db("tmp_db");
        bm.fm.use_db("tmp_db");
        bm.fm.create_file("tmp.table");

        let buf = [b'b'; PAGE_SIZE];
        bm.fm.write_page("tmp.table", 0, &buf);
        let file_id = bm.fm.get_file_id("tmp.table".to_string());

        let idx = bm.alloc_page(file_id, 0, true);

        let idx2 = bm.get_buf_id(file_id, 0);

        assert_eq!(idx, idx2);
        let buf = [b'a'; PAGE_SIZE];
        bm.write(idx, &buf, 0);
        bm.write_back(idx);
        bm.fm.write_page("tmp.table", 0, &buf);

        //检查
        let mut res = [b'\0'; PAGE_SIZE];
        bm.fm.read_page("tmp.table", 0, &mut res);

        for i in 0..PAGE_SIZE {
            assert_eq!(res[i], buf[i], "err happen at {}", i);
        }

        std::mem::drop(bm);
        std::fs::remove_dir_all("./../data/tmp_db").expect("remove tmp_db/");
    }
}
