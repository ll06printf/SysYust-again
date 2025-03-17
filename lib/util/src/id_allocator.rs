mod sysy_util {
    
    /// Rust不允许使用不在结构体内使用的类型作为范型参数
    /// 所以没办法打类型标签
    #[derive(PartialEq, Eq, Hash, Debug)]
    pub struct Id (u64);

    pub trait IdAllocator {
        fn new_id(&mut self) -> Id;
        fn drop_id(&mut self, e: Id);
    }

    /// 一直分配新Id的的Id分配器
    pub struct DefaultIdAllocator {
        next_number: u64,
    }

    impl DefaultIdAllocator {
        fn new() -> Self {
            Self{next_number: 0}
        }
    }

    impl IdAllocator for DefaultIdAllocator {
        fn new_id(&mut self) -> Id {
            let to_return = self.next_number;
            self.next_number += 1;
            Id(to_return)
        }
        fn drop_id(&mut self, _: Id) {
            // do nothing
        }
    }


    #[cfg(test)]
    mod tests {
        use std::collections::HashSet;

        use super::*;
        #[test]
        fn id_equal() {
            let id1 = Id(1);
            let id2 = Id(1);
            let id3 = Id(3);
            let id4 = Id(3);
            assert_eq!(id1, id2);
            assert_ne!(id1, id3);
            assert_eq!(id3, id4);
        }

        #[test]
        fn id_allocator_never_repeate() {
            let mut s = HashSet::new();
            let mut ida = DefaultIdAllocator::new();

            const REPEATE_TIME : usize = 1000;
            for _ in 0..REPEATE_TIME {
                s.insert(ida.new_id());
            }
            assert_eq!(REPEATE_TIME, s.len());
            s.into_iter().for_each(|e| ida.drop_id(e));
        }
        
    }
}