/// The documentation of `AHash::default()` states that it will generate identical initial states,
/// but that's only for one run, and the states may differ in multiple runs.
/// While the `AHashBuilder` here will generate identical initial states in multiple runs,
/// so that `HashSet`/`HashMap` using it will have the same iteration order.
#[derive(Copy, Clone, Eq, PartialEq, Default)]
pub struct AHashBuilder;

impl std::hash::BuildHasher for AHashBuilder {
  type Hasher = ahash::AHasher;

  fn build_hasher(&self) -> Self::Hasher {
    // copied from ahash::random_state::PI, which is private
    ahash::RandomState::with_seeds(0x243f_6a88_85a3_08d3, 0x1319_8a2e_0370_7344, 0xa409_3822_299f_31d0, 0x082e_fa98_ec4e_6c89).build_hasher()
  }
}

/// The deterministic `HashMap` type.
pub type HashMap<K, V> = std::collections::HashMap<K, V, AHashBuilder>;
/// The deterministic `HashSet` type.
pub type HashSet<K> = std::collections::HashSet<K, AHashBuilder>;
