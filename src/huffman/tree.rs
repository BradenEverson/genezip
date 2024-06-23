use std::{borrow::Borrow, cell::RefCell, collections::{BinaryHeap, HashMap}, hash::Hash, rc::Rc};


#[derive(Debug)]
struct Node<V> {
    pub(crate) freq: usize,
    pub(crate) value: Option<V>,
    pub(crate) left: Option<HuffmanNode<V>>,
    pub(crate) right: Option<HuffmanNode<V>>,
}

type HuffmanNode<V> = Rc<RefCell<Node<V>>>;

impl<V> Node<V> {
    pub fn new(freq: usize, value: Option<V>) -> Self {
        Self {
            freq,
            value,
            left: None,
            right: None
        }
    }

    pub fn new_branch(freq: usize, value: Option<V>) -> HuffmanNode<V> {
        let node = Self::new(freq, value);
        Rc::new(RefCell::new(node))
    }

    pub fn merge(left: HuffmanNode<V>, right: HuffmanNode<V>) -> HuffmanNode<V> {

        let left_freq: &RefCell<Node<V>> = left.borrow();
        let right_freq: &RefCell<Node<V>> = right.borrow();

        let node = Self::new(left_freq.borrow().freq + right_freq.borrow().freq, None);
        Rc::new(RefCell::new(node.with_right(right).with_left(left)))
    }

    pub fn with_left(mut self, left: HuffmanNode<V>) -> Self {
        self.left = Some(left);
        self
    }

    pub fn with_right(mut self, right: HuffmanNode<V>) -> Self {
        self.right = Some(right);
        self
    }
}

impl<V> Ord for Node<V> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.freq.cmp(&other.freq)
    }
}

impl<V> PartialOrd for Node<V> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<V> PartialEq for Node<V> {
    fn eq(&self, other: &Self) -> bool {
        self.freq == other.freq
    }
}

impl<V> Eq for Node<V> {}

pub struct HuffmanTree<V: Eq + Hash + Copy> {
    root: HuffmanNode<V>,
    frequencies: HashMap<V, usize>
}

impl<V: Eq + Hash + Copy> HuffmanTree<V> {

    pub fn from_data(from: &[V]) -> Self {
        let freq = Self::create_frequency_table(from);
        let huffman = Self::create_huffman(&freq);

        Self {
            root: huffman.unwrap(),
            frequencies: freq
        }
    }

    fn create_huffman(freq_table: &HashMap<V, usize>) -> Option<HuffmanNode<V>> {
        let mut heap = BinaryHeap::new();

        for (&val, &freq) in freq_table {
            heap.push(Node::new_branch(freq, Some(val)));
        }

        while heap.len() > 1 {
            let left = heap.pop().unwrap();
            let right = heap.pop().unwrap();

            let merged = Node::merge(left, right);
            heap.push(merged);
        }

        heap.pop()
    }

    fn create_frequency_table(from: &[V]) -> HashMap<V, usize> {
        let mut res = HashMap::new();
        
        for elem in from {
            *res.entry(*elem).or_insert(0) += 1;
        }
            
        res
    }
}
