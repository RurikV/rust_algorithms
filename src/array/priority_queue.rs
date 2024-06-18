use super::double_linked_list::DoubleLinkedList;

struct PriorityRec<T> {
    priority: i32,
    recs: DoubleLinkedList<T>,
}

pub struct PriorityQueue<T> {
    data: Vec<PriorityRec<T>>,
}

impl<T> PriorityQueue<T>
where
    T: Clone + Default,
{
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn enqueue(&mut self, priority: i32, item: T) {
        for i in 0..self.data.len() {
            if self.data[i].priority == priority {
                self.data[i].recs.add(item);
                return;
            }
            if self.data[i].priority < priority {
                let mut new_rec = PriorityRec {
                    priority,
                    recs: DoubleLinkedList::new(),
                };
                new_rec.recs.add(item);
                self.data.insert(i, new_rec);
                return;
            }
        }
        let mut new_rec = PriorityRec {
            priority,
            recs: DoubleLinkedList::new(),
        };
        new_rec.recs.add(item);
        self.data.push(new_rec);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.data.is_empty() {
            None
        } else {
            let rec = self.data[0].recs.remove(0);
            if self.data[0].recs.size() == 0 {
                self.data.remove(0);
            }
            Some(rec)
        }
    }
}
