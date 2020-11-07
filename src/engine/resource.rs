use std::collections::HashMap;
use std::sync::Arc;
use std::fs;
use std::fs::DirEntry;
use std::path::{Path, PathBuf};
use std::ops::Add;
use std::str::from_utf8;

pub struct ResourceLoader{
    buffer:HashMap<PathBuf,Vec<u8>>
}

impl ResourceLoader {
    pub fn init(){
        ResourceLoader::get_instance();
    }

    pub fn load_shader(name:&str) -> Option<&str> {
        let filepath = String::new();
        let filepath = filepath.add("data/shader/").add(name);

        let r = ResourceLoader::get_instance().buffer.get(PathBuf::from(filepath).as_path());
        r.and_then(|a|{
            from_utf8(a).ok()
        })
    }

    fn read_all_file(collector:&mut HashMap<PathBuf,Vec<u8>>,dir:DirEntry){
        if dir.file_type().unwrap().is_dir(){
            println!("Dir {:?}",dir);
            let root_dir = fs::read_dir(dir.path()).expect("资源目录不存在");
            for entry in root_dir {
                ResourceLoader::read_all_file(collector,entry.unwrap());
            }
        } else if dir.file_type().unwrap().is_file() {
            println!("File {:?}",dir);
            let data = fs::read(dir.path()).expect("资源文件读取失败");
            collector.insert(dir.path(), data);
        }
    }

    fn get_instance()->&'static Self{
        static mut INSTANCE:Option<Arc<ResourceLoader>> = None;
        unsafe {
            INSTANCE.get_or_insert_with(||{
                let mut buffer:HashMap<PathBuf,Vec<u8>> = HashMap::new();
                let root_dir = fs::read_dir("data").expect("资源目录不存在");
                for entry in root_dir {
                    ResourceLoader::read_all_file(&mut buffer,entry.unwrap());
                }
                println!("{:?}",buffer.keys());
                Arc::new(ResourceLoader{buffer})
            });

            INSTANCE.as_ref().unwrap()
        }
    }
}

impl Drop for ResourceLoader{
    fn drop(&mut self) {
        println!("Drop ResourceLoader")
    }
}