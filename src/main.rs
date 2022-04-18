use std::cmp::Ordering;
use std::collections::HashMap;

// An enum of all types of winning hands.
#[derive(Eq, Ord, PartialEq, PartialOrd, Debug)]
#[allow(dead_code)]
pub enum HandLevel {
  RoyalFlush,
  StraightFlush,
  FourKind,
  FullHouse,
  Flush,
  Straight,
  ThreeKind,
  TwoPair,
  Pair,
  HighCard,
  None,
}

// Card is an object with a value and a suit.
#[derive(Debug, Eq, Clone, Copy)]
pub struct Card {
  value: u32,
  suit: char,
}

// Given an unsigned integer value of 1 to 52 inclusive, it will produce a Card object from it.
impl Card {
  fn new(crd: u32) -> Card {
    if crd >= 40 {
        Card {value: crd - 39, suit: 'S'}
    } else if crd >= 27 {
        Card {value: crd - 26, suit: 'H'}
    } else if crd >= 14 {
        Card {value: crd - 13, suit: 'D'}
    } else {
        Card {value: crd, suit: 'C'}
    }
  }

  // Converts a Card into a String
  fn convert_string(&self) -> String {
    let output = format!("{}{}", self.value, self.suit);
    return output;
  }
}

impl Ord for Card {
  fn cmp(&self, other: &Card) -> Ordering {
    if self.value > other.value {
      Ordering::Greater
    } else if self.value < other.value {
      Ordering::Less
    } else {
      Ordering::Equal
    }
  }
}

impl PartialOrd for Card {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    self.value.partial_cmp(&other.value)
  }
}

impl PartialEq for Card {
  fn eq(&self, other: &Self) -> bool {
    self.value == other.value
  }
}

// A Hand is comprised of a vector of 7 cards.
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
#[allow(dead_code)]
pub struct Hand {
  cards: Vec<Card>,
  strength: HandLevel
}

// It is implemented by passing it an array of 7 integers from 1 to 52, inclusive. Defaults to no strength upon initialization.
impl Hand {
  fn new(cards: [u32; 7]) -> Hand {
    Hand {
      cards: vec![
        Card::new(cards[0]),
        Card::new(cards[1]),
        Card::new(cards[2]),
        Card::new(cards[3]),
        Card::new(cards[4]),
        Card::new(cards[5]),
        Card::new(cards[6])],
      strength: HandLevel::None
    }
  }

  // Returns a vector with all the Cards sorted by value in ascending order.
  fn sort_cards(&self) -> Vec<Card> {
    let mut temp: Vec<Card> = vec![Card { value: 0, suit: 'C' }; 7];
    temp.clone_from_slice(&(self.cards));
    temp.sort_by(|a, b| a.cmp(b));
    return temp;
  }

  // Create a dictionary of all card values
  fn create_map(&self) -> HashMap<u32, u32> {
    let mut dict = HashMap::new(); 

    // Iterate through all Cards in Hand
    for i in 0..7 {
      let val = (self.cards[i]).value;
      if dict.contains_key(&val) {
        if let Some(x) = dict.get_mut(&val) {
          *x += 1;
        }
      } else {
        dict.insert(val, 1);
      }
    }  
    return dict;
  } 

  // Return a set of 5 or more card with the same suit
  fn return_flushes(&self, vec: Vec<Card>) -> Vec<Card> {
    
    let mut output: Vec<Card> = Vec::new();
    for i in 0..7 {
      if vec[i].suit == 'C' {
        output.push(vec[i]);
      }
    }
    if output.len() >= 5 {
      return output;
    }

    let mut output: Vec<Card> = Vec::new();
    for i in 0..7 {
      if vec[i].suit == 'S' {
        output.push(vec[i]);
      }
    }
    if output.len() >= 5 {
      return output;
    }

    let mut output: Vec<Card> = Vec::new();
    for i in 0..7 {
      if vec[i].suit == 'H' {
        output.push(vec[i]);
      }
    }
    if output.len() >= 5 {
      return output;
    }

    let mut output: Vec<Card> = Vec::new();
    for i in 0..7 {
      if vec[i].suit == 'D' {
        output.push(vec[i]);
      }
    }
    if output.len() >= 5 {
      return output;
    }
    
    let output: Vec<Card> = Vec::new();
    return output;
  }  

  // Return every set of 5 Cards that count as a Straight
  fn return_straights(&self, vec: Vec<Card>) -> Vec<Vec<Card>> {
    let mut output: Vec<Vec<Card>> = Vec::new();

    // Go through cards looking for straights
    for i in 0..4 {
      // If the current card is 11 then no straights are possible with a low card of Jack.
      if vec[i].value >= 11 {
        break;
      }

      // Do both low-card and high-card for Aces.
      if vec[i].value == 1 {
        let mut temp: Vec<Card> = Vec::new();
        temp.push(vec[i]);

        for x in 2..6 {
          let exists = vec
            .iter()
            .position(|&a| a.value == x)
            .unwrap_or(9);

          // If x does not exist in the array then end the loop.
          if exists != 9 {
            temp.push(vec[exists]);
          } else {
            break;
          }
        }

        if temp.len() == 5 {
          output.push(temp);
        }

        let mut temp: Vec<Card> = Vec::new();

        for x in 10..14 {
          let exists = vec
            .iter()
            .position(|&a| a.value == x)
            .unwrap_or(9);

          // If x does not exist in the array then end the loop.
          if exists != 9 {
            temp.push(vec[exists]);
          } else {
            break;
          }
        }
        temp.push(vec[i]);

        if temp.len() == 5 {
          output.push(temp);
        }
        
      } else {
        let mut temp: Vec<Card> = Vec::new();
        let start = vec[i].value + 1;
        let end = vec[i].value + 5;
        temp.push(vec[i]);
        

        for x in start..end {
          let exists = vec
            .iter()
            .position(|&a| a.value == x)
            .unwrap_or(9);

          // If x does not exist in the array then end the loop.
          if exists != 9 {
            temp.push(vec[exists]);
          } else {
            break;
          }
        }
        if temp.len() == 5 {
          output.push(temp);
        }
      }
    }

    return output;
  }

  fn determine_strength(&mut self, vec: Vec<Card>) -> Vec<Card> {
    let output: Vec<Card> = Vec::new();
    let straights = self.return_straights(vec);

    // Check if any of the straights are also flushes
    for i in 0..(straights.len()) {
      let flush = self.return_flushes(straights[i]);

      if flush.len() != 0 {
        
      }
    }


    return output;
  }
}

pub fn deal(perm:[u32; 9]) -> Vec<Card> {
  let mut hand1 = [perm[0], perm[2], 1, 2, 3, 4, 5];
  let mut hand2 = [perm[1], perm[3], 1, 2, 3, 4, 5];

  for i in 4..9 {
    hand1[i - 2] = perm[i];
    hand2[i - 2] = perm[i];
  }

  println!("{:?}", hand1);
  println!("{:?}", hand2);

  let mut hand1 = Hand::new(hand1);
  let output = hand1.return_flushes(hand1.sort_cards());
  return output;
}

pub fn main() {   
  let perm:[u32;9] = [8, 2, 9, 4, 50, 41, 42, 43, 44];
  let output = deal(perm);

  println!("{:?}", output);
}
