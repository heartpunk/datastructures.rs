trait ImmStack<T> {
    fn push(self, item : T) -> Self;
    fn pop(self) -> (Option<T>, Option<Self>);
    fn new() -> Self;
}

#[deriving(Eq, ToStr)]
enum Chain<T> {
    Link(T, ~Chain<T>),
    Break
}

impl<T> ImmStack<T> for Chain<T> {
   fn push(self, item : T) -> Chain<T> {
       Link(item, ~self)
   }
   fn pop(self) -> (Option<T>, Option<Chain<T>>) {
       match self {
           Link(item, ~new_self) => return (Some(item), Some(new_self)),
           Break => return (None, None)
       }
   }
   fn new() -> Chain<T> {
       Break
   }
}

fn main() {
    let mut stack : ~Chain<int> = ~ImmStack::new();
    let mut i = 0;
    while i <= 100000000 {
        stack = ~stack.push(i);
        i += 1;
    }
}
