

pub trait StateMachine {
    type State;
    type Transition;
    fn next_state(starting_state: &Self::State, t: &Self::Transition) -> Self::State;
}
use std::collections::hash_map::DefaultHasher;
pub fn hash<T: std::hash::Hash>(t: &[T]) -> u64 {
    let mut hasher = DefaultHasher::new();
    for key in t {
        std::hash::Hash::hash(&key, &mut hasher);
    }
    std::hash::Hasher::finish(&hasher)
}
#[test]
fn test_hash_enum_vec() {
    #[derive(Hash)]
  
    enum KeyTest{
        One,
        Two,
        Three,
        Four

    }
    impl std::fmt::Display for KeyTest {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                KeyTest::One => write!(f, "One"),
                KeyTest::Two => write!(f, "Two"),
                KeyTest::Three => write!(f, "Three"),
                KeyTest::Four => write!(f, "Four"),
            }
        }
    }
    
    let mut input: Vec<KeyTest> = vec![KeyTest::One, KeyTest::Two, KeyTest::Three, KeyTest::Four];

    let hash1 = hash(&input);
    let hash2 = hash(&input);

    assert_eq!(hash1, hash2);
}
