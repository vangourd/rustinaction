enum Suit {
    Clubs,
    Spades,
    Diamonds,
    Hearts, // #1
}

enum Card{
    King(Suit), // #2
    Queen(Suit),
    Jack(Suit),
    Ace(Suit),
    Pip(Suit, usize), // #3
}
