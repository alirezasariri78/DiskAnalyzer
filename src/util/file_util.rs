pub (crate) fn is_video(file_extention: &str)->bool{
    let video_extentions:Vec<&str>=vec!["webm","mkv","flv","vob","ogv","drc","gif","gifv","mng","avi"
    ,"MTS","M2TS","TS","mov","qt","wmv","yuv","rm","rmvb","viv"
    ,"asf","amv","mp4","m4p","m4v","mpg","mp2","mpeg","mpe","mpv","mpg","mpeg"
    ,"m2v","svi","3gp","3g2","mxf","roq","nsv","f4v","f4p","f4a","f4b"];
    video_extentions.contains(&file_extention)
}

pub (crate) fn is_audio(file_extention: &str)->bool{
    let audio_extentions:Vec<&str>=vec!["mp3","wav","aac","wma","ogg","ape","alac","wavpack","opus"];
    audio_extentions.contains(&file_extention)
}


pub (crate) fn is_image(file_extention: &str)->bool{
    let image_extentions:Vec<&str>=vec!["jpg","png","gif","tiff","bmp","svg","webp","psd","ico","svgz","heif"];
    image_extentions.contains(&file_extention)
}