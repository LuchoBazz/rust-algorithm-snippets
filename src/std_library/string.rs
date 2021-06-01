trait VectorString {
    fn join_to_string(&self) -> String;
}
impl VectorString for Vec<u8> {
    fn join_to_string(&self) -> String {
        return self.iter().map(|&x| (x as char).to_string()).collect::<String>();
    }
}
impl VectorString for Vec<char> {
    fn join_to_string(&self) -> String {
        return self.iter().map(|x| x.to_string()).collect::<String>();
    }
}
trait StringVec {
    fn to_vector(&self) -> Vec<char>;
}
impl StringVec for String {
    fn to_vector(&self) -> Vec<char> {
        return self.chars().collect::<Vec<_>>();
    }
}