## `nom` subset we're aiming for
- [x] both big/little endian versions of f32/f64, {u/i}{8/16/32/64}
- [x] branch::alt
- [ ] bytes::complete::{tag, take_till, take_until, take_while},
- [ ] combinator::{complete, map},
- [ ] multi::count
- [ ] multi::{many0, many1},
- [ ] sequence::delimited - this is essentially between from parsec
- [ ] sequence::terminated
