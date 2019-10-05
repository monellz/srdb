use io::bm::BufManager;
use bit_vec::BitVec;
//use io::fm::FileManager;
//文件内容定义
//文件读写
pub trait PageInfo {
    fn write(&mut self, bm: &mut BufManager) -> usize;
    fn read(&mut self, bm: &mut BufManager) -> usize;
}

pub enum Page {
    FileInfoPage(FileInfoPage),
    NormalPage(NormalPage),
}

pub struct FileInfoPage {
    pub page_num: u32,
    pub record_num: u32,
    pub record_len: u32,
    //列数，包括RID
    pub column_num: u32,

    //TODO: 其他参数，其他参数可能依赖于coloum_num来读取
    //列名，列类型，主键等
}

impl FileInfoPage {
    pub fn new() -> FileInfoPage {
        unimplemented!();
    }

    pub fn test_new() -> FileInfoPage {
        FileInfoPage {
            page_num: 0,
            record_num: 0,
            record_len: 0,
            column_num: 0,
        }
    }
}

pub struct NormalPage {
    header: PageHeader,
}

pub struct PageHeader {
    //若为0表示下一个不存在(待分配)
    next_free_page_idx: usize,

    //空闲槽 位图记录
    free_slot_bitvec: BitVec,
    
    //空闲槽数量 (=位图true个数)
    free_slot_num: usize,
}


impl PageHeader {
    pub fn new(record_num_per_page: usize, next_free_page_idx: usize) -> PageHeader {
        PageHeader {
            next_free_page_idx,
            free_slot_bitvec: BitVec::from_elem(record_num_per_page, false),
            free_slot_num: record_num_per_page,
        }
    }
}


