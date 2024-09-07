use std::fs::DirEntry;

use file_util::{is_audio, is_image, is_video};

use crate::crawler::NodeType;

pub  mod file_util;


pub(crate) fn thousends_seperator(i: u64) -> String {
    let mut s = String::new();
    let i_str = i.to_string();
    let a = i_str.chars().rev().enumerate();
    for (idx, val) in a {
        if idx != 0 && idx % 3 == 0 {
            s.insert(0, ',');
        }
        s.insert(0, val);
    }
    s
}


pub(crate) fn get_file_type(entry: &DirEntry) -> NodeType {
    let  metadata=entry.metadata().expect("failed to get directory metadata");
    let path=entry.path();
    let extention=path.extension();
    if metadata.is_dir(){
        NodeType::Directory
    }
    else  {
        match extention {
            Some(ext)=>{
                let ext_str=ext.to_str().unwrap_or("");
                if is_video(ext_str){
                    NodeType::Video
                }
                else if is_audio(ext_str) {
                    NodeType::Audio
                }
                else if is_image(ext_str) {
                    NodeType::Image
                }
                else {
                    NodeType::File
                }
            },
            None=>NodeType::File
        }
    }
}
mod tests {

    #[test]
    fn thousends_seperator_test() {
        assert_eq!("1,000,000", super::thousends_seperator(1_000_000));
        assert_eq!("1", super::thousends_seperator(1));
        assert_eq!("1", super::thousends_seperator(001));
        assert_eq!("1", super::thousends_seperator(00001));
    }
}
