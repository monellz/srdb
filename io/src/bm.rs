use crate::fm::FileManager;
use util::constants::*;
use util::list::MultiList;
use std::collections::HashMap;
use bimap::BiMap;

//替换算法
struct FindReplace {
    list: MultiList,
}

impl FindReplace {
    fn new(capacity: usize) -> FindReplace {
        let mut list = MultiList::new(capacity, 1);
        for i in 0..BUF_PAGE_CAPACITY {
            list.insert(0, i as usize);
        }
        FindReplace {
            list,
        }
    }

    fn find(&mut self) -> usize {
        //找到当前空闲页索引
        //得到链表第一个元素，将其插入到链表尾并返回
        let idx = self.list.get_first(0);
        self.list.erase(idx);
        self.list.insert(0, idx);
        idx
    }

    fn access(&mut self, idx: usize) {
        //将idx标记为访问(即插入链表尾)
        self.list.insert(0, idx)
    }

    fn free(&mut self, idx: usize) {
        //将idx标记为空闲(即插入链表首，下次find即返回此idx)
        self.list.insert_as_first(0, idx);
    }
}

#[derive(Debug)]
pub struct CustomHashMap {
    //一对一hash
    //key = idx, val = (fname, page_id)
    hash: HashMap<usize, (String, usize)>,
    hash_rev: HashMap<(String, usize), usize>,
}

impl CustomHashMap {
    pub fn new() -> CustomHashMap {
        //可以预先分配内存?
        CustomHashMap {
            hash: HashMap::new(),
            hash_rev: HashMap::new(),
        }
    }

    pub fn find_page(&self, idx: usize) -> Option<&(String, usize)> {
        self.hash.get(&idx)
    }

    pub fn find_idx(&self, fname: &str, page_id: usize) -> Option<&usize> {
        self.hash_rev.get(&(fname.to_string(), page_id))
    }

    pub fn update(&mut self, idx: usize, fname: &str, page_id: usize) {
        if let Some((f, p)) = self.hash.insert(idx, (fname.to_string(), page_id)) {
            self.hash_rev.remove(&(f, p));
        }
        if let Some(i) = self.hash_rev.insert((fname.to_string(), page_id), idx) {
            self.hash.remove(&i);
        }
    }

    pub fn erase(&mut self, idx: usize) {
        //将缓存页索引idx对应的 idx fname page_id都删掉
        let (fname, page_id) = self.hash.remove(&idx).expect("bm::customhashmap erase idx");
        self.hash_rev.remove(&(fname, page_id)).expect("bm::CustomHashMap erase (fname, page_id)");
    }
}

pub struct BufManager {
    pub fm: FileManager,
    
    dirty: [bool; BUF_PAGE_CAPACITY],
    //buffer: [u8; BUF_PAGE_CAPACITY * PAGE_SIZE],
    buffer: Vec<u8>,

    //双向
    //idx:缓存页页码  <-> (file_id:文件id, page_id:物理页页码)
    hash: BiMap<usize, (usize, usize)>,

    findreplace: FindReplace,
    last: Option<usize>,
}


impl BufManager {
    pub fn new() -> BufManager {
        //TODO
        //每次更改当前数据库应该怎么处理?
        //注: buffer申请非常慢
        let mut buffer: Vec<u8> = Vec::new();
        buffer.reserve(BUF_PAGE_CAPACITY * PAGE_SIZE);
        buffer.resize(BUF_PAGE_CAPACITY * PAGE_SIZE, b'\0');
        BufManager {
            fm: FileManager::new(),
            dirty: [false; BUF_PAGE_CAPACITY],
            buffer: buffer,
            hash: BiMap::new(),
            findreplace: FindReplace::new(BUF_PAGE_CAPACITY),
            last: None, 
        }
    }
    pub fn fetch_page(&mut self, file_id: usize, page_id: usize) -> usize {
        //给文件fname的page_id页分配一个buf中的页面 返回分配的buf页号
        //调用者确保page_id页没有在缓存buf中

        //找到一个空闲页
        let idx = self.findreplace.find();
        debug_assert!(idx < BUF_PAGE_CAPACITY);
        
        if self.dirty[idx] {
            //若为dirty　则需要先写入文件才能继续使用
            let (prev_file_id, prev_page_id) = self.hash.get_by_left(&idx).expect("bm::fetch_page find page");
            self.fm.write_page_by_file_id(*prev_file_id, *prev_page_id, &self.buffer[idx * PAGE_SIZE..(idx + 1) * PAGE_SIZE]);
            self.dirty[idx] = false;
        }

        //更新bimap
        self.hash.insert(idx, (file_id, page_id));
        idx
    }

    pub fn alloc_page(&mut self, file_id: usize, page_id: usize, is_read: bool) -> usize {
        //在缓存中给file_id的page_id文件页分配缓存页
        //is_read决定是否将文件内容读入分配的缓存页中

        let idx = self.fetch_page(file_id, page_id);
        if is_read {
            //将文件内容读入buf
            self.fm.read_page_by_file_id(file_id, page_id, &mut self.buffer[idx * PAGE_SIZE..(idx + 1) * PAGE_SIZE]);
        }
        idx
    }

    pub fn get_buf_id(&mut self, file_id: usize, page_id: usize) -> usize {
        //找到fname的page_id对应的缓存的id
        match self.hash.get_by_right(&(file_id, page_id)) {
            Some(&idx) => {
                //若存在 则标记为已访问
                self.access(idx);
                idx
            },
            None => {
                //不存在　则进行缓存页分配
                let idx = self.fetch_page(file_id, page_id);
                //再将文件中的内容读入到idx对应的缓冲页中
                self.fm.read_page_by_file_id(file_id, page_id, &mut self.buffer[idx * PAGE_SIZE..(idx + 1) * PAGE_SIZE]);
                idx
            },
        }
    }

    pub fn access(&mut self, idx: usize) {
        //标记idx缓存页　被写过(即插入链表尾)
        match self.last {
            //上一次被标记为写过的　缓存页正好是这一次的 可直接返回
            Some(val) if val == idx => return,
            _ => {
                self.findreplace.access(idx);
                self.last = Some(idx);
            },
        }
    }

    pub fn mark_dirty(&mut self, idx: usize) {
        self.dirty[idx] = true;
        //标记被访问过
        self.access(idx);
    }

    pub fn release(&mut self, idx: usize) {
        //将idx标记为空闲(下次findreplace首先找到它)
        self.dirty[idx] = false;
        self.findreplace.free(idx); //此时idx被插入链表首
        self.hash.remove_by_left(&idx);
    }

    pub fn write_back(&mut self, idx: usize) {
        //将idx标记为空闲
        //判断dirty是否需要写回
        if self.dirty[idx] {
            let &(file_id, page_id) = self.hash.get_by_left(&idx).expect("bm::write back");
            println!("in write_back (file_id, page_id) and idx = ({}, {}) {}", file_id, page_id, idx);
            println!("{:?}", &self.buffer[idx * PAGE_SIZE..(idx * PAGE_SIZE + 10)]);
            self.fm.write_page_by_file_id(file_id, page_id, &mut self.buffer[idx * PAGE_SIZE..(idx + 1) * PAGE_SIZE]);
            self.dirty[idx] = false;
        }
        self.release(idx);
    }

    pub fn write_back_all(&mut self) {
        for i in 0..BUF_PAGE_CAPACITY {
            self.write_back(i as usize);
        }
    }

    pub fn get_physical_page_pos(&self, idx: usize) -> (usize, usize) {
        *self.hash.get_by_left(&idx).expect("bm::get_key")
    }

    pub fn write_back_by_file_id(&mut self, file_id: usize) {
        for i in 0..BUF_PAGE_CAPACITY {
            let &(f, _) = self.hash.get_by_left(&(i as usize)).expect("bm::write_back_by_file");
            if f == file_id { self.write_back(i); } 
        }
    }

    pub fn write(&mut self, idx: usize, buf: &[u8], offset: usize) {
        //在 idx * PAGE_SIZE + offset　开始写buf中全部内容

        //TODO 是否需要限制输入内容仅写入一页??
        if offset + buf.len() > PAGE_SIZE {
            unimplemented!();
        }
        assert!(idx * PAGE_SIZE + offset + buf.len() < BUF_PAGE_CAPACITY * PAGE_SIZE);
        unsafe {
            let dst_ptr = self.buffer.as_mut_ptr().offset((idx * PAGE_SIZE + offset) as isize);
            std::ptr::copy_nonoverlapping(buf.as_ptr(), dst_ptr, buf.len());
        }
        debug_assert!(self.buffer[idx * PAGE_SIZE + offset] == buf[0]);
        self.dirty[idx] = true;
    }
}