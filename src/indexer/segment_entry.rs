use crate::common::BitSet;
use crate::core::SegmentId;
use crate::core::SegmentMeta;
use crate::indexer::delete_queue::DeleteCursor;
use std::fmt;
use crate::{Segment, Opstamp};

/// A segment entry describes the state of
/// a given segment, at a given instant.
///
/// In addition to segment `meta`,
/// it contains a few transient states
/// - `state` expresses whether the segment is already in the
/// middle of a merge
/// - `delete_bitset` is a bitset describing
/// documents that were deleted during the commit
/// itself.
/// - `delete_cursor` is the position in the delete queue.
/// Deletes happening before the cursor are reflected either
/// in the .del file or in the `delete_bitset`.
#[derive(Clone)]
pub struct SegmentEntry {
    segment: Segment,
    delete_bitset: Option<BitSet>,
    delete_cursor: DeleteCursor,
}

impl SegmentEntry {
    /// Create a new `SegmentEntry`
    pub fn new(
        segment: Segment,
        delete_cursor: DeleteCursor,
        delete_bitset: Option<BitSet>,
    ) -> SegmentEntry {
        SegmentEntry {
            segment,
            delete_bitset,
            delete_cursor,
        }
    }

    /// Return a reference to the segment entry deleted bitset.
    ///
    /// `DocId` in this bitset are flagged as deleted.
    pub fn delete_bitset(&self) -> Option<&BitSet> {
        self.delete_bitset.as_ref()
    }


    pub fn set_delete_cursor(&mut self, delete_cursor: DeleteCursor) {
        self.delete_cursor = delete_cursor;
    }

    /// `Takes` (as in Option::take) the delete bitset of a segment entry.
    /// `DocId` in this bitset are flagged as deleted.
    pub fn take_delete_bitset(&mut self) -> Option<BitSet> {
        self.delete_bitset.take()
    }

    /// Reset the delete information in this segment.
    ///
    /// The `SegmentEntry` segment's `SegmentMeta` gets updated, and
    /// any delete bitset is drop and set to None.
    pub fn reset_delete_meta(&mut self, num_deleted_docs: u32, target_opstamp: Opstamp) {
        self.segment = self
            .segment
            .clone()
            .with_delete_meta(num_deleted_docs, target_opstamp);
        self.delete_bitset = None;
    }

    /// Return a reference to the segment_entry's delete cursor
    pub fn delete_cursor(&mut self) -> DeleteCursor {
        self.delete_cursor.clone()
    }

    /// Returns the segment id.
    pub fn segment_id(&self) -> SegmentId {
        self.meta().id()
    }

    /// Returns the `segment` associated to the `SegmentEntry`.
    pub fn segment(&self) -> &Segment {
        &self.segment
    }
    /// Accessor to the `SegmentMeta`
    pub fn meta(&self) -> &SegmentMeta {
        self.segment.meta()
    }
}


impl fmt::Debug for SegmentEntry {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "SegmentEntry({:?})", self.meta())
    }
}
