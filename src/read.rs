#[macro_export]
macro_rules! test_driver_read {
  ($tests:path) => {
    mod read {
      use super::*;

      use ::ardite::value::{Object, Value};
      use ::ardite::query::{Condition, SortRule, Range, Query};
      // TODO: Find a way to remove this dependency.
      use ::linear_map::LinearMap;

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
      fn test_read_all() {
        let driver = <$tests as $crate::Tests>::test_driver("read_all", vals());
        assert_eq!(
          driver.read(
            "read_all",
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default()
          ).unwrap().collect::<Vec<Value>>(),
          vec![val_a(), val_b(), val_c()]
        );
      }

      #[test]
      fn test_read_condition() {
        let driver = <$tests as $crate::Tests>::test_driver("read_condition", vals());
        assert_eq!(
          driver.read(
            "read_condition",
            Condition::False,
            Default::default(),
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
            Default::default(),
            Default::default()
          ).unwrap().collect::<Vec<Value>>(),
          vec![val_c()]
        );
      }

      #[test]
      fn test_read_sort() {
        let driver = <$tests as $crate::Tests>::test_driver("read_sort", vals());
        assert_eq!(
          driver.read(
            "read_sort",
            Default::default(),
            vec![SortRule::new(vec!["c".to_owned()], true)],
            Default::default(),
            Default::default()
          ).unwrap().collect::<Vec<Value>>(),
          vec![val_a(), val_c(), val_b()]
        );
        assert_eq!(
          driver.read(
            "read_sort",
            Default::default(),
            vec![SortRule::new(vec!["c".to_owned()], false)],
            Default::default(),
            Default::default()
          ).unwrap().collect::<Vec<Value>>(),
          vec![val_b(), val_a(), val_c()]
        );
      }

      #[test]
      fn test_read_range() {
        let driver = <$tests as $crate::Tests>::test_driver("read_range", vals());
        assert_eq!(
          driver.read(
            "read_range",
            Default::default(),
            Default::default(),
            Range::new(None, Some(2)),
            Default::default()
          ).unwrap().collect::<Vec<Value>>(),
          vec![val_a(), val_b()]
        );
        assert_eq!(
          driver.read(
            "read_range",
            Default::default(),
            Default::default(),
            Range::new(Some(1), Some(1)),
            Default::default()
          ).unwrap().collect::<Vec<Value>>(),
          vec![val_b()]
        );
        assert_eq!(
          driver.read(
            "read_range",
            Default::default(),
            Default::default(),
            Range::new(Some(1), None),
            Default::default()
          ).unwrap().collect::<Vec<Value>>(),
          vec![val_b(), val_c()]
        );
        assert_eq!(
          driver.read(
            "read_range",
            Default::default(),
            Default::default(),
            Range::new(Some(2), Some(40)),
            Default::default()
          ).unwrap().collect::<Vec<Value>>(),
          vec![val_c()]
        );
      }

      #[test]
      fn test_read_query() {
        let driver = <$tests as $crate::Tests>::test_driver("read_query", vals());
        assert_eq!(
          driver.read(
            "read_query",
            Default::default(),
            Default::default(),
            Default::default(),
            Query::All
          ).unwrap().collect::<Vec<Value>>(),
          vec![val_a(), val_b(), val_c()]
        );
        assert_eq!(
          driver.read(
            "read_query",
            Default::default(),
            Default::default(),
            Default::default(),
            Query::Keys({
              let mut keys = LinearMap::new();
              keys.insert("a".to_owned(), Query::All);
              keys.insert("c".to_owned(), Query::All);
              keys.insert("hello".to_owned(), Query::All);
              keys.insert("doc_a".to_owned(), Query::Keys({
                let mut keys = LinearMap::new();
                keys.insert("b".to_owned(), Query::All);
                keys
              }));
              keys.insert("doc_b".to_owned(), Query::Keys({
                let mut keys = LinearMap::new();
                keys.insert("hello".to_owned(), Query::All);
                keys.insert("doc_a".to_owned(), Query::Keys({
                  let mut keys = LinearMap::new();
                  keys.insert("b".to_owned(), Query::All);
                  keys
                }));
                keys
              }));
              keys
            })
          ).unwrap().collect::<Vec<Value>>(),
          vec![
            {
              let mut object = Object::new();
              object.insert("a".to_owned(), Value::I64(1));
              object.insert("c".to_owned(), Value::I64(3));
              Value::Object(object)
            },
            {
              let mut object = Object::new();
              object.insert("c".to_owned(), Value::I64(4));
              object.insert("hello".to_owned(), Value::String("world".to_owned()));
              object.insert("doc_a".to_owned(), {
                let mut object = Object::new();
                object.insert("b".to_owned(), Value::I64(2));
                Value::Object(object)
              });
              Value::Object(object)
            },
            {
              let mut object = Object::new();
              object.insert("a".to_owned(), Value::I64(1));
              object.insert("c".to_owned(), Value::I64(3));
              object.insert("doc_b".to_owned(), {
                let mut object = Object::new();
                object.insert("hello".to_owned(), Value::String("world".to_owned()));
                object.insert("doc_a".to_owned(), {
                  let mut object = Object::new();
                  object.insert("b".to_owned(), Value::I64(2));
                  Value::Object(object)
                });
                Value::Object(object)
              });
              Value::Object(object)
            }
          ]
        );
      }
    }
  }
}
