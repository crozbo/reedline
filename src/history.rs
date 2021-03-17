use std::collections::VecDeque;

const HISTORY_SIZE: usize = 100;

pub struct History {
    que: VecDeque<String>,
    cursor: Option<usize>,
}

impl History {
    pub fn new() -> History {
        History {
            que: VecDeque::with_capacity(HISTORY_SIZE),
            cursor: None,
        }
    }

    pub fn append(&mut self, s: String) {
        // empty lines are not appended to history
        if !s.is_empty() {
            if self.que.len() == HISTORY_SIZE {
                // History is "full", so we delete the oldest entry first,
                // before adding a new one.
                self.que.pop_back();
            }
            self.que.push_front(s);
            // reset the history cursor
            self.cursor = None;
        }
    }

    pub fn previous(&mut self) -> Option<String> {
        match self.cursor {
            Some(c) => {
                if c < self.len() - 1 {
                    self.cursor = Some(c + 1);
                }
                Some(String::from(self.que.get(self.cursor.unwrap()).unwrap()))
            }
            None => {
                if !self.is_empty() {
                    self.cursor = Some(0);
                    Some(String::from(self.que.get(0).unwrap()))
                } else {
                    None
                }
            }
        }
    }

    pub fn next(&mut self) -> Option<String> {
        match self.cursor {
            Some(c) => {
                if c == 0 {
                    self.cursor = None;
                    Some(String::new())
                } else {
                    self.cursor = Some(c - 1);
                    Some(String::from(self.que.get(self.cursor.unwrap()).unwrap()))
                }
            }
            None => None,
        }
    }

    pub fn len(&self) -> usize {
        self.que.len()
    }

    pub fn is_empty(&self) -> bool {
        self.que.is_empty()
    }
}
