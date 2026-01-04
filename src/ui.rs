use ratatui::{
    Frame,
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Paragraph},
};

use crate::{AppState, types::Account};

pub fn render_ui(frame: &mut Frame<'_>, state: &AppState) {
    let title: Line = Span::styled("RSD-TUI", Style::default().fg(Color::Blue)).into();
    let alert_style: Style = Style::default().fg(Color::Red);

    let content: Box<[Line]> = match state {
        AppState::Login { psd: _psd, alert } => {
            let password_prompt: Line<'_> =
                Span::styled("Enter password: ", Style::default()).into();

            match alert {
                Some(err) => [password_prompt, Span::styled(err, alert_style).into()].into(),
                None => [password_prompt].into(),
            }
        }
        AppState::MainScreen { accounts, selected } => accounts
            .iter()
            .enumerate()
            .map(|(index, account): (usize, &Account)| {
                let style = if index == *selected {
                    Style::default().bg(Color::White).fg(Color::Black)
                } else {
                    Style::default()
                };

                Span::styled(format!("{}) {}", index, account), style)
            })
            .map(Into::<Line>::into)
            .collect::<Box<[Line]>>(),
        AppState::Exit => [Span::styled("Exiting...", Style::default()).into()].into(),
    };

    let text = Text::from([[title].as_slice(), content.iter().as_slice()].concat());

    frame.render_widget(Paragraph::new(text).block(Block::bordered()), frame.area());

    //frame.render_widget("Enter Password:", frame.area());
}
