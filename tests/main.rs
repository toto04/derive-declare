#[derive(derive_declare::Declare, Debug)]
pub struct MyStruct {
    pub field_one: String,
    pub field_two: i32,
}

impl Default for MyStruct {
    fn default() -> Self {
        MyStruct {
            field_one: "Default".to_string(),
            field_two: 0,
        }
    }
}

#[cfg(test)]
pub mod tests {

    use super::*;

    #[test]
    fn test_my_struct_macro() {
        let a = my_struct! {
            field_one: "Hello".to_string(),
            field_two: 42,
        };

        assert_eq!(a.field_one, "Hello");
        assert_eq!(a.field_two, 42);
    }

    #[test]
    fn test_my_struct_default() {
        let a = my_struct! {};
        print!("{a:?}");
        assert_eq!(a.field_one, "Default");
        assert_eq!(a.field_two, 0);
    }

    #[test]
    fn test_my_struct_partial() {
        let a = my_struct! {
            field_one: "Hello".to_string()
        };
        print!("{a:?}");
        assert_eq!(a.field_one, "Hello");
        assert_eq!(a.field_two, 0);
    }

    #[test]
    fn test_my_struct_named_assignment() {
        let field_one = "Hello".to_string();
        let a = my_struct! {
            field_one,
            field_two: 42,
        };

        assert_eq!(a.field_one, "Hello");
        assert_eq!(a.field_two, 42);
    }
}
