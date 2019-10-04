use util::constants::*;

use std::collections::HashMap;
use std::fs::{File, OpenOptions, create_dir};
use std::env;
use std::io::{SeekFrom, prelude::*};



pub struct FileManager {
    current_db: Option<String>,
    perm_id: HashMap<String, u32>,
    next_id: u32,
}

impl FileManager {
    pub fn new() -> FileManager {
        //设置当当前路径为data
        //println!("{:?}", env::current_dir().unwrap());
        env::set_current_dir("../data/").expect("fm::new set_current_dir");
        FileManager {
            current_db: None,
            perm_id: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn use_db(&mut self, db_name: &str) {
        self.current_db = Some(db_name.to_string());
        let path = format!("{}/perm.id", db_name);
        //读取对应数据库目录下的perm.id文件
        match File::open(&path) {
            Ok(mut f) => {
                let mut contents = String::new();
                f.read_to_string(&mut contents).unwrap();
                if contents != "" {
                    for s in contents.split("\n") {
                        let mut iter = s.split_whitespace();
                        let key = iter.next().take().unwrap();
                        let val: u32 = iter.next().take().unwrap().parse().unwrap();
                        if self.next_id <= val {
                            self.next_id = val + 1;
                        }
                        debug_assert_eq!(iter.next(), None);
                        self.perm_id.insert(key.to_string(), val);
                    }
                }
            },
            Err(_) => {
                eprintln!("create and init {}", path);
                File::create(path).expect("fm::use_db");
            }
        }
    }
    pub fn create_file(&mut self, fname: &str) {
        let db_name = self.current_db.as_ref().expect("fm::create_file parse db_name");
        let fpath = format!("{}/{}", db_name, fname);
        File::create(fpath).expect("fm::create_file");
        self.perm_id.insert(fname.to_string(), self.next_id);
        self.next_id += 1;
    }
    pub fn create_db(&self, db_name: &str) {
        create_dir(db_name).expect("fm::craete_db");        
    }

    pub fn write_page(&self, fname: &str, page_id: u32, buf: &[u8]) {
        let mut f = match self.current_db.as_ref() {
            Some(db_name) => {
                let fpath = format!("{}/{}", db_name, fname);
                OpenOptions::new().append(true).open(fpath).expect("fm::write_page open file")
            },
            None => panic!("not use any database"),
        };
        debug_assert_eq!(buf.len(), PAGE_SIZE, "buf size != PAGE_SIZE");
        f.seek(SeekFrom::Start((page_id as u64) << 13)).expect("fm::write_page seek");
        f.write_all(buf).expect("fm::write_page write_all");
    }

    pub fn read_page(&self, fname: &str, page_id: u32, buf: &mut [u8]){
        let mut f = match self.current_db.as_ref() {
            Some(db_name) => {
                let fpath = format!("{}/{}", db_name, fname);
                OpenOptions::new().read(true).open(fpath).expect("fm::write_page open file")
            },
            None => panic!("not use any database"),
        };
        debug_assert_eq!(buf.len(), PAGE_SIZE, "buf size != PAGE_SIZE");
        f.seek(SeekFrom::Start((page_id as u64) << 13)).expect("fm::read_page seek");
        f.read_exact(buf).expect("fm::read_page read_exact");
    }

    pub fn get_perm_id(&self, fname: &str) -> u32 {
        match self.perm_id.get(fname) {
            Some(&v) => v,
            None => panic!("not found fname {} in perm_id hashmap", fname),
        }
    }
}

impl Drop for FileManager {
    fn drop(&mut self) {
        //写回perm.id
        if let Some(db_name) = self.current_db.as_ref() {
            let perm_path = format!("{}/perm.id", db_name);
            let mut f = OpenOptions::new().write(true).open(perm_path).expect("fm::drop open perm.id");
            for (k, v) in &self.perm_id {
                f.write_fmt(format_args!("{} {}\n", k, v)).expect("fm::drop write fmt perm_id")
            }
        }
    }
}