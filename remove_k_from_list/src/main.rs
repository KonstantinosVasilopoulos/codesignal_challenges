// Singly-linked lists are already defined with this interface:
struct List<T> {
    value: T,
    next: Option<Box<List<T>>>
}

impl<T> List<T> {
    fn new(v : T) -> Self {
        List { value: v, next: None }
    }
}

type ListNode<T> = Option<Box<List<T>>>;

fn solution(mut l: ListNode<i32>, k: i32) -> ListNode<i32> {
    // Iterate over the list
    let mut node = &mut l;
    while node.is_some() {
        if node.as_ref().unwrap().value == k {
            
        }
    }

    l
}
