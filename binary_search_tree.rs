
enum BinaryIntTree<'r> {
  Branch(&'r mut Option<BinaryIntTree<'r>>, &'r mut Option<BinaryIntTree<'r>>, int)
}

fn main() {
  let mut tree: BinaryIntTree = ~Branch(~Some(Branch(~None, ~None, 1)), ~Some(Branch(~None, ~None, 3)), 2);
  // println(tree.to_str());
  let mut cur_node = &mut Some(tree);
  let num_to_insert = 10;
  loop {
    match *cur_node {
      Some(Branch(left, right, val)) => {
        if (val == num_to_insert) {
          println("can't insert a duplicate number.");
          break;
        };
        if (val > num_to_insert) {
          println("going to the right.");
        } else {
          cur_node = left;
          println("going to the left.");
        };
      },
      None => {
        println("this is where we should insert it, isn't it?");
        break;
      }
    }
  }
}
