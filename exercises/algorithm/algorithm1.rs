/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/
// I AM NOT DONE

impl<T> LinkedList<T>
where
    T: PartialOrd + Display,
{
    pub fn merge(mut list_a: LinkedList<T>, mut list_b: LinkedList<T>) -> Self {
        let mut merged_list = LinkedList::new();
        let mut a_ptr = list_a.start;
        let mut b_ptr = list_b.start;

        while let (Some(a), Some(b)) = (a_ptr, b_ptr) {
            let a_node = unsafe { a.as_ref() };
            let b_node = unsafe { b.as_ref() };

            if a_node.val <= b_node.val {
                merged_list.add(a_node.val.clone());
                a_ptr = a_node.next;
            } else {
                merged_list.add(b_node.val.clone());
                b_ptr = b_node.next;
            }
        }

        // Add remaining elements from list_a
        while let Some(a) = a_ptr {
            let a_node = unsafe { a.as_ref() };
            merged_list.add(a_node.val.clone());
            a_ptr = a_node.next;
        }

        // Add remaining elements from list_b
        while let Some(b) = b_ptr {
            let b_node = unsafe { b.as_ref() };
            merged_list.add(b_node.val.clone());
            b_ptr = b_node.next;
        }

        merged_list
    }
}