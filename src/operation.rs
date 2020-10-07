use crate::clipboard::clipboard::Clipboard;
use crate::selection::selection::Selection;

pub trait Operation {
    fn perform(&self, clipboard: Option<&mut Clipboard>, selection: Option<&mut Selection>);
    fn undo(&self, clipboard: Option<&mut Clipboard>, selection: Option<&mut Selection>);
}