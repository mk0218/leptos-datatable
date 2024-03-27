
use leptos::{component, leptos_dom::logging::console_log, view, CollectView, IntoView, RwSignal, SignalGet};
use crate::{Column, Data, DataType, Datum, Row};

#[derive(Debug)]
struct DataTypeMismatch((), ());

#[component]
fn TCell(r#type: DataType, datum: RwSignal<Datum>) -> impl IntoView {
    let valid = match (datum.get(), r#type) {
        (Datum::Number(_), DataType::Number) |
        (Datum::String(_), DataType::String) |
        (_, DataType::Any) => Ok(()),
        (_, _t) => Err(DataTypeMismatch((), ()))
    };

    let err_style = "
        position: absolute; 
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        border: 1px solid red
    ";

    match valid {
        Ok(_) => view! { <td>{datum}</td> },
        Err(_) => view! {
            <td style="position: relative">
            {datum}
            <div
                class="error"
                style={err_style} />
            </td>
        },
    }
}

#[component]
/// Renders table row 
fn TRow<'a>(columns: &'a Vec<Column>, row: Row) -> impl IntoView {
    view! {
        <tr>
            {row.unwrap().into_iter().zip(columns.iter()).map(|(d, c)| {
                let &Column { r#type: t, .. } = c;
                view! { <TCell r#type={t} datum={d} /> }
            }).collect_view()}
        </tr>
    }
}

#[component]
pub fn DataTable(
    columns: Vec<Column>,
    data: RwSignal<Data>,
    #[prop(optional, into)]
    class: String,
) -> impl IntoView {
    view! {
        <table class={class}>
            <thead>
                <tr>
                    {columns.iter().map(|column| {
                        view! { <th>{column.name.clone()}</th> }
                    }).collect_view()}
                </tr>
            </thead>
            <tbody>
                {data.get().unwrap().into_iter().map(|row| {
                    view! { <TRow columns={&columns} row={row} /> }
                }).collect_view()}
            </tbody>
        </table>
    }
}
