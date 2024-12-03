use candid::CandidType;

#[derive(Clone, CandidType)]
pub struct Blog {
    title: String,
    date: u32,
    content: String,
    tags: Vec<String>
}

impl Blog {
    pub fn new(title: String, date: u32, content: String, tags: Vec<String>) -> Self {
        Self {
            title,
            date,
            content,
            tags
        }
    }
}
// u32 - liczba nie ujemna która spełnia pewną zasadnę np.
// typ "u" to inaczej unsigned ,czyli nie ujemny
// u8 - liczba z zakresu od 0 do 255 (256 - 1)