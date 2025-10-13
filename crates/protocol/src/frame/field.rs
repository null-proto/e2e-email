//
// mail metadata field strats with M_* and file metadata field starts with F_*
//


// --- mail metadata starts

/// number of file in the mail
pub static  M_FILE_COUNT : &str = "file-count";

/// from address (sender)
pub static  M_FROM : &str = "file-from";

/// to address (receiver)
pub static  M_TO :&str = "file-to";

/// mail short title
pub static  M_TITLE : &str = "file-title";


// --- mail metadata ends
// --- file metadata starts

/// file extension : after `.`(dot)
pub static  F_FILE_EXT : &str = "file-ext";

/// file size
pub static  F_FILE_SIZE : &str = "file-size";
