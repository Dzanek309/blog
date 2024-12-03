use candid::CandidType;
use ic_cdk::api::time;

#[derive(Clone, CandidType)]
pub struct Blog {
    title: String,
    date: u64,
    content: String,
    tags: Vec<String>
}

impl Blog {
    pub fn new(title: String, content: String, tags: Vec<String>) -> Self {
        Self {
            title,
            date: time(),
            content,
            tags
        }
    }
}
// u32 - liczba nie ujemna która spełnia pewną zasadnę np.
// typ "u" to inaczej unsigned ,czyli nie ujemny
// u8 - liczba z zakresu od 0 do 255 (256 - 1)