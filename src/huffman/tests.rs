#[cfg(test)]
mod tests {
    use crate::huffman::tree::HuffmanTree;

    #[test]
    pub fn test_encodings_work_chars() {
        let input: Vec<char> = "The quick brown fox jumped over my fat dog Teller, they soon found out they had much in common over tea".chars().collect();
        let huffman = HuffmanTree::from_data(&input);

        for character in input {
            let directions_to = huffman.encodings_to(&character);
            assert!(directions_to.is_some());
            let result_to = huffman.get_to(directions_to.unwrap().clone()).expect("Directions lead somewhere");
            assert_eq!(character, result_to)
        }
    }


    #[test]
    pub fn test_encodings_work_strings() {
        let input: Vec<String> = "My name is Braden Everson woohoo I sure do love Rust it is cool and good".split(" ").map(|str| str.to_string()).collect();
        let huffman = HuffmanTree::from_data(&input);

        for character in input {
            let directions_to = huffman.encodings_to(&character);
            assert!(directions_to.is_some());
            let result_to = huffman.get_to(directions_to.unwrap().clone()).expect("Directions lead somewhere");
            assert_eq!(character, result_to)
        }
    }


    #[test]
    pub fn test_encodings_work_integer_types() {
        let input = vec![1,2,3,4,4,4,4,5,99,150,88,19203480,312432,4132,2,2,2,3,23,24,321,4,1,321,3,12,432,432,14,32]; //Generated by a beautiful RNG system known as randomly slapping keyboard
        let huffman = HuffmanTree::from_data(&input);

        for character in input {
            let directions_to = huffman.encodings_to(&character);
            assert!(directions_to.is_some());
            let result_to = huffman.get_to(directions_to.unwrap().clone()).expect("Directions lead somewhere");
            assert_eq!(character, result_to)
        }
    }


}