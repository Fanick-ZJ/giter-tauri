#[macro_export]
macro_rules! make_serializable {
  (
    $(#[$enum_attr:meta])* // 捕获枚举上的所有属性（如 #[derive(...)]）
      pub enum $enum_name:ident {
          $(
              $variant:ident
          ),* $(,)?
      }
  ) => {
      $(#[$enum_attr])* // 将属性应用到生成的枚举上
      pub enum $enum_name {
          $(
              $variant,
          )*
      }

      impl serde::Serialize for $enum_name {
          fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
          where
              S: serde::Serializer,
          {
              // 引入trait到作用域
              use serde::ser::SerializeStruct;

              let mut s = serializer.serialize_struct(stringify!($enum_name), 1)?;
              match self {
                  $(
                      $enum_name::$variant => {
                         s.serialize_field("type", stringify!($variant))?;
                      }
                  )*
              }
              s.end()
          }
      }
  };
}
