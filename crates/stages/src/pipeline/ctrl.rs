use reth_primitives::{BlockNumber, SealedHeader};

/// Determines the control flow during pipeline execution.
#[derive(Debug, Eq, PartialEq)]
pub enum ControlFlow {
    /// An unwind was requested and must be performed before continuing.
    Unwind {
        /// The block to unwind to.
        target: BlockNumber,
        /// The block that caused the unwind.
        bad_block: SealedHeader,
    },
    /// The pipeline is allowed to continue executing stages.
    Continue {
        /// Block number reached by the stage.
        block_number: BlockNumber,
    },
    /// Pipeline made no progress
    NoProgress {
        /// Block number reached by the stage.
        block_number: Option<BlockNumber>,
    },
}

impl ControlFlow {
    /// Whether the pipeline should continue executing stages.
    pub fn should_continue(&self) -> bool {
        matches!(self, ControlFlow::Continue { .. } | ControlFlow::NoProgress { .. })
    }

    /// Returns true if the control flow is unwind.
    pub fn is_unwind(&self) -> bool {
        matches!(self, ControlFlow::Unwind { .. })
    }

    /// Returns the pipeline progress, if the state is not `Unwind`.
    pub fn progress(&self) -> Option<BlockNumber> {
        match self {
            ControlFlow::Unwind { .. } => None,
            ControlFlow::Continue { block_number } => Some(*block_number),
            ControlFlow::NoProgress { block_number } => *block_number,
        }
    }
}
