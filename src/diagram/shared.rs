
pub(crate) fn get_color_from_size(size:u64)->&'static str{
    const GREEN_SIZE_BYTES:u64=1_024_000_000;
    const YELLOW_SIZE_BYTES:u64=GREEN_SIZE_BYTES*10;
    const YELLOW_START:u64=GREEN_SIZE_BYTES+1; 
    match size {
        0..=GREEN_SIZE_BYTES=>"Green",
        YELLOW_START..=YELLOW_SIZE_BYTES=>"Yellow",
        _=>"Red",
    }
}