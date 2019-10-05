use util::constants::*;

use std::fs::{File, OpenOptions, create_dir};
use std::env;
use std::io::prelude::*;
use std::os::unix::prelude::FileExt;
use bimap::BiMap;



pub struct FileManager {
    current_db: Option<String>,
    //文件名和file_id一一对应
    //便于打开文件等操作
    fname_file_id_map: BiMap<String, usize>,
    
    next_id: usize,
}

impl FileManager {
    pub fn new() -> FileManager {
        //设置当前路径为data
        //println!("{:?}", env::current_dir().unwrap());
        env::set_current_dir("../data/").expect("fm::new set_current_dir");
        FileManager {
            current_db: None,
            fname_file_id_map: BiMap::new(),
            next_id: 0,
        }
    }

    pub fn show_database(db_name: Option<&str>) -> Result<(), &str> {
        unimplemented!();
        //如果是当前数据库，则直接输出fname_file_id_map里面元素即可
    }

    pub fn use_db(&mut self, db_name: &str) {
        self.current_db = Some(db_name.to_string());
        let path = format!("{}/file.id", db_name);
        //读取对应数据库目录下的file.id文件
        match File::open(&path) {
            Ok(mut f) => {
                let mut contents = String::new();
                f.read_to_string(&mut contents).unwrap();
                if contents != "" {
                    /* 格式 
                    fname1 id\nfname2 id\n
                    */
                    for s in contents.trim().split("\n") {
                        let mut iter = s.split_whitespace();
                        let key = iter.next().take().unwrap();
                        let val: usize = iter.next().take().unwrap().parse().unwrap();
                        if self.next_id <= val {
                            self.next_id = val + 1;
                        }
                        debug_assert_eq!(iter.next(), None);

                        self.fname_file_id_map.insert(key.to_string(), val);
                    }
                }
            },
            Err(_) => {
                eprintln!("create and init {}", path);
                File::create(path).expect("fm::use_db");
            }
        }
    }

    //得到文件id 必须确保存在该文件
    //TODO: fname用&str还是String?
    pub fn get_file_id(&self, fname: String) -> usize {
        *self.fname_file_id_map.get_by_left(&fname).unwrap()
    }

    //创建新文件并返回文件id
    pub fn create_file(&mut self, fname: &str) -> usize {
        let db_name = self.current_db.as_ref().expect("fm::create_file parse db_name");
        let fpath = format!("{}/{}", db_name, fname);
        File::create(fpath).expect("fm::create_file");

        self.fname_file_id_map.insert(fname.to_string(), self.next_id);
        self.next_id += 1;
        self.next_id - 1
    }

    //删除文件
    pub fn remove_file(&mut self, fname: &str) -> Result<(), &str> {
        //确认文件存在
        match self.fname_file_id_map.remove_by_left(&fname.to_string()) {
            Some(_) => {
                //存在 则删除
                std::fs::remove_file(format!("{}/{}", self.current_db.as_ref().expect("fm::remove_file not use any database"), fname)).expect("fm::remove_file cant remove");
                Ok(())
            },
            None => Err("file not exist")
        }
    }

    pub fn create_db(&self, db_name: &str) {
        create_dir(db_name).expect("fm::craete_db");        
    }

    pub fn write_page_by_file_id(&self, file_id: usize, page_id: usize, buf: &[u8]) {
        let fname = self.fname_file_id_map.get_by_right(&file_id).expect("fm::write_page_by_file_id get fname");
        self.write_page(fname, page_id, buf);
    }

    pub fn write_page(&self, fname: &str, page_id: usize, buf: &[u8]) {
        let f = match self.current_db.as_ref() {
            Some(db_name) => {
                let fpath = format!("{}/{}", db_name, fname);
                //File::open(fpath).expect("fm::write_page open file")
                OpenOptions::new().write(true).open(fpath).expect("fm::write_page open file")
            },
            None => panic!("not use any database"),
        };
        debug_assert_eq!(buf.len(), PAGE_SIZE, "buf size != PAGE_SIZE");
        assert!(f.write_at(buf, (page_id as u64) << PAGE_SIZE_BASE).expect("fm::write_page write") == buf.len());
    }

    pub fn read_page_by_file_id(&self, file_id: usize, page_id: usize, buf: &mut [u8]) {
        let fname = self.fname_file_id_map.get_by_right(&file_id).expect("fm::read_page_by_file_id get fname");
        self.read_page(fname, page_id, buf);
    }

    pub fn read_page(&self, fname: &str, page_id: usize, buf: &mut [u8]) {
        let f = match self.current_db.as_ref() {
            Some(db_name) => {
                let fpath = format!("{}/{}", db_name, fname);
                File::open(fpath).expect("fm::read_page open file")
                //OpenOptions::new().read(true).open(fpath).expect("fm::write_page open file")
            },
            None => panic!("not use any database"),
        };
        debug_assert_eq!(buf.len(), PAGE_SIZE, "buf size != PAGE_SIZE");
        assert!(f.read_at(buf, (page_id as u64) << PAGE_SIZE_BASE).expect("fm::read_page read") == buf.len());
    }

}

impl Drop for FileManager {
    fn drop(&mut self) {
        //写回file.id
        if let Some(db_name) = self.current_db.as_ref() {
            let file_id = format!("{}/file.id", db_name);
            //使用create将旧内容覆盖
            let mut f = File::create(file_id).expect("fm::drop create file.id");
            for (k, v) in &self.fname_file_id_map {
                f.write_fmt(format_args!("{} {}\n", k, v)).expect("fm::drop write fmt file_id")
            }
        }
    }
}