use leptos::*;
use leptos_datatable::*;

fn main() { mount_to_body(|| view! { <App /> })}

#[component]
fn App() -> impl IntoView {
    let columns = columns![
        "Any", DataType::Any;
        "Number", DataType::Number;
        "String", DataType::String
    ];

    let row1 = row![1, 2, "c"];
    let row2 = row!["2", 3, "d"];
    let row3 = row!["a", "s", "d"]; // "s" is wrong!

    let data = RwSignal::from(Data(vec![row1, row2, row3]));

    let increment = move |_| {
        data.with(|d| d.update(0, 1, |v| {
            if let Datum::Number(Some(n)) = v {
                *v = (*n + 1.0).into()
            }
        }))
    };
    
    view! {
        <div class="container">
            <h2>DataTable Example</h2>
            <DataTable
                columns={columns}
                data={data}
                class="table"
            />
            <button type="button" on:click=increment>
                "Add 1 to (row 1, column 2)"
            </button>
            <div>
                <p>
                    "Data types can be specified in column definition. `Number`, `String`,
                    `Any` are available as data types. In above example, the `s` in column
                    Number, row 3 has wrong type, since it's type of `String` while column
                    Number has type of `Number`. Cells with mismatched data types are
                    stressed with red border."
                </p>
                <p>
                    "Styling the table is up to you, since the crate only cares about
                    table data validation and operations. Specify class for the DataTable
                    component and apply styles for table elements such as <tr>, <th>, and
                    <td> through the css class. The `error cell` red border can also be
                    styled with `.error` selector(`!important` property will be needed
                    for border property.). Please see styles.css for an example."
                </p>
            </div>
        </div>
    }
}