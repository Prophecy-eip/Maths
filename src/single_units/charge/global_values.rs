pub mod charge {
    const THROWS_TO_HIT: Vec<(Range<usize>, u8)> =
        vec![(4..20, 2), (1..4, 3), (-3..1, 4), (-7..-3, 5), (-20..-7, 6)]; // here we use 20 as max value for range, we should change it later, same for -20

    const THROWS_TO_WOUND: Vec<(Range<usize>, u8)> =
        vec![(2..20, 2), (1..2, 3), (0..1, 4), (-1..0, 5), (-20..-2, 6)]; // here we use 20 as max value for range, we should change it later, same for -20

    const SIZE_DICE: usize = 6;
    const ARMOUR_SAVE_THRESHOLD: usize = 7;
}
