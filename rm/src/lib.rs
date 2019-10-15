pub mod page;

use io::bm::BufManager;
use util::constants::*;

//Rid.0:page_id,  Rid.1: offset 
pub struct Rid(usize, usize);

pub struct Record {

}


pub struct RecordManager {
    pub bm: BufManager,
}

impl RecordManager {
    pub fn new() -> RecordManager {
        RecordManager {
            bm: BufManager::new(),
        }
    }

    //创建新文件(新表)
    //返回文件id
    pub fn create_file(&mut self, fname: &str, record_len: usize) -> usize {
        let file_id =  self.bm.fm.create_file(fname);
        
        //写入第0,1页信息进入缓冲区，并直接写回
        let page_zero = self.bm.alloc_page(file_id, 0, false);

        //unimplemented!();
        //TODO
        //test!!
        let info = page::FileInfoPage::new();


        let bytes: &[u8] = unsafe {util::any_as_u8_slice(&info)};
        self.bm.write(page_zero, bytes, 0);

        let page_one = self.bm.alloc_page(file_id, 1, false);
        //第1页全写0
        self.bm.write(page_one, &[0;PAGE_SIZE], 0);

        //写回
        self.bm.write_back(page_zero);
        self.bm.write_back(page_one);

        file_id
    }


    //删除文件(表)
    //可能失败 正常失败->文件不存在
    pub fn remove_file(&mut self, fname: &str) -> Result<(), &str> {
        self.bm.fm.remove_file(fname)
    }


    //TODO: 需要这个接口? 还是直接上层调用fileManager接口即可?
    pub fn get_file_id(&self, fname: &str) -> usize {
        self.bm.fm.get_file_id(fname.to_string())
    }
    
    /******单条记录接口*********/
    pub fn get_record(&self, rid: Rid) -> Option<()> {
        //TODO: 记录类型
        unimplemented!()
    }

    pub fn insert_record(&mut self, data: &[u8]) {
        unimplemented!()
    }

    pub fn update_record(&mut self, data: &[u8]) {
        unimplemented!()
    }

    pub fn remove_record(&mut self, rid: Rid) {
        unimplemented!()
    }
    /*******获得属性值满足某一要求的记录******/
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rm_create_first_page() {
        let mut rm = RecordManager::new();

        //setup
        rm.bm.fm.create_db("tmp_db");
        rm.bm.fm.use_db("tmp_db");

        let file_id = rm.bm.fm.create_file("tmp.table");
        let page_zero = rm.bm.alloc_page(file_id, 0, false);

        //往buf里写
        let mut info = page::FileInfoPage::test_new();
        info.column_num = 10;
        info.page_num = 0;
        info.record_len = 5;
        info.record_num = 0;

        //写入buf
        let bytes: &[u8] = unsafe {util::any_as_u8_slice(&info)};
        //println!("{:?}", bytes);

        rm.bm.write(page_zero, bytes, 0);


        //写第一页
        let page_one = rm.bm.alloc_page(file_id, 1, false);

        //全部写回
        rm.bm.write_back(page_zero);
        rm.bm.write_back(page_one);

        let page_zero = rm.bm.alloc_page(file_id, 0, true);

        let new_info: page::FileInfoPage = unsafe {
            util::read_from_u8(rm.bm.read(page_zero, std::mem::size_of::<page::FileInfoPage>()))
        };

        assert_eq!(new_info.column_num, 10);
        assert_eq!(new_info.page_num, 0);
        assert_eq!(new_info.record_len, 5);
        assert_eq!(new_info.record_num, 0);

        std::mem::drop(rm);
        std::fs::remove_dir_all("./../data/tmp_db").expect("remove tmp_db/");
    }
}
