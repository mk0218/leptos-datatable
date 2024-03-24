use leptos::IntoView;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DataType {
    String,
    Number,
    Any,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Column {
    pub name: String,
    pub r#type: DataType,
}

#[macro_export]
macro_rules! columns {
    [$($n: expr),+] => {
        vec![$(Column {
            name: $n.into(),
            r#type: DataType::Any
        }),+]
    };
    [$($n: expr, $t: expr);+] => {
        vec![$(Column {
            name: $n.into(),
            r#type: $t,
        }),+]
    };
    [$t: expr; $c: expr] => {
        vec![Column {
            name: String::new(),
            r#type: $t
        }; $c]
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Datum {
    String(Option<String>),
    Number(Option<f64>),
}

impl From<String> for Datum {
    fn from(value: String) -> Self {
        Datum::String(Some(value))
    }
}

impl From<&str> for Datum {
    fn from(value: &str) -> Self {
        Datum::String(Some(value.into()))
    }
}

impl From<i32> for Datum {
    fn from(value: i32) -> Self {
        Datum::Number(Some(value.into()))
    }
}

impl IntoView for Datum {
    fn into_view(self) -> leptos::View {
        match self {
            Datum::String(Some(d)) => d.into_view(),
            Datum::Number(Some(d)) => d.into_view(),
            _ => "".into_view(),
        }
    }
}

impl From<f64> for Datum {
    fn from(value: f64) -> Self {
        Datum::Number(Some(value))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Row(pub Vec<Datum>);

#[macro_export]
macro_rules! row {
    [$($v: expr),+] => {
        Row(vec![$($v.into()),+])
    };
}

impl Row {
    pub fn unwrap(&self) -> Vec<Datum> {
        match self {
            Row(row) => row.to_vec()
        }
    }
}

#[derive(Clone)]
pub struct Data(pub Vec<Row>);

impl Data {
    pub fn unwrap(&self) -> Vec<Row> {
        match self {
            Data(rows) => rows.to_vec()
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macro_columns_1() {
        let columns = columns!["A", "B", "C"];
        assert_eq!(columns, vec![
            Column { name: "A".into(), r#type: DataType::Any },
            Column { name: "B".into(), r#type: DataType::Any },
            Column { name: "C".into(), r#type: DataType::Any },
        ])
    }

    #[test]
    fn test_macro_columns_2() {
        let columns = columns!["A", DataType::String; "B", DataType::Number];
        assert_eq!(columns, vec![
            Column { name: "A".into(), r#type: DataType::String },
            Column { name: "B".into(), r#type: DataType::Number },
        ])
    }

    #[test]
    fn test_macro_columns_3() {
        let columns = columns![DataType::Number; 3];
        assert_eq!(columns, vec![
            Column { name: String::new(), r#type: DataType::Number },
            Column { name: String::new(), r#type: DataType::Number },
            Column { name: String::new(), r#type: DataType::Number },
        ])
    }

    #[test]
    fn test_macro_row() {
        let row = row![1, "A", 2, "B", 3.1, "c", "d"];
        assert_eq!(row, Row(vec![
            Datum::Number(Some(1.0)),
            Datum::String(Some("A".into())),
            Datum::Number(Some(2.0)),
            Datum::String(Some("B".into())),
            Datum::Number(Some(3.1)),
            Datum::String(Some("c".into())),
            Datum::String(Some("d".into())),
        ]))
    }
}