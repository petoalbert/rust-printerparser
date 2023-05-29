use crate::printerparser::*;
use crate::parsers::*;

#[cfg(test)]
mod tests {
    use std::collections::LinkedList;

    use super::*;

    #[test]
    fn test_digit_expected() {
        assert!(matches!(digit().parse("2", &mut ()), Ok(("", '2'))));
        assert!(matches!(digit().parse("a", &mut ()), Err(_)));
        assert_eq!(digit().print('2', &mut ()).unwrap(), "2")
    }

    #[test]
    fn test_string() {
        let expected = "hello".to_owned();
        assert!(matches!(
            string("hello").parse("hello there", &mut ()),
            Ok((" there", expected))
        ));

        assert_eq!(
            string("hello").print("hello".to_owned(), &mut ()).unwrap(),
            "hello"
        );

        assert!(matches!(
            string("hello").parse("general kenobi", &mut ()),
            Err(_)
        ));
    }

    #[test]
    fn test_tag() {
        let expected = b"hello".to_vec();
        assert!(matches!(
            tag(b"hello").read(b"hello there", &mut ()),
            Ok((b" there", expected))
        ));

        assert_eq!(
            tag(b"hello").write(b"hello".to_vec(), &mut ()).unwrap(),
            b"hello"
        );

        assert!(matches!(
            tag(b"hello").read(b"general kenobi", &mut ()),
            Err(_)
        ));
    }

    #[test]
    fn test_preceded_by() {
        let grammar = preceded_by(char('*'), string("hello"));
        let expected = "hello".to_owned();
        assert_eq!(grammar.parse("*hello", &mut ()), Ok(("", expected)));
        assert_eq!(
            string("*hello")
                .print("*hello".to_owned(), &mut ())
                .unwrap(),
            "*hello"
        )
    }

    #[test]
    fn test_repeat() {
        let grammar = string("rust").repeat();

        let values = vec!["rust".to_owned(), "rust".to_owned(), "rust".to_owned()];
        let mut list_for_parse = LinkedList::new();

        list_for_parse.extend(values);

        match grammar.parse("rustrustrust", &mut ()) {
            Ok(("", result)) => assert_eq!(result, list_for_parse),
            _ => panic!("Unexpected value"),
        }

        let values = vec!["rust".to_owned(), "rust".to_owned()];
        let mut list_for_print = LinkedList::new();

        list_for_print.extend(values);

        assert_eq!(grammar.print(list_for_print, &mut ()).unwrap(), "rustrust")
    }

    #[test]
    fn test_repeat1() {
        let grammar = repeat1(string("rust"));
        let values = vec!["rust".to_owned(), "rust".to_owned()];
        let mut list_for_parse = LinkedList::new();

        list_for_parse.extend(values);

        match grammar.parse("rustrust", &mut ()) {
            Ok(("", result)) => assert_eq!(result, list_for_parse),
            _ => panic!("Unexpected value"),
        }

        let values = vec![()];
        let mut list_for_parse2 = LinkedList::new();
        list_for_parse2.extend(values);

        assert!(matches!(grammar.parse("", &mut ()), Err(_)))
    }

    #[test]
    fn test_count_ok() {
        let grammar = digit().count(3);

        let (rest, result) = grammar.parse("123hello", &mut ()).unwrap();
        assert_eq!(rest, "hello");

        let mut list = LinkedList::new();
        list.extend(vec!['1', '2', '3']);
        assert_eq!(result, list);

        let printed = grammar.print(list, &mut ()).unwrap();
        assert_eq!(printed, "123")
    }

    #[test]
    fn test_count_not_enough() {
        let grammar = digit().count(5);

        let result = grammar.parse("123hello", &mut ());
        assert!(matches!(result, Err(_)));
    }

    #[test]
    fn test_count_too_many() {
        let grammar = digit().count(2);

        let (rest, result) = grammar.parse("123hello", &mut ()).unwrap();
        assert_eq!(rest, "3hello");

        let mut list = LinkedList::new();
        list.extend(vec!['1', '2']);
        assert_eq!(result, list)
    }

    #[test]
    fn test_or() {
        let grammar = string("rust").or(string("haskell"));

        let parsed_rust = "rust".to_owned();
        let parsed_haskell = "haskell".to_owned();

        assert!(matches!(
            grammar.parse("rust", &mut ()),
            Ok(("", parsed_rust))
        ));
        assert!(matches!(
            grammar.parse("haskell", &mut ()),
            Ok(("", parsed_haskell))
        ));
        assert!(matches!(grammar.parse("javascript", &mut ()), Err(_)));
        assert_eq!(grammar.print(parsed_rust, &mut ()).unwrap(), "rust");
        assert_eq!(grammar.print(parsed_haskell, &mut ()).unwrap(), "haskell");
    }

    #[test]
    fn test_take_while() {
        let grammar = take_while(ANY_CHAR, |a| a.is_digit(10));
        let values = vec!['1', '2', '3'];
        let mut list_for_parse = LinkedList::new();

        list_for_parse.extend(values);

        match grammar.parse("123aaaa", &mut ()) {
            Ok(("aaaa", result)) => assert_eq!(result, list_for_parse),
            v => panic!("Unexpected value {:?}", v),
        }

        match grammar.parse("aaaa", &mut ()) {
            Ok(("aaaa", result)) => assert_eq!(result, LinkedList::new()),
            v => panic!("Unexpected value {:?}", v),
        }
    }

    #[test]
    fn test_take_till() {
        let grammar = take_till(ANY_CHAR, |a| a.is_ascii_punctuation());
        let (rest, result) = grammar.parse("shut! up", &mut ()).unwrap();
        let mut list = LinkedList::new();
        list.extend(vec!['s', 'h', 'u', 't']);
        assert_eq!(rest, "! up");
        assert_eq!(result, list);

        let printed = grammar.print(list, &mut ()).unwrap();
        assert_eq!(printed, "shut")
    }

    #[test]
    fn test_i32() {
        let bytes: [u8; 4] = [1, 2, 3, 4]; // 1 + 2*2^8 + 3*2^16 + 4*2^24 4 + 3*2^8 + 2*2^16 + 2^24

        let (_, i_le) = i32().read(&bytes, &mut Endianness::LittleEndian).unwrap();
        let (_, i_be) = i32().read(&bytes, &mut Endianness::BigEndindan).unwrap();
        let bytes_le = i32().write(i_le, &mut Endianness::LittleEndian).unwrap();
        let bytes_be = i32().write(i_be, &mut Endianness::BigEndindan).unwrap();

        assert_eq!(i_le, 67_305_985);
        assert_eq!(i_be, 16_909_060);
        assert_eq!(bytes_le, bytes);
        assert_eq!(bytes_be, bytes);
    }

    #[test]
    fn test_state() {
        let le_bytes: [u8; 5] = [0, 1, 2, 3, 4];
        let be_bytes: [u8; 5] = [1, 4, 3, 2, 1];

        let endianness = bytes(1).as_state(
            |bs, s: &mut Endianness| match bs.first().unwrap() {
                0 => {
                    *s = Endianness::LittleEndian;
                    Ok(())
                }
                1 => {
                    *s = Endianness::BigEndindan;
                    Ok(())
                }
                _ => Err("Unreadable endianness".to_owned()),
            },
            |s: &Endianness| match s {
                Endianness::LittleEndian => Ok([0].to_vec()),
                Endianness::BigEndindan => Ok([1].to_vec()),
            },
        );

        let grammar = preceded_by(endianness, i32());

        let mut parsed_state = Endianness::LittleEndian;
        let (_, i) = grammar.read(&be_bytes, &mut parsed_state).unwrap();
        let result_bytes = grammar.write(i, &mut Endianness::LittleEndian).unwrap();

        assert_eq!(parsed_state, Endianness::BigEndindan);
        assert_eq!(i, 67_305_985);
        assert_eq!(result_bytes, le_bytes);
    }
}