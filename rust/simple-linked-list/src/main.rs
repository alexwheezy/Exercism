use simple_linked_list::SimpleLinkedList;

fn main() {
    let mut list: SimpleLinkedList<i32> = SimpleLinkedList::new();
    assert_eq!(list.len(), 0);
    list.push(123);
    assert_eq!(list.len(), 1);
    assert_eq!(list.peek(), Some(&123));

    list.push(456);
    assert_eq!(list.len(), 2);
    assert_eq!(list.peek(), Some(&456));

    assert_eq!(list.pop(), Some(456));
}
