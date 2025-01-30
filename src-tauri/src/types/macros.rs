use serde::{Serialize, Serializer};

#[macro_export]
macro_rules! make_serializable {
  (
    $(#[$enum_attr:meta])* // 捕获枚举上的所有属性（如 #[derive(...)]）
      pub enum $enum_name:ident {
          $(
              $variant:ident ($data_ty:ty)
          ),* $(,)?
      }
  ) => {
      $(#[$enum_attr])* // 将属性应用到生成的枚举上
      pub enum $enum_name {
          $(
              $variant($data_ty),
          )*
      }

      impl serde::Serialize for $enum_name {
          fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
          where
              S: serde::Serializer,
          {
              let mut s = serializer.serialize_struct(stringify!($enum_name), 2)?;
              match self {
                  $(
                      $enum_name::$variant(data) => {
                          s.serialize_field("type", stringify!($variant))?;
                          s.serialize_field("data", data)?;
                      }
                  )*
              }
              s.end()
          }
      }
  };
}
