#[macro_export]
macro_rules! test_driver_read {
  ($tests:path) => {
    mod read {
      use super::*;

      use ::ardite::value::{Object, Value};
      use ::ardite::query::{Condition, SortRule, Range};

      fn val_a() -> Value {
        let mut object = Object::new();
        // TODO: Use more flexible insert.
        object.insert("a".to_owned(), Value::I64(1));
        object.insert("b".to_owned(), Value::I64(2));
        object.insert("c".to_owned(), Value::I64(3));
        object.insert("d".to_owned(), Value::I64(4));
        Value::Object(object)
      }

      fn val_b() -> Value {
        let mut object = Object::new();
        object.insert("b".to_owned(), Value::I64(2));
        object.insert("c".to_owned(), Value::I64(4));
        object.insert("hello".to_owned(), Value::String("world".to_owned()));
        object.insert("doc_a".to_owned(), val_a());
        Value::Object(object)
      }

      fn val_c() -> Value {
        let mut object = Object::new();
        object.insert("a".to_owned(), Value::I64(1));
        object.insert("c".to_owned(), Value::I64(3));
        object.insert("doc_b".to_owned(), val_b());
        Value::Object(object)
      }

      fn vals() -> Vec<Value> {
        let mut vals = Vec::new();
        vals.push(val_a());
        vals.push(val_b());
        vals.push(val_c());
        vals
      }

      #[test]
      fn test_all() {
        let driver = <$tests as $crate::Tests>::test_driver("read_all", vals());
        assert_eq!(
          driver.read(
            "read_all",
            Default::default(),
            Default::default(),
            Default::default()
          ).unwrap().collect::<Vec<Value>>(),
          vec![val_a(), val_b(), val_c()]
        );
      }

      #[test]
      fn test_condition() {
        let driver = <$tests as $crate::Tests>::test_driver("read_condition", vals());
        assert_eq!(
          driver.read(
            "read_condition",
            Condition::False,
            Default::default(),
            Default::default()
          ).unwrap().collect::<Vec<Value>>(),
          vec![]
        );
        assert_eq!(
          driver.read(
            "read_condition",
            Condition::And(vec![Condition::True, Condition::False]),
            Default::default(),
            Default::default()
          ).unwrap().collect::<Vec<Value>>(),
          vec![]
        );
        assert_eq!(
          driver.read(
            "read_condition",
            Condition::Or(vec![Condition::True, Condition::False]),
            Default::default(),
            Default::default()
          ).unwrap().collect::<Vec<Value>>(),
          vec![val_a(), val_b(), val_c()]
        );
        assert_eq!(
          driver.read(
            "read_condition",
            Condition::Key("c".to_owned(), Box::new(Condition::Equal(Value::I64(3)))),
            Default::default(),
            Default::default()
          ).unwrap().collect::<Vec<Value>>(),
          vec![val_a(), val_c()]
        );
        assert_eq!(
          driver.read(
            "read_condition",
            Condition::Key(
              "doc_b".to_owned(),
              Box::new(Condition::Key(
                "doc_a".to_owned(),
                Box::new(Condition::Key(
                  "d".to_owned(),
                  Box::new(Condition::Equal(Value::I64(4)))
                ))
              ))
            ),
            Default::default(),
            Default::default()
          ).unwrap().collect::<Vec<Value>>(),
          vec![val_c()]
        );
      }

      #[test]
      fn test_sort() {
        let driver = <$tests as $crate::Tests>::test_driver("read_sort", vals());
        assert_eq!(
          driver.read(
            "read_sort",
            Default::default(),
            vec![SortRule::new(vec!["c".to_owned()], true)],
            Default::default()
          ).unwrap().collect::<Vec<Value>>(),
          vec![val_a(), val_c(), val_b()]
        );
        assert_eq!(
          driver.read(
            "read_sort",
            Default::default(),
            vec![SortRule::new(vec!["c".to_owned()], false)],
            Default::default()
          ).unwrap().collect::<Vec<Value>>(),
          vec![val_b(), val_a(), val_c()]
        );
      }

      #[test]
      fn test_range() {
        let driver = <$tests as $crate::Tests>::test_driver("read_range", vals());
        assert_eq!(
          driver.read(
            "read_range",
            Default::default(),
            Default::default(),
            Range::new(None, Some(2))
          ).unwrap().collect::<Vec<Value>>(),
          vec![val_a(), val_b()]
        );
        assert_eq!(
          driver.read(
            "read_range",
            Default::default(),
            Default::default(),
            Range::new(Some(1), Some(1))
          ).unwrap().collect::<Vec<Value>>(),
          vec![val_b()]
        );
        assert_eq!(
          driver.read(
            "read_range",
            Default::default(),
            Default::default(),
            Range::new(Some(1), None)
          ).unwrap().collect::<Vec<Value>>(),
          vec![val_b(), val_c()]
        );
        assert_eq!(
          driver.read(
            "read_range",
            Default::default(),
            Default::default(),
            Range::new(Some(2), Some(40))
          ).unwrap().collect::<Vec<Value>>(),
          vec![val_c()]
        );
      }

      #[test]
      fn test_condition_before_range() {
        let driver = <$tests as $crate::Tests>::test_driver("read_condition_before_range", vals());
        assert_eq!(
          driver.read(
            "read_condition_before_range",
            Condition::Key("c".to_owned(), Box::new(Condition::Equal(Value::I64(3)))),
            Default::default(),
            Range::new(None, Some(1))
          ).unwrap().collect::<Vec<Value>>(),
          vec![val_a()]
        );
        assert_eq!(
          driver.read(
            "read_condition_before_range",
            Condition::Key("c".to_owned(), Box::new(Condition::Equal(Value::I64(3)))),
            Default::default(),
            Range::new(Some(1), Some(1))
          ).unwrap().collect::<Vec<Value>>(),
          vec![val_c()]
        );
      }
    }
  }
}
