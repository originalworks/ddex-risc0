#[macro_use]
extern crate yaserde;
#[macro_use]
extern crate yaserde_derive;

#[test]
fn basic_enum() {
  #[derive(Debug, PartialEq, YaDeserialize, YaSerialize)]
  #[yaserde(rename = "base")]
  pub struct XmlStruct {
    color: Color,
  }

  #[derive(Debug, PartialEq, YaDeserialize, YaSerialize)]
  #[yaserde(rename = "color")]
  #[derive(Default)]
  pub enum Color {
    #[default]
    White,
    Black,
    #[yaserde(rename = "custom")]
    Custom {
      enabled: String,
      u8_value: u8,
      i8_value: i8,
      u16_value: u16,
      i16_value: i16,
      u32_value: u32,
      i32_value: i32,
      u64_value: u64,
      i64_value: i64,
      f32_value: f32,
      f64_value: f64,
      color: RGBColor,
      alpha: Alpha,
      alphas: Vec<Alpha>,
    },
  }

  assert_eq!(Color::default(), Color::White);

  #[derive(Debug, PartialEq, YaDeserialize, YaSerialize)]
  pub struct RGBColor {
    red: String,
    green: String,
    blue: String,
  }

  #[derive(Debug, PartialEq, YaDeserialize, YaSerialize, Default)]
  pub enum Alpha {
    #[default]
    Transparent,
    Opaque,
  }

  let model = XmlStruct {
    color: Color::Black,
  };

  let content = "<base><color>Black</color></base>";
  serialize_and_validate!(model, content);
  deserialize_and_validate!(content, model, XmlStruct);

  let model = XmlStruct {
    color: Color::Custom {
      enabled: "true".to_string(),
      u8_value: 8,
      i8_value: -8,
      u16_value: 16,
      i16_value: -16,
      u32_value: 32,
      i32_value: -32,
      u64_value: 64,
      i64_value: -64,
      f32_value: 32.0,
      f64_value: 64.0,
      color: RGBColor {
        red: "0".to_string(),
        green: "128".to_string(),
        blue: "255".to_string(),
      },
      alpha: Alpha::Opaque,
      alphas: vec![Alpha::Opaque, Alpha::Transparent],
    },
  };

  let content = r#"
<base>
<color><enabled>true</enabled>
<u8_value>8</u8_value>
<i8_value>-8</i8_value>
<u16_value>16</u16_value>
<i16_value>-16</i16_value>
<u32_value>32</u32_value>
<i32_value>-32</i32_value>
<u64_value>64</u64_value>
<i64_value>-64</i64_value>
<f32_value>32</f32_value>
<f64_value>64</f64_value>
<color><red>0</red><green>128</green><blue>255</blue></color>
<alpha>Opaque</alpha>
<alphas>Opaque</alphas>
<alphas>Transparent</alphas>
</color>
</base>"#;

  serialize_and_validate!(model, content);
  // TODO
  // deserialize_and_validate!(content, model, XmlStruct);
}

#[test]
fn attribute_enum() {
  #[derive(Debug, PartialEq, YaDeserialize, YaSerialize)]
  #[yaserde(rename = "base")]
  pub struct XmlStruct {
    #[yaserde(attribute)]
    color: Color,
  }

  #[derive(Debug, PartialEq, YaDeserialize, YaSerialize)]
  #[yaserde(rename = "color")]
  #[derive(Default)]
  pub enum Color {
    #[yaserde(rename = "pink")]
    #[default]
    Pink,
  }

  let model = XmlStruct { color: Color::Pink };

  let content = r#"<base color="pink" />"#;
  serialize_and_validate!(model, content);
  deserialize_and_validate!(content, model, XmlStruct);
}

#[test]
fn attribute_enum2() {
  #[derive(Debug, PartialEq, YaSerialize, YaDeserialize)]
  #[yaserde(rename = "child1")]
  struct Child1 {
    #[yaserde(attribute, rename = "val")]
    pub val: String,
  }

  impl Default for Child1 {
    fn default() -> Child1 {
      Child1 {
        val: "hello world".into(),
      }
    }
  }

  #[derive(Debug, PartialEq, YaSerialize, YaDeserialize)]
  #[yaserde(rename = "child2")]
  #[derive(Default)]
  struct Child2 {
    #[yaserde(attribute)]
    pub num: u8,
  }

  #[derive(Debug, PartialEq, YaSerialize, YaDeserialize)]
  #[yaserde(flatten)]
  enum Base {
    #[yaserde(flatten, rename = "child1")]
    C1(Child1),
    #[yaserde(flatten, rename = "child2")]
    C2(Child2),
  }

  impl Default for Base {
    fn default() -> Base {
      Base::C1(Child1 {
        val: "hello world".into(),
      })
    }
  }

  let content = r#"<child1 val="hello world" />"#;
  let model = Base::C1(Child1 {
    val: "hello world".into(),
  });

  serialize_and_validate!(model, content);
  deserialize_and_validate!(content, model, Base);

  let content = r#"<child2 num="7" />"#;
  let model = Base::C2(Child2 { num: 7 });

  serialize_and_validate!(model, content);
  deserialize_and_validate!(content, model, Base);

  #[derive(Debug, PartialEq, YaSerialize, YaDeserialize)]
  #[yaserde(rename = "base")]
  enum Base2 {
    #[yaserde(flatten)]
    C1(Child1),
  }

  impl Default for Base2 {
    fn default() -> Base2 {
      Base2::C1(Child1 {
        val: "hello world".into(),
      })
    }
  }

  let content = r#"<base><child1 val="hello world" /></base>"#;
  let model = Base2::C1(Child1 {
    val: "hello world".into(),
  });
  serialize_and_validate!(model, content);
  println!("{:?}", yaserde::de::from_str::<Base2>(content));
  deserialize_and_validate!(content, model, Base2);
}

#[test]
fn unnamed_enum() {
  #[derive(Debug, PartialEq, YaDeserialize, YaSerialize)]
  #[yaserde(rename = "base")]
  pub struct XmlStruct {
    color: Enum,
  }

  #[derive(Debug, PartialEq, YaDeserialize, YaSerialize)]
  pub struct OtherStruct {
    fi: i32,
    se: i32,
  }

  #[derive(Debug, PartialEq, YaDeserialize, YaSerialize, Default)]
  pub enum Enum {
    #[default]
    Simple,
    Field(String),
    FullPath(String),
    Integer(i32),
    UserStruct(OtherStruct),
    OptionString(Option<String>),
    OptionUserStruct(Option<OtherStruct>),
    Strings(Vec<String>),
    Ints(Vec<i32>),
    Structs(Vec<OtherStruct>),
    #[yaserde(rename = "renamed")]
    ToRename(u32),
    #[yaserde(rename = "renamed.with.dots")]
    ToRenameDots(u32),
  }

  let model = XmlStruct {
    color: Enum::Field("some_text".to_owned()),
  };

  let content = "<base><color><Field>some_text</Field></color></base>";
  serialize_and_validate!(model, content);
  deserialize_and_validate!(content, model, XmlStruct);

  let model = XmlStruct {
    color: Enum::FullPath("some_text".to_owned()),
  };

  let content = "<base><color><FullPath>some_text</FullPath></color></base>";
  serialize_and_validate!(model, content);
  deserialize_and_validate!(content, model, XmlStruct);

  let model = XmlStruct {
    color: Enum::Integer(56),
  };

  let content = "<base><color><Integer>56</Integer></color></base>";
  serialize_and_validate!(model, content);
  deserialize_and_validate!(content, model, XmlStruct);

  let model = XmlStruct {
    color: Enum::UserStruct(OtherStruct { fi: 24, se: 42 }),
  };

  let content = "<base><color><UserStruct><fi>24</fi><se>42</se></UserStruct></color></base>";
  serialize_and_validate!(model, content);
  deserialize_and_validate!(content, model, XmlStruct);

  let model = XmlStruct {
    color: Enum::OptionString(Some("some_text".to_owned())),
  };

  let content = "<base><color><OptionString>some_text</OptionString></color></base>";
  serialize_and_validate!(model, content);
  deserialize_and_validate!(content, model, XmlStruct);

  let model = XmlStruct {
    color: Enum::OptionString(None),
  };

  let content = "<base><color /></base>";
  serialize_and_validate!(model, content);
  // TODO
  // deserialize_and_validate!(content, model, XmlStruct);

  let model = XmlStruct {
    color: Enum::OptionUserStruct(Some(OtherStruct { fi: 12, se: 23 })),
  };

  let content =
    "<base><color><OptionUserStruct><fi>12</fi><se>23</se></OptionUserStruct></color></base>";
  serialize_and_validate!(model, content);
  deserialize_and_validate!(content, model, XmlStruct);

  let model = XmlStruct {
    color: Enum::OptionUserStruct(None),
  };

  let content = "<base><color /></base>";
  serialize_and_validate!(model, content);
  // TODO
  // deserialize_and_validate!(content, model, XmlStruct);

  let model = XmlStruct {
    color: Enum::Strings(vec!["abc".to_owned(), "def".to_owned()]),
  };

  let content = "<base><color><Strings>abc</Strings><Strings>def</Strings></color></base>";
  serialize_and_validate!(model, content);
  deserialize_and_validate!(content, model, XmlStruct);

  let model = XmlStruct {
    color: Enum::Ints(vec![23, 45]),
  };

  let content = "<base><color><Ints>23</Ints><Ints>45</Ints></color></base>";
  serialize_and_validate!(model, content);
  deserialize_and_validate!(content, model, XmlStruct);

  let model = XmlStruct {
    color: Enum::Structs(vec![
      OtherStruct { fi: 12, se: 23 },
      OtherStruct { fi: 34, se: 45 },
    ]),
  };

  let content = "<base><color><Structs><fi>12</fi><se>23</se></Structs><Structs><fi>34</fi><se>45</se></Structs></color></base>";
  serialize_and_validate!(model, content);
  deserialize_and_validate!(content, model, XmlStruct);

  let model = XmlStruct {
    color: Enum::ToRename(87),
  };

  let content = "<base><color><renamed>87</renamed></color></base>";
  serialize_and_validate!(model, content);
  deserialize_and_validate!(content, model, XmlStruct);

  let model = XmlStruct {
    color: Enum::ToRenameDots(84),
  };

  let content = "<base><color><renamed.with.dots>84</renamed.with.dots></color></base>";
  serialize_and_validate!(model, content);
  deserialize_and_validate!(content, model, XmlStruct);
}

#[test]
fn tagged_enum() {
  #[derive(Debug, PartialEq, YaSerialize, YaDeserialize, Default)]
  #[yaserde(tag = "type")]
  #[yaserde(rename = "foobar")]
  enum XmlEnum {
    #[default]
    #[yaserde(rename = "foo")]
    Foo,
    #[yaserde(rename = "bar")]
    Bar,
  }

  #[derive(Debug, PartialEq, YaSerialize, YaDeserialize, Default)]
  #[yaserde(rename = "base")]
  struct XmlStruct {
    #[yaserde(rename = "foobar")]
    foo_bar: XmlEnum,
  }

  let model = XmlEnum::Foo;
  let content = "<foobar type=\"foo\" />";
  serialize_and_validate!(model, content);
  deserialize_and_validate!(content, model, XmlEnum);

  let model = XmlEnum::Bar;
  let content = "<foobar type=\"bar\" />";
  serialize_and_validate!(model, content);
  deserialize_and_validate!(content, model, XmlEnum);

  let model = XmlStruct {
    foo_bar: XmlEnum::Foo,
  };
  let content = "<base><foobar type=\"foo\" /></base>";
  serialize_and_validate!(model, content);
  deserialize_and_validate!(content, model, XmlStruct);
}
