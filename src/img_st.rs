use std::hash::{DefaultHasher, Hash, Hasher};
use std::collections::BTreeMap;
use std::process;
use std::io;
use std::time::SystemTime;
use std::fs::{self, DirEntry};
use std::path::{Path, PathBuf};


#[derive(Debug)]
#[derive(Hash)]
pub struct ImgSt {
    pub path: PathBuf,
    pub st_size: u64,
    pub st_mtime: SystemTime
}

pub fn new_image_stat(path: &PathBuf) -> std::io::Result<ImgSt> {
//    let path_buf = PathBuf::from(path);
    let stat = fs::metadata(path)?;
    let img_st:ImgSt = ImgSt {
            path: PathBuf::from(path),
            st_size: stat.len(),
            st_mtime: stat.modified()?
    };
    Ok(img_st)
}

impl ImgSt {
  pub fn new(path: &PathBuf) -> ImgSt {
    let res= new_image_stat(path);
    let img_st = match res {
        Ok(img ) =>  img,
        Err(err) =>  {
            println!("Error {} reading image file {}", err, path.display());
//         Err(err) =>  panic!("Error reading image file {}", path);
            process::exit(1);
        }
    };
    img_st
  }
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

/*
impl Hash for ImgSt {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.phone.hash(state);
    }
}
*/

struct ImgMap {
    img_map: BTreeMap<u64, ImgSt> // BTreeMap for storing names (strings) and scores (u32)
}

impl ImgMap {
  pub fn new() -> ImgMap {
    ImgMap {
        img_map : BTreeMap::new() // BTreeMap for storing names (strings) and scores (u32)
    }
  }

  pub fn insert(&mut self, img_st: ImgSt)-> u64 {
    let hash: u64 = calculate_hash(&img_st);
    // let hash: u64 = img_st.hash(); 
    self.img_map.insert(hash, img_st);
    hash
  }

  pub fn get(&self, hash: &u64) -> Option<&ImgSt>{
    self.img_map.get(&hash)
  }

  pub fn print(&self) {
    // Iterate over key-value pairs in sorted order
    for (hash, img) in &self.img_map {
      print_img_st(hash, img);
    }
  }
}  

/*
static mut img_map: BTreeMap<u64, ImgSt> = BTreeMap::new(); // BTreeMap for storing names (strings) and scores (u32)

pub fn insert_img_st(img_st: ImgSt) -> u64 {
    let hash: u64 = calculate_hash(&img_st);
    // let hash: u64 = img_st.hash(); 
    unsafe {
        img_map.insert(hash, img_st);
    }
    hash
}

fn read_img_dir(path:&str) {
    let paths = fs::read_dir(dir).unwrap();
    for path in paths {
        let path = path.unwrap().path();
        let img_st = ImgSt::new(&path);
        insert_img_st(img_st);
    }
}       
*/

fn visit_dirs(dir: PathBuf, img_map: &mut ImgMap, cb: &dyn Fn(&DirEntry, &mut ImgMap)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(path, img_map, cb)?;
            } else {
                cb(&entry, img_map);
            }
        }
    }
    Ok(())
}

pub fn load_dir(img_map: &ImgMap, dir: PathBuf) -> io::Result<()> {
    visit_dirs(dir, img_map, &|entry, img_map| {
        let path = entry.path();
        let img_st = ImgSt::new(&path);
        img_map.insert(img_st);
    })
}

pub fn print_img_st(hash:&u64, img: &ImgSt) {
      println!("img_st:{}:{}:{}:{}", *hash, img.path.display(),img.st_size, img.st_mtime.elapsed().unwrap().as_secs());
}


#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  pub fn test_files() {
    let mut img_map : ImgMap = ImgMap::new();
    let img_st1 : ImgSt = ImgSt::new(&PathBuf::from("/home/orambla/Pictures/Dni1.jpg"));
    let img_st2  : ImgSt = ImgSt::new(&PathBuf::from("/home/orambla/Pictures/Dni2.jpg"));
    let hash1 = img_map.insert(img_st1);
    let _hash2 = img_map.insert(img_st2);  

    // Access a value by key
    let result: Option<&ImgSt> = img_map.get(&hash1);
    match result {
      Some(img) => println!("Img1: {:?}", img),
      None => println!("No image found")
    }
    img_map.print();
  }

  #[test]
  pub fn test_dir() {
    let mut img_map : ImgMap = ImgMap::new();
    load_dir(&mut img_map, PathBuf::from("/home/orambla/Pictures")).unwrap();
    img_map.print();
  }
}