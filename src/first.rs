// pub enum List {
//     Empty,
//     Elem(i32, Box<List>), //这里元祖中存在i32 和 Empty 枚举中默认 0 起冲突
//                             //无法触发空指针优化
//                             //即可以在两个元素存在时，用 0 打tag从而进行优化，
//                             //设置枚举大小为另一个枚举的 最大大小+1 并且对这个值取向上到 2 的指数
// }

//以上需要 4 + 8 = 12 => 16 个字节。遇到 Empty 会产生 12 个字节的浪费

//况且在分割链表的时候 我们希望拿到分割点的智能指针，这样就可以不用把堆中的数据拷贝到栈上
//所以我们要直接能够拿到智能指针

//因此在这里思考 既要使用空指针优化又要让枚举中为智能指针 所以如下

pub struct List {
    head: Link, //结构体的大小和字段的大小相加相等， 这里是一个零成本的抽象
}


enum Link {
    Empty,
    More(Box<Node>),
}


struct Node {
    val: i32,
    next: Link, //这里 Link 才是包裹节点和空枚举的那个目标枚举类型
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }   

    pub fn push(&mut self, elem: i32) {
        let new_node = Node {
            val: elem,
            next: std::mem::replace(&mut self.head, Link::Empty),
        };
        self.head = Link::More(Box::new(new_node));
    }

    pub fn pop(&mut self) -> Option<i32> {
        match std::mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(val) => {
                self.head = val.next;
                Some(val.val)
            }
        }
    }
}

#[cfg(test)]
mod test {

    use super::List;

    #[test]
    fn basic() {
        let mut list = List::new();

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));


        list.push(4);
        list.push(5);

        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        assert_eq!(list.pop(), Some(1));

        assert_eq!(list.pop(), None);

    }
}

