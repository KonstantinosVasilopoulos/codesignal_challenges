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

fn solution(l: ListNode<i32>) -> bool {
    // Create a stack and append the nodes' values
    let mut stack: Vec<i32> = Vec::new();
    let mut node: &ListNode<i32> = &l;
    while node.is_some() {
        stack.push((*node).as_ref().unwrap().value);
        node = &node.as_ref().unwrap().next;
    }

    // Pop the items of the stack and compare them with the list's nodes
    let mut item = stack.pop();
    let mut node: &ListNode<i32> = &l;
    while item.is_some() {
        // Compare the values
        if item.unwrap() != node.as_ref().unwrap().value {
            return false;
        }

        // Get the next pair
        item = stack.pop();
        node = &node.as_ref().unwrap().next;
    }

    true
}

// Test the solution
fn main() {
    let mut l: ListNode<i32> = generate_list(vec![0, 1, 0]);
    assert!(solution(l));
    l = generate_list(vec![1, 2, 2, 3]);
    assert!(!solution(l));
}

// Generate a linked list from a vector
fn generate_list<T: Clone>(a: Vec<T>) -> ListNode<T> {
    if a.is_empty() {
        return None;
    }

    let mut head: ListNode<T> = Some(Box::new(List::new(a[0].clone())));
    let mut node: &mut ListNode<T> = &mut head;
    for i in 1..a.len() {
        // Add new node
        node.as_mut().unwrap().next = Some(Box::new(List::new(a[i].clone())));
        node = &mut node.as_mut().unwrap().next;
    }

    head
}

// Print a linked list
#[allow(dead_code)]
fn print_list<T: std::fmt::Debug>(l: &ListNode<T>) {
    // Iterate over the list and print each node's value
    let mut node: &ListNode<T> = l;
    while node.is_some() {
        print!("{:?} ", node.as_ref().unwrap().value);
        node = &(*node).as_ref().unwrap().next;
    }

    println!();
}
