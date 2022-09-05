use std::cell::RefCell;
use std::rc::Rc;
use rand::Rng;


#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }
}

fn check_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    if let Some(node) = root {
        if node.borrow().val == node.borrow().left.as_ref().unwrap().borrow().val + node.borrow().right.as_ref().unwrap().borrow().val {
            return true;
        }
    }
    false
}

fn main() {
    // let root = Some(Rc::new(RefCell::new(TreeNode::new(10))));
    //
    // root.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    // root.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(6))));
    // assert_eq!(true, check_tree(root))
    let mut list = vec![3, 2, 3, 1, 2, 4, 5, 5, 6];
    let new_list = quick_sort2(&mut list);
    println!("{:?}", new_list);
}

// 1, 1, 1, 2, 3
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let n = nums.len();
    if n == 0 {
        return 0;
    }
    let mut slow = 1;
    let mut fast = 1;
    while fast < n {
        if nums[fast] != nums[fast - 1] {
            nums[slow] = nums[fast];
            slow += 1;
        }
        fast += 1;
    }
    slow as i32
}

pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    let mut nums = nums;
    let size = nums.len();
    assert!(size > 0);
    return quick_select(&mut nums, 0, size as i32 - 1, size as i32 - k);
}

pub fn quick_select(nums: &mut Vec<i32>, l: i32, r: i32, index: i32) -> i32 {
    let q = random_partition(nums, l, r);
    return if q == index {
        nums[q as usize]
    } else {
        if q < index { quick_select(nums, q + 1, r, index) } else { quick_select(nums, l, q - 1, index) }
    };
}

pub fn random_partition(nums: &mut Vec<i32>, l: i32, r: i32) -> i32 {
    let i = rand::thread_rng().gen_range(0..(r - l + 1)) + 1;
    swap(nums, i, r);
    partition(nums, l, r)
}

pub fn partition(nums: &mut Vec<i32>, l: i32, r: i32) -> i32 {
    let x = nums[r as usize];
    let mut i = l - 1;
    for j in l..r {
        if nums[j as usize] <= x {
            i += 1;
            swap(nums, i, j);
        }
    }
    swap(nums, i + 1, r);
    i + 1
}

pub fn swap(nums: &mut Vec<i32>, l: i32, r: i32) {
    nums.swap(l as usize, r as usize)
}

pub fn find_kth_largest2(nums: Vec<i32>, k: i32) -> i32 {
    let mut nums = nums;
    let size = nums.len();
    assert!(size > 0);
    return *quick_sort2(&mut nums).get(size - k as usize).unwrap();
}


fn quick_sort2(list: &mut Vec<i32>) -> &mut Vec<i32> {
    if list.len() > 1 {
        let mut pivot_count = 1;
        let pivot = list[1];
        let (mut left, mut right) = (Vec::new(), Vec::new());
        for num in list.iter_mut() {
            if *num == pivot {
                // left.push(*num);
                pivot_count += 1;
                continue;
            } else if *num < pivot {
                left.push(*num);
            } else {
                right.push(*num);
            }
        }

        let (mut left, mut right) = (quick_sort2(&mut left), quick_sort2(&mut right));
        for _ in 1..pivot_count {
            left.push(pivot);
        }
        pivot_count = 1;
        left.append(&mut right);

        list.clear();
        list.append(&mut left);
    }
    list
}