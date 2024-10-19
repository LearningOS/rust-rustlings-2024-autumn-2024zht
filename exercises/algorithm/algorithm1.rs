/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/

use std::collections::LinkedList;

// 定义一个新的类型 MyLinkedList，它包装了标准库中的 LinkedList
pub struct MyLinkedList<T>(LinkedList<T>);

impl<T> MyLinkedList<T> {
    // 创建一个新的 MyLinkedList 实例
    pub fn new() -> Self {
        MyLinkedList(LinkedList::new())
    }

    // 将元素添加到链表的末尾
    pub fn push_back(&mut self, val: T) {
        self.0.push_back(val);
    }
}

// 为 MyLinkedList 实现 merge 函数
impl<T> MyLinkedList<T>
where
    T: PartialOrd + Clone,
{
    pub fn merge(mut list_a: MyLinkedList<T>, mut list_b: MyLinkedList<T>) -> Self {
        let mut merged_list = MyLinkedList::new();
        let mut a_iter = list_a.0.into_iter();
        let mut b_iter = list_b.0.into_iter();

        let mut a_next = a_iter.next();
        let mut b_next = b_iter.next();

        loop {
            match (a_next.take(), b_next.take()) {
                (Some(a), Some(b)) => {
                    if a <= b {
                        merged_list.push_back(a);
                        a_next = a_iter.next();
                        b_next = Some(b);
                    } else {
                        merged_list.push_back(b);
                        b_next = b_iter.next();
                        a_next = Some(a);
                    }
                }
                (Some(a), None) => {
                    merged_list.push_back(a);
                    a_next = a_iter.next();
                }
                (None, Some(b)) => {
                    merged_list.push_back(b);
                    b_next = b_iter.next();
                }
                (None, None) => break,
            }
        }

        merged_list
    }
}

// 测试代码
fn main() {
    let mut list_a = MyLinkedList::new();
    list_a.push_back(1);
    list_a.push_back(3);
    list_a.push_back(5);

    let mut list_b = MyLinkedList::new();
    list_b.push_back(2);
    list_b.push_back(4);
    list_b.push_back(6);

    let merged_list = MyLinkedList::merge(list_a, list_b);

    for val in merged_list.0 {
        println!("{}", val);
    }
}