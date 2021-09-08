#[derive(Clone)]
struct Node {
    val: i64,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(val:i64) -> Node {
        Node{val, next:None}
    }

    fn push(&mut self, val:i64){
        if let Some(node) = &self.next {
            self.next.as_ref().unwrap().push(val);
        } else {
            self.next = Some(Box::new(Node::new(val)));
        }
    }

    fn len(&self) -> i64 {
        match &self.next {
            Some(node) => 1 + node.len(),
            None => 0,
        }
    }
}

struct List {
    head: Box<Node>,
}

impl List {
    fn new() -> List {
        let head = Box::new(Node::new(0));
        List{head}
    }

    fn len(&self) -> i64 {
        0
    }

    fn push(&mut self, n:i64) {
        self.head.push(n)
    }

    fn remove(&self, index:i64){

    }
}

#[cfg(test)]
mod tests {
    use super::List;
    #[test]
    fn test_len(){
        let mut ls = List::new();
        assert_eq!(0, ls.len());
        ls.push(0);
        assert_eq!(1, ls.len());
        ls.push(1);
        assert_eq!(2, ls.len());
        //ls.remove(0);
        //assert_eq!(1, ls.len());
    }
}