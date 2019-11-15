

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Value{
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Suit{
    Club,
    Diamond,    
    Heart,
    Spade
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Card(Suit, Value);

#[derive(Copy, Clone, Debug)]
pub struct Hand
{
  pub cards:[Card;5]
}

// Converts from simple string format [value][suit], e.g 3S = three of spades.
pub fn card_from_str(s:&str)->Card
{
  let mut st:Suit = Suit::Club;
  let mut vl:Value = Value::Two;
  
  if s.starts_with('3'){vl = Value::Three;}
  else if s.starts_with('4'){vl = Value::Four;}
  else if s.starts_with('5'){vl = Value::Five;}
  else if s.starts_with('6'){vl = Value::Six;}
  else if s.starts_with('7'){vl = Value::Seven;}
  else if s.starts_with('8'){vl = Value::Eight;}
  else if s.starts_with('9'){vl = Value::Nine;}
  else if s.starts_with('T'){vl = Value::Ten;}
  else if s.starts_with('J'){vl = Value::Jack;}
  else if s.starts_with('Q'){vl = Value::Queen;}
  else if s.starts_with('K'){vl = Value::King;}
  else if s.starts_with('A'){vl = Value::Ace;}

  if s.ends_with('S'){st = Suit::Spade;}
  else if s.ends_with('H'){st = Suit::Heart;}
  else if s.ends_with('D'){st = Suit::Diamond;}

  Card(st,vl)
}

pub fn hand_from_str(s:&str)->Hand
{
  let mut cards:Vec<Card> = Vec::new();
  let parts = s.split(" ");
  for pt in parts{
    &cards.push(card_from_str(pt));
  }
  &cards.sort_unstable();

  Hand{cards: [cards[0], cards[1], cards[2], cards[3], cards[4]]}
}

#[derive(Debug)]
struct Stats{
  hand:Hand,
  suit_freq:[usize;4],
  value_freq:[usize;13],
  highest_value:usize,
  kickers:[usize;5]
}

impl Stats{

  // returns suit index if any suit in the hand occurs at the specified frequency.
  fn suit_contains_frequency(&self, f:usize)->Option<usize>{
    for i in 0..4{
      if self.suit_freq[i] == f{
        return Some(i);
      }
    }

    None
  }

  // returns value index if any value in the hand occurs at the specified frequency.
  fn value_contains_frequency(&self, f:usize)->Option<usize>{
    for i in 0..13{
      if self.value_freq[i] == f{
        return Some(i);
      }
    }

    None
  }

}//impl

fn get_stats(_hand:&Hand)->Stats
{
  let mut suits:[usize;4] = [0,0,0,0];
  let mut values:[usize;13] = [0,0,0,0,0,0,0,0,0,0,0,0,0];
  let mut high_val = 0;      // Highest card in hand.

  for c in &_hand.cards{
    let s_idx = c.0 as usize;
    let v_idx = c.1 as usize;
    if v_idx > high_val{
      high_val = v_idx;
    }
    suits[s_idx] = suits[s_idx] + 1;
    values[v_idx] = values[v_idx] + 1;
  }

  let mut kickers:[usize;5] = [0,0,0,0,0];
  let mut kick_index = 0;

  for i in 0..13{
    if values[i] == 1{
      kickers[kick_index] = i+2;
      kick_index += 1;
    }
  }

  kickers.sort();
  kickers.reverse();

  Stats {hand:*_hand, suit_freq: suits, 
        value_freq: values,
        highest_value: high_val,
        kickers: kickers}
}

/*
    High Card: Highest value card.
    One Pair: Two cards of the same value.
    Two Pairs: Two different pairs.
    Three of a Kind: Three cards of the same value.
    Straight: All cards are consecutive values.
    Flush: All cards of the same suit.
    Full House: Three of a kind and a pair.
    Four of a Kind: Four cards of the same value.
    Straight Flush: All cards are consecutive values of same suit.
    Royal Flush: Ten, Jack, Queen, King, Ace, in same suit.
*/
pub fn get_score(hand:&Hand)->usize{
  let stats = get_stats(&hand);

  // Flush test
  let s = stats.suit_contains_frequency(5);
  if s != None
  {
    // Royal Flush
    let ten_idx = Value::Ten as usize;
    if stats.value_freq[ten_idx] == 1 &&
       stats.value_freq[ten_idx + 1] == 1 &&
       stats.value_freq[ten_idx + 2] == 1 &&
       stats.value_freq[ten_idx + 3] == 1 &&
       stats.value_freq[ten_idx + 4] == 1
    {
      return 10000;
    }

    // Straight Flush
    for i in 0..8{
      if stats.value_freq[i] == 1 &&
         stats.value_freq[i+1] == 1 &&
         stats.value_freq[i+2] == 1 &&
         stats.value_freq[i+3] == 1 &&
         stats.value_freq[i+4] == 1{
           return 9000 + i+6;
         }
    }
  }

  // Four of a kind (value)
  let v4 = stats.value_contains_frequency(4);
  if v4 != None
  {
    return 8000 + v4.unwrap()+2;
  }

  // Full house
  let v3 = stats.value_contains_frequency(3);
  let v2 = stats.value_contains_frequency(2);
  if v3 != None && v2 != None{
    return 7000 + (v3.unwrap()+2)*7 + v2.unwrap()+2;
  }

  // Flush
  if s != None{
    return 6000 + stats.highest_value+2;
  }

  // Straight
  for i in 0..8{
    if stats.value_freq[i] == 0 { continue; }
    if stats.value_freq[i] == 1 &&
       stats.value_freq[i+1] == 1 &&
       stats.value_freq[i+2] == 1 &&
       stats.value_freq[i+3] == 1 &&
       stats.value_freq[i+4] == 1 { return 5000 + stats.highest_value+2; }
       else { break; }
  }

  // Three of a kind
  if v3 != None{
    return 4000 + (v3.unwrap()+2)*7 + stats.kickers[0] + stats.kickers[1];
  }

  // Two pair or single pair
  let mut pair_index = 0;
  let mut pair_indexes:[usize;2] = [0,0];
  for i in 0..13{
    if stats.value_freq[i] == 2{
      pair_indexes[pair_index] = i;
      pair_index += 1;
    }
  }

  // Two pair case
  if pair_index == 2{
    return 3000 + (pair_indexes[0]+2)*14 + (pair_indexes[1]+2)*14 + stats.kickers[0]; 
  }

  // Single pair case
  if pair_index == 1{
    return 2000 + (pair_indexes[0]+2)*14 + stats.kickers[0];
  }
 
  // High card
  stats.highest_value
}

#[test]
fn test_1()
{
  // Pair 5s, 7 high
  let h1 = hand_from_str("6D 7C 5D 5H 3S");

  // Pair 5s, jack high
  let h2 = hand_from_str("5C JC 2H 5S 3D");
  
  let s1 = get_score(&h1);
  let s2 = get_score(&h2);
  assert!(s2 > s1);  
}
