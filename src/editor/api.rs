pub trait EditorApi {
    fn open(files: &Vec<String>, wait: bool);
}
