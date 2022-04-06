pub trait DictionaryTrait<TKey, TValue> {
    fn add(self, key: TKey, value: TValue);
    fn get_hash_code();
}

pub struct Dictionary {}

impl DictionaryTrait<i32, String> for Dictionary {
    fn add(self, key: i32, value: String) {}
    fn get_hash_code() {}
}
