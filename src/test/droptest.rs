use crate::cell::Cell;
use crate::rc::Rc;

const ALIVE: bool = true;
const DROPPED: bool = !ALIVE;

pub struct DropTestIndicator {
    cell: Rc<Cell<bool>>,
}

impl DropTestIndicator {
    pub fn is_alive(&self) -> bool {
        self.cell.get() == ALIVE
    }
}
pub struct DropTest {
    indicator: Rc<Cell<bool>>,
}

impl DropTest {
    pub fn new() -> (DropTestIndicator, DropTest) {
        let cell = Rc::new(Cell::new(ALIVE));
        let indicator = DropTestIndicator { cell: cell.clone() };
        let droptest = Self { indicator: cell };
        (indicator, droptest)
    }
}

impl Drop for DropTest {
    fn drop(&mut self) {
        self.indicator.set(DROPPED);
    }
}
