use genezip::huffman::tree::HuffmanTree;

fn main() {
    let text = "ATGCCCGTAGCGCGCGCCCCATATAT".chars().collect::<Vec<char>>();
    let tree = HuffmanTree::from_data(&text);

    for elem in text {
        println!("{} -{:?}-> {}", elem, tree.encodings_to(&elem), tree.get_to(tree.encodings_to(&elem).unwrap().clone()).unwrap());
    }
}
