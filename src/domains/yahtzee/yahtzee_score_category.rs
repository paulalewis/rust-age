use enum_ordinalize::Ordinalize;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Ordinalize)]
pub enum YahtzeeScoreCategory {
    Ones,
    Twos,
    Threes,
    Fours,
    Fives,
    Sixes,
    ThreeOfKind,
    FourOfKind,
    FullHouse,
    SmallStraight,
    LargeStraight,
    Yahtzee,
    Chance,
}
