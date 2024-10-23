use std::{cell::RefCell, rc::Rc};
use std::collections::VecDeque;
type TreeNodeRef = Rc<RefCell<TreeNode>>;

#[derive(Debug, Clone)]
pub struct TreeNode
{
    val: String,
    left: Option<TreeNodeRef>,
    right: Option<TreeNodeRef>,
}

impl TreeNode
{
    pub fn new(val: String) -> Self
    {
        Self
        {
            val,
            left: None,
            right: None,
        }
    }

    pub fn set_left(&mut self, left: TreeNodeRef)
    {
        self.left = Some(left);
    }

    pub fn set_right(&mut self, right: TreeNodeRef)
    {
        self.right = Some(right);
    }

    pub fn show(tree_node: &TreeNode)
    {
        println!("{:?}", tree_node);
    }

    pub fn show_tree_as_wid(root: TreeNodeRef) -> String
    {
        let mut queue = VecDeque::new();
        queue.push_back(root);
        let mut result = "".to_string();

        while let Some(current) = queue.pop_front() {
            let current = current.borrow();
            result.push_str(&current.val);
            result.push_str(", ");

            if let Some(ref left) = current.left {
                queue.push_back(left.clone());
            }

            if let Some(ref right) = current.right {
                queue.push_back(right.clone());
            }
        }

        result
    }
}

pub fn create_node(val: String) -> TreeNodeRef
{
    Rc::new(RefCell::new(TreeNode::new(val)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_node() {
        let root = create_node("root".to_string());
        let left_child = create_node("left".to_string());
        let right_child = create_node("right".to_string());

        root.borrow_mut().set_left(left_child.clone());
        root.borrow_mut().set_right(right_child.clone());

        TreeNode::show(&root.borrow());
        TreeNode::show(&left_child.borrow());
        TreeNode::show(&right_child.borrow());

        assert_eq!(root.borrow().val, "root");
        assert_eq!(root.borrow().left.as_ref().unwrap().borrow().val, "left");
        assert_eq!(root.borrow().right.as_ref().unwrap().borrow().val, "right");
    }

    #[test]
    fn test_show_tree_as_wid() {
        let root = create_node("root".to_string());
        let left_child = create_node("left".to_string());
        let right_child = create_node("right".to_string());
        let left_left_child = create_node("left_left".to_string());
        let left_right_child = create_node("left_right".to_string());

        root.borrow_mut().set_left(left_child.clone());
        root.borrow_mut().set_right(right_child.clone());
        left_child.borrow_mut().set_left(left_left_child.clone());
        left_child.borrow_mut().set_right(left_right_child.clone());

        let result = TreeNode::show_tree_as_wid(root.clone());
        assert_eq!(result, "root, left, right, left_left, left_right, ");
    }
}