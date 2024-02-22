fn main() {}
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
struct Solution;
impl Solution {
    pub fn construct_from_pre_post(
        preorder: Vec<i32>,
        postorder: Vec<i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        //[root,left,right] //left root   left[0]
        //[left,right,root]
        let mut cache = HashMap::new();
        for (i, v) in postorder.iter().enumerate() {
            cache.insert(*v, i);
        }

        fn dsf(
            pre_order: &Vec<i32>,
            post_order: &Vec<i32>,
            pre_left: usize,
            pre_right: usize,
            post_left: usize,
            post_right: usize,
            cache: &HashMap<i32, usize>,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if pre_left > pre_right || post_left > post_right {
                return None;
            }
            //root val
            let root_val = pre_order[pre_left];
            if pre_left == pre_right {
                return Some(Rc::new(RefCell::new(TreeNode {
                    val: root_val,
                    left: None,
                    right: None,
                })));
            }
            let left_root = pre_order[pre_left + 1];
            //找到left_root在post_order中的位置,这个位置可以区分
            let pos = cache.get(&left_root).unwrap();
            let left_diff = pos - post_left;

            Some(Rc::new(RefCell::new(TreeNode {
                val: root_val,
                left: dsf(
                    pre_order,
                    post_order,
                    pre_left + 1,
                    pre_left + 1 + left_diff,
                    post_left,
                    *pos,
                    cache,
                ),
                right: dsf(
                    pre_order,
                    post_order,
                    pre_left + 1 + left_diff + 1,
                    pre_right,
                    *pos + 1,
                    post_right - 1,
                    cache,
                ),
            })))
        }

        dsf(
            &preorder,
            &postorder,
            0,
            preorder.len() - 1,
            0,
            postorder.len() - 1,
            &cache,
        )
    }
}
