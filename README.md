## `nom` subset we're aiming for
- [x] both big/little endian versions of f32/f64, {u/i}{8/16/32/64}
- [x] branch::alt
- [x] multi::count
- [x] multi::{many0, many1} - `repeat`, `repeat1` here
- [x] sequence::delimited - `surrounded_by` here
- [x] sequence::terminated - `followed_by` here
- [ ] bytes::complete::{tag, take_till, take_until, take_while},
- [ ] combinator::{complete, map},
