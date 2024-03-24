use cosmic::{
    iced::{Length, Padding},
    iced_widget::{column, Row, Scrollable},
    widget::{text, text_input, Button, Column, MouseArea, Space},
    Element,
};

use crate::{db::Data, window::Message};

fn entry_view(data: &Data) -> Element<Message> {
    let content = Row::new()
        .push(text(data.value()))
        .push(Space::with_width(Length::Fill))
        .push(Button::new(text("Delete")).on_press(Message::Delete(data.clone())));

    MouseArea::new(content)
        .on_release(Message::OnClick(data.clone()))
        .into()
}

fn entry_list_view<'a, I>(entries: I) -> Element<'a, Message>
where
    I: Iterator<Item = &'a Data>,
{
    let entries_view = entries.map(|data| entry_view(data));

    let column = Column::with_children(entries_view);

    Scrollable::new(column).into()
}

fn query_view(query: &str) -> Element<Message> {
    text_input("value", query)
        .on_clear(Message::Query("".to_string()))
        .on_input(Message::Query)
        .into()
}

pub fn windows_view<'a, I>(query: &'a str, entries: I) -> Element<'a, Message>
where
    I: Iterator<Item = &'a Data>,
{
    Column::new()
        .push(query_view(query))
        .push(Space::with_height(20))
        .push(entry_list_view(entries))
        .padding(Padding::new(10f32))
        .into()
}
