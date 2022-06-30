#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

#[allow(dead_code)]
pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    add_two_numbers_recurse(l1, l2, 0)
}

fn add_two_numbers_recurse(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
    carry: i32,
) -> Option<Box<ListNode>> {
    match (l1, l2) {
        (Some(x1), Some(x2)) => {
            let sum = x1.val + x2.val;
            Some(Box::new(ListNode {
                next: add_two_numbers_recurse(x1.next, x2.next, ((sum + carry) >= 10) as i32),
                val: (sum + carry) % 10,
            }))
        }
        (Some(x1), None) => Some(Box::new(ListNode {
            next: add_two_numbers_recurse(x1.next, None, ((x1.val + carry) >= 10) as i32),
            val: (x1.val + carry) % 10,
        })),
        (None, Some(x2)) => Some(Box::new(ListNode {
            next: add_two_numbers_recurse(None, x2.next, ((x2.val + carry) >= 10) as i32),
            val: (x2.val + carry) % 10,
        })),
        (None, None) if carry == 1 => Some(Box::new(ListNode {
            next: None,
            val: carry,
        })),
        (None, None) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_two_numbers() {
        let l1 = Some(Box::new(ListNode {
            next: Some(Box::new(ListNode {
                next: Some(Box::new(ListNode { next: None, val: 3 })),
                val: 4,
            })),
            val: 2,
        }));
        let l2 = Some(Box::new(ListNode {
            next: Some(Box::new(ListNode {
                next: Some(Box::new(ListNode { next: None, val: 4 })),
                val: 6,
            })),
            val: 5,
        }));
        let a = Some(Box::new(ListNode {
            next: Some(Box::new(ListNode {
                next: Some(Box::new(ListNode { next: None, val: 8 })),
                val: 0,
            })),
            val: 7,
        }));
        assert_eq!(a, add_two_numbers(l1, l2));

        let l1 = Some(Box::new(ListNode {
            next: Some(Box::new(ListNode {
                next: Some(Box::new(ListNode { next: None, val: 5 })),
                val: 4,
            })),
            val: 2,
        }));
        let l2 = Some(Box::new(ListNode {
            next: Some(Box::new(ListNode {
                next: Some(Box::new(ListNode { next: None, val: 4 })),
                val: 6,
            })),
            val: 5,
        }));
        let a = Some(Box::new(ListNode {
            next: Some(Box::new(ListNode {
                next: Some(Box::new(ListNode {
                    next: Some(Box::new(ListNode { next: None, val: 1 })),
                    val: 0,
                })),
                val: 0,
            })),
            val: 7,
        }));
        assert_eq!(a, add_two_numbers(l1, l2));

        let l1 = Some(Box::new(ListNode {
            next: Some(Box::new(ListNode {
                next: Some(Box::new(ListNode {
                    next: Some(Box::new(ListNode {
                        next: Some(Box::new(ListNode {
                            next: Some(Box::new(ListNode {
                                next: Some(Box::new(ListNode { next: None, val: 9 })),
                                val: 9,
                            })),
                            val: 9,
                        })),
                        val: 9,
                    })),
                    val: 9,
                })),
                val: 9,
            })),
            val: 9,
        }));
        let l2 = Some(Box::new(ListNode {
            next: Some(Box::new(ListNode {
                next: Some(Box::new(ListNode {
                    next: Some(Box::new(ListNode { next: None, val: 9 })),
                    val: 9,
                })),
                val: 9,
            })),
            val: 9,
        }));
        let a = Some(Box::new(ListNode {
            next: Some(Box::new(ListNode {
                next: Some(Box::new(ListNode {
                    next: Some(Box::new(ListNode {
                        next: Some(Box::new(ListNode {
                            next: Some(Box::new(ListNode {
                                next: Some(Box::new(ListNode {
                                    next: Some(Box::new(ListNode { next: None, val: 1 })),
                                    val: 0,
                                })),
                                val: 0,
                            })),
                            val: 0,
                        })),
                        val: 9,
                    })),
                    val: 9,
                })),
                val: 9,
            })),
            val: 8,
        }));
        assert_eq!(a, add_two_numbers(l1, l2));

        let l1 = Some(Box::new(ListNode { next: None, val: 0 }));
        assert_eq!(l1, add_two_numbers(l1.clone(), l1.clone()));
    }
}
