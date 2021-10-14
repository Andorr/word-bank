use std::vec;

struct Word {
    id: String,
    value: String,
    created_at: String,
    updated_at: String,
    translations: Vec<Translation>,
    kind: String, 
    tags: Vec<String>
}

struct Translation {
    id: String,
    word_id: String,
    value: String,
    created_at: String,
    updated_at: String,
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
