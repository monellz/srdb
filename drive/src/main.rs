use io::bm;
fn main() {
    let mut b = bm::BufManager::new();
    b.fm.create_db("test_db");
    b.fm.use_db("test_db");
    
    b.fm.create_file("testfile.txt");
    b.fm.create_file("testfile2.txt");
    let file_id = b.fm.get_file_id("testfile.txt".to_string());
    let file_id2 = b.fm.get_file_id("testfile2.txt".to_string());
    println!("Hello, srdb!");
    for i in 0..100 {
        let page_id = b.alloc_page(file_id, i, false);

        let buf = b.get_mut_buf_page(page_id, 2);
        //println!("{}", page_id as u8);
        buf[0] = page_id as u8;
        buf[1] = file_id as u8;
        b.mark_dirty(page_id); //标记脏页

        let page_id = b.alloc_page(file_id2, i, false);
        let buf = b.get_mut_buf_page(page_id, 2);
        buf[0] = page_id as u8;
        buf[1] = file_id2 as u8;
        b.mark_dirty(page_id);
    }


    for i in 0..100 {
        let page_id = b.get_buf_id(file_id, i);
        let buf_len = 2;
        let buf = b.get_buf_page(page_id, buf_len);
        println!("file_id {} -> {} : {}", file_id, buf[0], buf[1]);
        b.access(page_id);

        let page_id = b.get_buf_id(file_id2, i);
        let buf_len = 2;
        let buf = b.get_buf_page(page_id, buf_len);
        println!("file_id {} -> {} : {}", file_id2, buf[0], buf[1]);
        b.access(page_id);
    }

    b.write_back_all();
}
