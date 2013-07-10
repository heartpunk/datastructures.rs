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

trait MutStack<T> {
    fn push(self, item : T) -> ~Self;
    fn pop(self) -> Option<T>;
    fn new() -> Self;
}

impl<T> MutStack<T> for Chain<T> {
    fn push(self, item : T) -> ~Chain<T> {
        ~Link(item, ~self)
    }
    fn pop(self) -> Option<T> {
        match self {
            Link(item, ~_) => return Some(item),
            Break => return None
        }
    }
    fn new() -> Chain<T> {
        Break
    }
}

fn main() {
    let b : ~Chain<int> = ~MutStack::new();
    println(b.push(1).push(2).push(3).to_str());
}
