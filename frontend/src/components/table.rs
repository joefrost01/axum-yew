use yew::prelude::*;

/// Represents sort state for a column
#[derive(Clone, PartialEq, Debug)]
pub enum SortDirection {
    Ascending,
    Descending,
}

/// Properties for the Table component
#[derive(Properties, Clone, PartialEq)]
pub struct TableProps {
    #[prop_or(false)]
    pub loading: bool,
    #[prop_or("No data".to_string())]
    pub no_data_message: String,
    #[prop_or_default]
    pub children: Html,
}

/// A Table component
#[function_component(Table)]
pub fn table(props: &TableProps) -> Html {
    html! {
        <div class="overflow-x-auto">
            <table class="min-w-full divide-y divide-gray-200 dark:divide-gray-700 table-fixed dark:bg-gray-900">
                {props.children.clone()}
            </table>
        </div>
    }
}

/// Properties for the TableHead component
#[derive(Properties, Clone, PartialEq)]
pub struct TableHeadProps {
    #[prop_or_default]
    pub children: Html,
}

/// Table Header component
#[function_component(TableHead)]
pub fn table_head(props: &TableHeadProps) -> Html {
    html! {
        <thead class="bg-gray-50 dark:bg-gray-800">
            {props.children.clone()}
        </thead>
    }
}

/// Properties for the TableBody component
#[derive(Properties, Clone, PartialEq)]
pub struct TableBodyProps {
    #[prop_or(false)]
    pub loading: bool,
    #[prop_or("No data".to_string())]
    pub no_data_message: String,
    #[prop_or(0)]
    pub col_span: usize,
    #[prop_or_default]
    pub children: Html,
    #[prop_or(false)]
    pub empty: bool,
}

/// Table Body component
#[function_component(TableBody)]
pub fn table_body(props: &TableBodyProps) -> Html {
    let content = if props.loading {
        html! {
            <tr>
                <td class="text-center py-4" colspan={props.col_span.to_string()}>
                    <div class="flex justify-center items-center space-x-2 text-gray-500 dark:text-gray-400">
                        <i class="fas fa-spinner fa-spin"></i>
                        <span>{"Loading..."}</span>
                    </div>
                </td>
            </tr>
        }
    } else if props.empty {
        html! {
            <tr>
                <td class="text-center py-4 text-gray-500 dark:text-gray-400" colspan={props.col_span.to_string()}>
                    {&props.no_data_message}
                </td>
            </tr>
        }
    } else {
        props.children.clone()
    };
    
    html! {
        <tbody class="bg-white dark:bg-gray-900 divide-y divide-gray-200 dark:divide-gray-700">
            {content}
        </tbody>
    }
}