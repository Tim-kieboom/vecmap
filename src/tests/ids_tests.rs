use crate::ids::*;
use crate::vec_map_index::VecMapIndex;

crate::impl_ids!(TestNodeId, TestBlockId);

#[test]
fn generated_id_has_correct_error_and_begin() {
    assert_eq!(TestNodeId::ERROR, TestNodeId::error());
    assert_eq!(TestNodeId::ERROR.index(), 0);

    assert_eq!(TestNodeId::begin(), TestNodeId(1));
    assert_eq!(TestNodeId::begin().index(), 1);
}

#[test]
fn alloc_returns_current_then_advances() {
    let mut id = TestNodeId::begin();

    assert_eq!(id.alloc(), TestNodeId(1));
    assert_eq!(id.alloc(), TestNodeId(2));
    assert_eq!(id.alloc(), TestNodeId(3));

    assert_eq!(id.last(), TestNodeId(3));
    assert_eq!(id.index(), 4);
}

#[test]
fn alloc_does_not_reuse_ids() {
    let mut generator = TestNodeId::begin();

    let first = generator.alloc();
    let second = generator.alloc();
    let third = generator.alloc();

    assert_ne!(first, second);
    assert_ne!(second, third);
    assert_ne!(first, third);

    assert_eq!([first.index(), second.index(), third.index()], [1, 2, 3]);
}

#[test]
fn id_generator_starts_at_begin() {
    let generator: IdGenerator<TestNodeId> = IdGenerator::new();

    assert_eq!(generator.last(), TestNodeId::begin());
}

#[test]
fn id_generator_default_starts_at_begin() {
    let generator = IdGenerator::<TestNodeId>::default();

    assert_eq!(generator.last(), TestNodeId::begin());
}

#[test]
fn id_generator_allocates_monotonically() {
    let mut generator = IdGenerator::<TestNodeId>::new();

    assert_eq!(generator.alloc(), TestNodeId(1));
    assert_eq!(generator.alloc(), TestNodeId(2));
    assert_eq!(generator.alloc(), TestNodeId(3));

    assert_eq!(generator.last(), TestNodeId(3));
}

#[test]
fn id_generator_is_cloneable_and_independent() {
    let mut original = IdGenerator::<TestNodeId>::new();
    original.alloc();

    let mut clone = original.clone();

    assert_eq!(original.last(), clone.last());

    let from_original = original.alloc();
    let from_clone = clone.alloc();

    assert_eq!(from_original, from_clone);
    assert_eq!(original.last(), clone.last());
}

#[test]
fn generated_ids_support_ordering_and_equality() {
    assert!(TestNodeId(1) < TestNodeId(2));
    assert!(TestNodeId(2) > TestNodeId(1));
    assert!(TestNodeId(2) >= TestNodeId(2));
    assert!(TestNodeId(2) <= TestNodeId(2));

    assert_eq!(TestNodeId(7), TestNodeId(7));
    assert_ne!(TestNodeId(7), TestNodeId(8));
}

#[test]
fn generated_ids_can_be_hash_keys() {
    use std::collections::HashSet;

    let mut ids = HashSet::new();
    ids.insert(TestNodeId(1));
    ids.insert(TestNodeId(1));
    ids.insert(TestNodeId(2));

    assert_eq!(ids.len(), 2);
    assert!(ids.contains(&TestNodeId(1)));
    assert!(ids.contains(&TestNodeId(2)));
}

#[test]
fn generated_ids_round_trip_through_json() {
    let id = TestNodeId(42);

    let json = serde_json::to_string(&id).unwrap();
    let restored: TestNodeId = serde_json::from_str(&json).unwrap();

    assert_eq!(id, restored);
    assert_eq!(restored.index(), 42);
}

#[test]
fn id_generator_round_trips_through_json() {
    let mut generator = IdGenerator::<TestNodeId>::new();
    generator.alloc();
    generator.alloc();

    let json = serde_json::to_string(&generator).unwrap();
    let restored: IdGenerator<TestNodeId> = serde_json::from_str(&json).unwrap();

    assert_eq!(generator.last(), restored.last());
    assert_eq!(restored.last(), TestNodeId(2));
}

#[test]
fn macro_supports_multiple_ids_and_attributes() {
    crate::impl_ids!(
        /// Documentation should be accepted.
        #[allow(dead_code)]
        AttributedId,
        PlainId,
    );

    let mut attributed = IdGenerator::<AttributedId>::new();
    let mut plain = IdGenerator::<PlainId>::new();

    assert_eq!(attributed.alloc(), AttributedId(1));
    assert_eq!(plain.alloc(), PlainId(1));

    assert_eq!(attributed.last().0, plain.last().0);
}
