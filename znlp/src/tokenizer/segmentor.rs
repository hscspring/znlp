use std::path::PathBuf;


struct WordSegmentor {
   dict_path: Pathbuf,
   user_dict_path: Pathbuf,
}

struct CharSegmentor {
}

impl WordSegmentor {
    fn new(dict_path: Pathbuf, user_dict_path: Pathbuf) -> Self {
        dict_path,
        user_dict_path,
    }
}



#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
