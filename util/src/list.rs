pub struct MultiList {
    //每个链表容量
    capacity: usize,
    //链表数量
    list_num: usize,

    data: Vec<(usize, usize)>,
}

impl MultiList {
    fn link(&mut self, prev: usize, next: usize) {
        //链接两节点
        self.data[prev].1 = next;
        self.data[next].0 = prev;
    }

    pub fn new(capacity: usize, list_num: usize) -> MultiList {
        let mut v: Vec<(usize, usize)> = vec![(0, 0); capacity + list_num];
        for (i, k) in v.iter_mut().enumerate() {
            *k = (i, i);
        }

        MultiList {
            capacity: capacity,
            list_num: list_num,
            data: v,
        }
    }

    pub fn erase(&mut self, idx: usize) {
        //删除节点idx
        debug_assert!(idx < self.capacity);
        if self.data[idx].0 != idx {
            //将该节点前后链接
            self.link(self.data[idx].0, self.data[idx].1);
            self.data[idx] = (idx, idx);
        }
    }

    pub fn insert(&mut self, list_idx: usize, elem: usize) {
        debug_assert!(list_idx < self.list_num && elem < self.capacity);
        //将elem插入到list_idx链表尾
        //自删除
        self.erase(elem);
        let list_head = list_idx + self.capacity;
        let prev = self.data[list_head].0;

        self.link(prev, elem);
        self.link(elem, list_head);
        /*
        初始: ...<- tail <- head <- ...
        现在: ...<- tail <- elem <- head <- ...
        */
    }

    pub fn insert_as_first(&mut self, list_idx: usize, elem: usize) {
        debug_assert!(list_idx < self.list_num && elem < self.capacity);
        //将elem插入到list_idx链表头之后的第一个
        self.erase(elem);
        let list_head = list_idx + self.capacity;
        let next = self.data[list_head].1;

        self.link(list_head, elem);
        self.link(elem, next);
        /*
        初始: ..<- tail <- head <- next...
        现在: ..<- tail <- head <- elem <- next ...
        */
    }

    pub fn get_first(&self, list_idx: usize) -> usize {
        debug_assert!(list_idx < self.list_num);
        self.data[list_idx + self.capacity].1
    }

    pub fn next(&self, idx: usize) -> usize {
        debug_assert!(idx < self.capacity + self.list_num);
        self.data[idx].1
    }

    pub fn is_head(&self, idx: usize) -> bool {
        debug_assert!(idx < self.capacity + self.list_num);
        idx >= self.capacity
    }

    pub fn is_alone(&self, idx: usize) -> bool {
        self.data[idx].1 == idx
    }
}

