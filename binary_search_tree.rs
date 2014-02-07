#[deriving(Eq, Clone)]
enum BinaryIntTree {
  Branch(Option<~BinaryIntTree>, Option<~BinaryIntTree>, int)
}

impl ToStr for BinaryIntTree {
  fn to_str(&self) -> ~str {
    match *self {
      Branch(None, None, val) => val.to_str(),
      Branch(Some(ref left), Some(ref right), val) => format!("Branch<{:s}, {:s}, {:s}>", left.to_str(), right.to_str(), val.to_str()),
      Branch(None, Some(ref right), val) => format!("Branch<None, {:s}, {:s}>", right.to_str(), val.to_str()),
      Branch(Some(ref left), None, val) => format!("Branch<{:s}, None, {:s}>", left.to_str(), val.to_str())
    }
  }
}

fn insert(mut cur_node: BinaryIntTree, num_to_insert: int) -> () {
  match cur_node {
    Branch(_, Some(ref right), val) if num_to_insert > val => {
      println("go right!");
      insert((**right).clone(), num_to_insert);
    },
    Branch(Some(ref left), _, val) if num_to_insert < val => {
      println("go left!");
      insert((**left).clone(), num_to_insert);
    },
    Branch(_, None, val) if num_to_insert > val => {
      println("found spot!")
      // here we should replace the none with a new branch representing the new value, with no children.
      // (paying no attention to the shape of tree this makes yet! just practicing rust for now. I will
      // certainly spend time optimizing the datastructure/making it more correct after this.)
    },
    Branch(None, _, val) if num_to_insert < val => {
      println("found spot!")
      // here we should replace the none with a new branch representing the new value, with no children.
      // (paying no attention to the shape of tree this makes yet! just practicing rust for now. I will
      // certainly spend time optimizing the datastructure/making it more correct after this.)
    },
    Branch(_, _, val) if num_to_insert == val => fail!("cannot insert duplicate values."),
    Branch(_, _, _) => fail!("I can't imagine why we'd end up here?")
  }
}

fn main() {
  let mut tree: BinaryIntTree = Branch(Some(~Branch(None, None, 1)), Some(~Branch(None, None, 3)), 2);
  println(tree.to_str());
  // let mut cur_node = &mut tree;
  let num_to_insert = 10;
  insert(tree, num_to_insert);
}
