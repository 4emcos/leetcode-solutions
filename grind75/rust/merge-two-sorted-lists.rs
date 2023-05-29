#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
fn main() {
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut result = Some(Box::new(ListNode::new(0)));
        let mut current = &mut result;

        while list1.is_some() && list2.is_some() {
            let val1 = list1.as_ref().unwrap().val;
            let val2 = list2.as_ref().unwrap().val;

            if val1 < val2 {
                let next = list1.as_mut().unwrap().next.take();
                current.as_mut().unwrap().next = list1;
                list1 = next;
            } else {
                let next = list2.as_mut().unwrap().next.take();
                current.as_mut().unwrap().next = list2;
                list2 = next;
            }

            current = &mut current.as_mut().unwrap().next;
        }
        if list1.is_some() {
            current.as_mut().unwrap().next = list1;
        } else {
            current.as_mut().unwrap().next = list2;
        }

        return result.unwrap().next;
    }

    let list1 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode { val: 2, next: None })),
    }));

    let list2 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode { val: 3, next: None })),
    }));

    let result = merge_two_lists(list1, list2);

    println!("{:?}", result);
}
