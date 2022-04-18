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
    (self.value == other.value) && (self.suit == other.suit)
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

  // Given a set of cards and a value, return all Cards with the same value
  fn return_samevalue(&self, val: u32, vec: Vec<Card>) -> Vec<Card> {
    let mut output: Vec<Card> = Vec::new();

    for i in 0..(vec.len()) {
      if vec[i].value == val {
        output.push(vec[i]);
      }
    }
    return output;
  }

  // Return a set of 4 cards if they all have the same value
  fn return_fourkind(&self, vec: Vec<Card>, hash: HashMap<u32, u32>) -> Vec<Card> {
    let mut output: Vec<Card> = vec![Card { value: 0, suit: 'C' }; 4];

    for (key, val) in &hash {
      if val == &4 {
        output = self.return_samevalue(*key, vec.to_vec());
      }
    }

    return output;
  }

  // Return every set of 3 cards with the same value.
  fn return_threekind(&self, vec: Vec<Card>, hash: HashMap<u32, u32>) -> Vec<Vec<Card>> {
    let mut output: Vec<Vec<Card>> = Vec::new();

    for (key, val) in &hash {
      if val == &3 {
        output.push(self.return_samevalue(*key, vec.to_vec()));
      }
    }

    return output;
  }

  // Return every set of 2 cards with the same value.
  fn return_twokind(&self, vec: Vec<Card>, hash: HashMap<u32, u32>) -> Vec<Vec<Card>> {
    let mut output: Vec<Vec<Card>> = Vec::new();

    for (key, val) in &hash {
      if val == &2 {
        output.push(self.return_samevalue(*key, vec.to_vec()));
      }
    }

    return output;
  }

  // Return the card with the highest value.
  fn return_highcard(&self, vec: Vec<Card>) -> Card {
    let mut max = 0;
    let mut ind = 0;

    for i in 0..(vec.len()) {
      if max == 0 {
        max = vec[i].value;
      }

      if vec[i].value > max{
        max = vec[i].value;
        ind = i;
      }
    }
    
    return vec[ind];
  }

  //Return the strongest hand and update the Hand.strength.
  fn determine_strength(&mut self, vec: Vec<Card>) -> Vec<Card> {
    let flushes: Vec<Card> = self.return_flushes(vec.to_vec());
    let straights: Vec<Vec<Card>> = self.return_straights(vec.to_vec());
    
    let map: HashMap<u32, u32> = self.create_map();
    let map1: HashMap<u32, u32> = self.create_map();
    let map2: HashMap<u32, u32> = self.create_map();
    let fourkinds: Vec<Card> = self.return_fourkind(vec.to_vec(), map);
    let threekinds: Vec<Vec<Card>> = self.return_threekind(vec.to_vec(), map1);
    let twokinds: Vec<Vec<Card>> = self.return_twokind(vec.to_vec(), map2);
    
    

    // Check if this hand contains straights and flushes. If it contains both, then check for a royal flush, otherwise return the strongest straight flush if there is one.
    if (straights.len() >= 1) && (flushes.len() >= 1) {
      let mut straightflushes: Vec<Vec<Card>> = Vec::new(); 
      for i in 0..(straights.len()) {
        let mut temp = true;
        
        for x in 0..5 {
          if flushes.contains(&straights[i][x]) == false {
            temp = false;
            break;
          }
        }

        if temp {
          straightflushes.push(straights[i].to_vec());
        }
      }

      let mut max = 0;
      let mut ind = 0;
      // Check if we found any straight flushes.
      if straightflushes.len() >= 1 {
        
        for i in 0..(straightflushes.len()) {
          let highest = self.return_highcard(straightflushes[i].to_vec());

          // Remember the index of the straight flush with the strongest card
          if (highest.value > max) | (highest.value == 1) {
            max = highest.value;
            ind = i;
          }

          // If the strongest card is an Ace, then it's a Royal Flush.
          if highest.value == 1 {
            self.strength = HandLevel::RoyalFlush;
            return straightflushes[i].to_vec();
          }
        }

        // If no royal flush was found, then return the straight flush with the strongest card.
        self.strength = HandLevel::StraightFlush;
        return straightflushes[ind].to_vec();
      }
    } 

    // If there is a four of a kind, then return that.
    if fourkinds.len() >= 1 {
      self.strength = HandLevel::FourKind;
      return fourkinds;
    }

    // If there is three of a kind and two of a kind, then return the full house with the strongest of each.
    if (threekinds.len() >= 1) && (twokinds.len() >= 1) {
      let mut output: Vec<Card> = Vec::new(); 
      let mut max = 0;
      let mut ind = 0;

      for i in 0..(twokinds.len()) {
        let highest = self.return_highcard(twokinds[i].to_vec());

        // Remember the index of the pair with the strongest card
        if (highest.value > max) | (highest.value == 1) {
          max = highest.value;
          ind = i;
        }
      }

      output.push(twokinds[ind][0]);
      output.push(twokinds[ind][1]);

      max = 0;
      ind = 0;

      for i in 0..(threekinds.len()) {
        let highest = self.return_highcard(threekinds[i].to_vec());

        // Remember the index of the pair with the strongest card
        if (highest.value > max) | (highest.value == 1) {
          max = highest.value;
          ind = i;
        }
      }

      output.push(threekinds[ind][0]);
      output.push(threekinds[ind][1]);
      output.push(threekinds[ind][2]);

      self.strength = HandLevel::FullHouse;
      return output;
    }

  // Return strongest flush.
  if flushes.len() >= 1 {
    let mut output: Vec<Card> = Vec::new(); 

    for i in (flushes.len() - 5)..(flushes.len()) {
      output.push(flushes[i])
    }

    self.strength = HandLevel::Flush;
    return output;
  }

  // Return strongest straight.
  if straights.len() >= 1 {
    let mut max = 0;
    let mut ind = 0;

    for i in 0..(straights.len()) {
      let highest = self.return_highcard(straights[i].to_vec());

      // Remember the index of the pair with the strongest card.
      if (highest.value > max) | (highest.value == 1) {
        max = highest.value;
        ind = i;
      }
    }

    self.strength = HandLevel::Straight;
    return straights[ind].to_vec();
  }

  // Return strongest three of a kind.
  if threekinds.len() >= 1 {
    let mut max = 0;
    let mut ind = 0;

    for i in 0..(threekinds.len()) {
      let highest = self.return_highcard(threekinds[i].to_vec());

      // Remember the index of the pair with the strongest card
      if (highest.value > max) | (highest.value == 1) {
        max = highest.value;
        ind = i;
      }
    }

    self.strength = HandLevel::ThreeKind;
    return threekinds[ind].to_vec();
  }

  // Return strongest two pairs.
  if twokinds.len() >= 2 {
    let mut output: Vec<Card> = Vec::new(); 
    let mut max = 0;
    let mut ind = 0;

    for i in 0..(twokinds.len()) {
      let highest = self.return_highcard(twokinds[i].to_vec());

      // Remember the index of the pair with the strongest card
      if (highest.value > max) | (highest.value == 1) {
        max = highest.value;
        ind = i;
      }
    }

    let mut max1 = 0;
    let mut ind1 = 0;

    for i in 0..(twokinds.len()) {
      let highest = self.return_highcard(twokinds[i].to_vec());

      // Remember the index of the pair with the strongest card
      if ((highest.value > max1) | (highest.value == 1)) && (highest.value != max) {
        max1 = highest.value;
        ind1 = i;
      }
    }
    
    output.push(twokinds[ind1][0]);
    output.push(twokinds[ind1][1]);
    output.push(twokinds[ind][0]);
    output.push(twokinds[ind][1]);

    self.strength = HandLevel::TwoPair;
    return output;
  }

  // Return strongest single pair.
  if twokinds.len() >= 1 {
    let mut max = 0;
    let mut ind = 0;

    for i in 0..(twokinds.len()) {
      let highest = self.return_highcard(twokinds[i].to_vec());

      // Remember the index of the pair with the strongest card
      if (highest.value > max) | (highest.value == 1) {
        max = highest.value;
        ind = i;
      }
    }
    
    self.strength = HandLevel::Pair;
    return twokinds[ind].to_vec();
  }

  // Return highest card.
  let mut output: Vec<Card> = Vec::new(); 
  output.push(self.return_highcard(vec));
  return output;
  }

  // Return a higher value, the better stronger a Hand is.
  fn hand_strength(&self) -> u32 {
    match self.strength {
      HandLevel::RoyalFlush => 10,
      HandLevel::StraightFlush => 9,
      HandLevel::FourKind => 8,
      HandLevel::FullHouse => 7,
      HandLevel::Flush => 6,
      HandLevel::Straight => 5,
      HandLevel::ThreeKind => 4,
      HandLevel::TwoPair => 3,
      HandLevel::Pair => 2,
      HandLevel::HighCard => 1,
      HandLevel::None => 0,
    }
  }
}

pub fn cards_tostring(vec: Vec<Card>) -> Vec<String> {
  let mut output: Vec<String> = Vec::new();

  for i in 0..(vec.len()) {
    output.push(vec[i].convert_string());
  }
  
  return output;
}

pub fn deal(perm:[u32; 9]) -> Vec<String> {
  let mut hand1 = [perm[0], perm[2], 1, 2, 3, 4, 5];
  let mut hand2 = [perm[1], perm[3], 1, 2, 3, 4, 5];

  for i in 4..9 {
    hand1[i - 2] = perm[i];
    hand2[i - 2] = perm[i];
  }

  println!("{:?}", hand1);
  println!("{:?}", hand2);

  let mut hand1 = Hand::new(hand1);
  let mut hand2 = Hand::new(hand2);

  let hand1_cards = hand1.sort_cards();
  let hand2_cards = hand2.sort_cards();

  let hand1_strongest = hand1.determine_strength(hand1_cards.to_vec());
  let hand2_strongest = hand2.determine_strength(hand2_cards.to_vec());

  if hand1.hand_strength() > hand2.hand_strength() {
    return cards_tostring(hand1_strongest.to_vec());
    
  } else if hand1.hand_strength() < hand2.hand_strength() {
    return cards_tostring(hand2_strongest.to_vec());
    
  } else {

    let temp1 = (hand1.return_highcard(hand1_strongest.to_vec())).value;
    let temp2 = (hand2.return_highcard(hand2_strongest.to_vec())).value;
    
    if temp1 > temp2 {
      return cards_tostring(hand1_strongest.to_vec());
    } else {
      return cards_tostring(hand2_strongest.to_vec());
    }
  }
}

pub fn main() {   
  let perm:[u32;9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
  let winner:Vec<String> = deal(perm);

  println!("{:?}", winner);
}
