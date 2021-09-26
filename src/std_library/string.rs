trait VectorString { fn join_to_string(&self) -> String; }
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
trait ToChars { fn to_chars(&self) -> Vec<char>; }
trait ToBytes { fn to_bytes(&self) -> Vec<u8>; }
impl ToChars for String {
    fn to_chars(&self) -> Vec<char> {
        return self.chars().collect::<Vec<_>>();
    }
}
impl ToBytes for String {
    fn to_bytes(&self) -> Vec<u8> {
        return self.chars().map(|ch| (ch as u8)).collect::<Vec<_>>();
    }
}
impl ToBytes for Vec<char> {
    fn to_bytes(&self) -> Vec<u8> {
        return self.iter().map(|&ch| (ch as u8)).collect::<Vec<_>>();
    }
}
trait ToByte { fn at(&self, index: usize) -> u8; }
impl ToByte for Vec<char> {
    fn at(&self, index: usize) -> u8 {
        return self[index] as u8;
    }
}