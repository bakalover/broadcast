use std::{marker::PhantomData, ops::Index};

use loom::sync::{atomic::{AtomicPtr, AtomicUsize}, Arc};

const K_CELLS: usize = 61;

pub(crate) struct InfiniteArray<T: Clone + Send> { // Inject into each Sender/Reciever (Actually a view aka distributed array)
    s_index: Arc<AtomicPtr<usize>>, // Shared between all infinite array views inside Senders/Recievers
    segment_s: Segment<T>,
}

impl< T: Clone + Send> InfiniteArray<T> {
    fn locate_cell(&self, index: usize) -> Cell<T> {
        let segm = self.find_and_move_forward_send(self.segment_s, index / K_CELLS);
    }

    fn find_and_move_forward_send(&self, segm: Segment<T>, required_segm_id: usize) -> Segment<T>{

    }
    // segm := findAndMoveForwardSend ( segm , s / K)
    // 5 if segm . id != s / K: // the cell is INTERRUPTED
    // 6 CAS (&S , s , segm . id * K) // skip INTERRUPTED cells
    // 7 continue
    // 8 // The rest is same as before , but manipulates
    // 9 // the ‘segm.cells[i]‘ cell instead , invok
}

impl<'a, T: Clone + Send> Index<usize> for InfiniteArray<'a, T> {
    type Output = Cell<T>;

    fn index(&self, index: usize) -> &Self::Output {
        self.locate_cell(index)
    }
}

pub(crate) struct Cell<T: Clone + Send> {
    marker: PhantomData<T>,
    marker
}

pub(crate) struct Segment<T: Clone> {
    id: usize,
    cells: [Cell<T>; K_CELLS],
    next: Option<AtomicPtr<Segment<T>>>,
    prev: Option<AtomicPtr<Segment<T>>>,
}
