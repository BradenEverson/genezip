use genezip::huffman::tree::HuffmanTree;

fn main() {
    let text = "ATGCCCGTA".chars().collect::<Vec<char>>();
    let tree = HuffmanTree::from_data(&text);

    for elem in text {
        println!("How to get to {} in tree: \n{:?}", elem, tree.encodings_to(&elem));
    }
}
