#[deriving(Eq, ToStr, Clone)]
enum BinaryTree<T> {
    Branch(~Option<BinaryTree<T>>, ~Option<BinaryTree<T>>),
    Leaf(T)
}

fn main() {
  let tree : ~BinaryTree<int> = ~Branch(~Some(Leaf(1)), ~None);
  println(tree.to_str());
  println(Branch(~Some(Leaf(2)), ~Some(*tree)).to_str());
}
