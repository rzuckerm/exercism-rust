use std::ptr::null_mut;

// this module adds some functionality based on the required implementations
// here like: `LinkedList::pop_back` or `Clone for LinkedList<T>`
// You are free to use anything in it, but it's mainly for the test framework.
mod pre_implemented;

pub struct LinkedList<T> {
    head: *mut Node<T>,
    tail: *mut Node<T>,
    size: usize,
}

pub struct Node<T> {
    element: T,
    next: *mut Node<T>,
    prev: *mut Node<T>,
}

impl<T> Node<T> {
    pub fn new(element: T) -> Self {
        Self {
            element,
            next: null_mut(),
            prev: null_mut(),
        }
    }
}

pub struct Cursor<'a, T> {
    list: &'a mut LinkedList<T>,
    ptr: *mut Node<T>,
}

pub struct Iter<'a, T>(Option<&'a Node<T>>);

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: null_mut(),
            tail: null_mut(),
            size: 0,
        }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for LinkedList)
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn len(&self) -> usize {
        self.size
    }

    /// Return a cursor positioned on the front element
    pub fn cursor_front(&mut self) -> Cursor<'_, T> {
        Cursor {
            ptr: self.head,
            list: self,
        }
    }

    /// Return a cursor positioned on the back element
    pub fn cursor_back(&mut self) -> Cursor<'_, T> {
        Cursor {
            ptr: self.tail,
            list: self,
        }
    }

    /// Return an iterator that moves from front to back
    pub fn iter(&self) -> Iter<'_, T> {
        unsafe { Iter(self.head.as_ref()) }
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

// the cursor is expected to act as if it is at the position of an element
// and it also has to work with and be able to insert into an empty list.
impl<T> Cursor<'_, T> {
    /// Take a mutable reference to the current element
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        unsafe { Some(&mut self.ptr.as_mut()?.element) }
    }

    /// Move one position forward (towards the back) and
    /// return a reference to the new position
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<&mut T> {
        self.ptr = unsafe { self.ptr.as_mut() }?.next;
        self.peek_mut()
    }

    /// Move one position backward (towards the front) and
    /// return a reference to the new position
    pub fn prev(&mut self) -> Option<&mut T> {
        self.ptr = unsafe { self.ptr.as_mut() }?.prev;
        self.peek_mut()
    }

    /// Remove and return the element at the current position and move the cursor
    /// to the neighboring element that's closest to the back. This can be
    /// either the next or previous position.
    pub fn take(&mut self) -> Option<T> {
        if self.ptr.is_null() {
            return None;
        }

        let ptr = self.ptr;
        self.list.size -= 1;

        unsafe {
            // Move head and ptr to next element if ptr is at head
            if ptr == self.list.head {
                self.list.head = (*ptr).next;
                self.ptr = self.list.head;
            } else {
                // Else, connect previous item of ptr to next item of ptr
                (*((*ptr).prev)).next = (*ptr).next;
            }

            // Move tail and ptr to previous element if ptr is at tail
            if ptr == self.list.tail {
                self.list.tail = (*ptr).prev;
                self.ptr = self.list.tail;
            } else {
                // Else, connect next item of ptr to prev item of ptr and
                // move ptr to next item
                (*((*ptr).next)).prev = (*ptr).prev;
                self.ptr = (*ptr).next;
            }

            Some(Box::from_raw(ptr).element)
        }
    }

    pub fn insert_after(&mut self, element: T) {
        // Create new node
        let ptr = Box::into_raw(Box::new(Node::new(element)));

        // If list is empty, set head, tail, and ptr to new node
        if self.list.is_empty() {
            self.init_single_element(ptr);
        } else {
            // All operations only derefence a pointer that is known not to be null
            unsafe {
                // Move tail to new node if ptr at tail
                if self.ptr == self.list.tail {
                    self.list.tail = ptr;
                } else {
                    // Attach ptr previous node to new node
                    (*((*self.ptr).next)).prev = ptr;
                }

                // Insert new node into list
                (*ptr).prev = self.ptr;
                (*ptr).next = (*self.ptr).next;

                // Attach ptr next node to new node
                (*self.ptr).next = ptr;
            }
        }

        self.list.size += 1;
    }

    pub fn insert_before(&mut self, element: T) {
        // Create new node
        let ptr = Box::into_raw(Box::new(Node::new(element)));

        // If list is empty, set head, tail, and ptr to new node
        if self.list.is_empty() {
            self.init_single_element(ptr);
            //self.list.head = ptr;
            //self.list.tail = ptr;
            //self.ptr = ptr;
        } else {
            // All operations only derefence a pointer that is known not to be null
            unsafe {
                // Move head to new node if ptr at head
                if self.ptr == self.list.head {
                    self.list.head = ptr;
                } else {
                    // Attach ptr next node to new node
                    (*((*self.ptr).prev)).next = ptr;
                }

                // Insert new node into list
                (*ptr).prev = (*self.ptr).prev;
                (*ptr).next = self.ptr;

                // Attach ptr previous node to new node
                (*self.ptr).prev = ptr;
            }
        }

        self.list.size += 1;
    }

    fn init_single_element(&mut self, ptr: *mut Node<T>) {
        self.list.head = ptr;
        self.list.tail = ptr;
        self.ptr = ptr;
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        let element = &self.0?.element;
        unsafe { self.0 = self.0.unwrap().next.as_ref() }
        Some(element)
    }
}
