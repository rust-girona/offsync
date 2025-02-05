use std::collections::BTreeMap;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::fs;

// ImageStat struct 
#[derive(Debug)]
#[derive(Hash)]
struct ImgSt {
    path: String,
    hash: u64,
    st_size: u64,
    st_mtime: u64
}

/* 
impl Hash for ImgSt {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.phone.hash(state);
    }
}
*/


// assert_eq!(calculate_hash(&person1), calculate_hash(&person2));

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}


fn getImgSt(path:String) -> ImgSt {
    let stat = fs::metadata(path);
    let imgSt = ImgSt { 
        path: path, hash: 0, // calculate_hash(path), 
        st_size: stat.expect("File not found").len(),
        st_mtime: stat.expect("File not found").modified()
    };
    imgSt
}


pub(crate) fn test_data() {
  let mut imgMap: BTreeMap<u64, ImgSt> = BTreeMap::new(); // BTreeMap for storing names (strings) and scores (u32)

  let imgSt1 : ImgSt = ImgSt {
    path: String::from("/home/orambla/Pictures/Dni1.jpg"),
    hash: 1,
    st_size: 1024,
    st_mtime: 1234567890
  };
  let imgSt2  : ImgSt = ImgSt {
    path: String::from("/home/orambla/Pictures/Dni2.jpg"),
    hash: 2,
    st_size: 1024,
    st_mtime: 1234567890
  };

  let hash1 = calculate_hash(&imgSt1);
  let hash2 = calculate_hash(&imgSt2);
  imgMap.insert(hash1, imgSt1);
  imgMap.insert(hash2, imgSt2);

  // Access a value by key
  let result: Option<&ImgSt> = imgMap.get(&hash1);
  match result {
    Some(img) => println!("Img1: {:?}", img),
    None => println!("No image found")
  }
//   println!("Img1: {:?}", img1);  

  // Iterate over key-value pairs in sorted order
  for (hash, img) in &imgMap {
    println!("{}: {}", hash, img.path);
  }
}