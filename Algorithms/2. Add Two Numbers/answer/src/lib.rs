// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let l1_value = get_value(l1);
    let l2_value = get_value(l2);

    return None;
}

fn get_value(l1: Option<Box<ListNode>>) -> i32 {
    let mut value = String::new();
    let mut current_node = l1;

    loop {
        match current_node.unwrap().next {
            Some(next) => {
                value.push_str(&next.val.to_string());
                current_node = Some(next);
            }
            None => break
        }
    }

    // value = value.chars().rev().collect();
    value.parse().unwrap()
}

fn build_value(v1: i32, v2: i32) -> Option<Box<ListNode>> {
    let result = (v1 + v2).to_string();

    let nodes: Vec<Option<Box<ListNode>>> = result.chars().into_iter().map(
        |x| Some(
            Box::new(
                ListNode::new(
                    x.to_string().parse() as i32
                )
            )
        )
    ).collect();

    for (i, node) in nodes.iter().enumerate() {
        if i == (nodes.len() - 1) {
            break;
        }
        node.unwrap().next = node[i + 1];
    }

    None
}

fn teste() {
    let mut n1 = ListNode::new(45);
    let mut n2 = ListNode::new(45);
    let mut n3 = ListNode::new(45);
    let mut n4 = ListNode::new(45);
    let mut n5 = ListNode::new(45);

    n1.next = Some(Box::new(n2.clone()));
    n2.next = Some(Box::new(n3.clone()));
    n3.next = Some(Box::new(n4.clone()));
    n4.next = Some(Box::new(n5.clone()));

    println!("{:?}", n1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}