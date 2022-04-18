# Poker-Hands-Rust-
This program deals two 2-card poker hands and a shared
five card pool according to the
rules of Texas Holdem poker. It determines the winning hand and return it.
When determining the strength of each player’s hand, the two cards that
player was dealt, as well as the five cards present in the shared pool are considered, with the stronger hand will
be returned as the winner.


The input will be the first 9 values in a permutation of the integers 1-52.
This represents a shuffling of a standard deck of cards, with each set of 13 cards representing a suit:  
  
[1..13] are Clubs  
[14..26] are Diamonds  
[27..39] are Hearts  
[40..52] are Spades.  
  
Thus, an input sequence that started with the integers [38, 48, 11, 6, ...] would represent
Queen of Hearts, 9 of Spades, Jack of Clubs, 6 of Clubs, and so on. 

This permutation as input and use it to deal two poker hands of two
cards each in an alternating fashion. I.e., the first card goes to hand 1, the second card goes
to hand 2, the third card goes to hand 1, fourth to hand 2. The remaining five cards will form
the shared pool. Once dealt, each hand is anazlyed and a winner is decided. 

## Tie Breaking
According to the standard rules of poker, the ranks of the cards are used to decide the
winner when both hands have the same strength. For example, if both hands are a flush,
then the hand with the card of highest rank is declared the winner. If both hands have a
pair, then the hand whose pair is higher wins. For example, a pair of Kings beats a pair of
Sevens. If both hands have the same pair, i.e. each hand has a pair of threes, then the hand
with the next highest card wins (called the kicker).

## Implementation
The function called deal accepts an array of
integers as an argument and returns the winning hand. The usage of the deal function must be as follows:
let perm:[u32;9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
let winner:Vec<String> = deal(perm);
The deal function accepts an array of 9 unsigned integers, and returns only the winning cards as a vector of Strings formatted in this way:  
    
1 => Ace of Spades => "1S"  
38 => Queen of Hearts => "12H"  
38 => 9 of Spades => "9S"  
  
## Example
If the input into deal is:  
  
perm:[u32;9] = [9, 8, 7, 6, 5, 4, 3, 2, 1]  
  
Then we see that each hand is:    
  
hand1 = [9, 7, 5, 4, 3, 2, 1];  
hand2 = [8, 6, 5, 4, 3, 2, 1];  
  
The fuction would then output:  
  
["2C", "3C", "4C", "5C", "6C"]  
  
This is because, while both hands have a straight, hand2's straight has a high-card of 6 and thus it is the one returned.


