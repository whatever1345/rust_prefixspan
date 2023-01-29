pub(crate) struct PseudoSequence {
    pub(crate) seq_id: usize,
    pub(crate) seq_index: usize,
}

impl PseudoSequence {
    pub(crate) fn new(id: usize, index: usize) -> PseudoSequence {
        PseudoSequence { seq_id: id, seq_index: index}
    }
}