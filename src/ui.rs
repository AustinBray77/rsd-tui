use ratatui::{
    Frame,
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Paragraph},
};

use crate::{
    AppState,
    account::Account,
    state::{COMMAND_COUNT, COMMAND_STRS, UserMessage},
};

pub fn render_ui(frame: &mut Frame<'_>, state: &AppState) {
    let title: Line = Span::styled("RSD-TUI", Style::default().fg(Color::Blue)).into();
    let alert_style: Style = Style::default().fg(Color::Red);
    let info_style: Style = Style::default().fg(Color::Green);
    let default_style: Style = Style::default();
    let highlighted_style: Style = Style::default().bg(Color::White).fg(Color::Black);

    let content: Box<[Line]> = match state {
        AppState::Login { psd: _psd, message } => {
            let password_prompt: Line<'_> =
                Span::styled("Enter password: ", Style::default()).into();

            match message {
                Some(UserMessage::Error(err)) => {
                    [password_prompt, Span::styled(err, alert_style).into()].into()
                }
                Some(UserMessage::Info(info)) => {
                    [password_prompt, Span::styled(info, info_style).into()].into()
                }
                None => [password_prompt].into(),
            }
        }
        AppState::MainScreen {
            clipboard: _clipboard,
            accounts,
            selected_command: Some(command),
            hovering,
            message,
        } => {
            let account_heading = Span::styled(
                format!("For Account: {}", accounts[*hovering]),
                default_style.clone().bold(),
            );

            let mut lines: Vec<Line<'_>> = Vec::new();
            lines.reserve_exact(COMMAND_COUNT + if let Some(_) = message { 2 } else { 1 });

            lines.push(account_heading.into());

            COMMAND_STRS
                .iter()
                .enumerate()
                .map(|(index, command_str): (usize, &&str)| {
                    let style = if index == *command as usize {
                        highlighted_style
                    } else {
                        default_style
                    };

                    Span::styled(command_str.to_string(), style)
                })
                .map(Into::<Line<'_>>::into)
                .for_each(|line| lines.push(line));

            match message {
                Some(UserMessage::Error(err)) => lines.push(Span::styled(err, alert_style).into()),
                Some(UserMessage::Info(info)) => lines.push(Span::styled(info, info_style).into()),
                _ => {}
            }

            lines.into()
        }
        AppState::MainScreen {
            clipboard: _clipboard,
            accounts,
            selected_command: None,
            hovering,
            message: _message,
        } => accounts
            .iter()
            .enumerate()
            .map(|(index, account): (usize, &Account)| {
                let style = if index == *hovering {
                    highlighted_style
                } else {
                    default_style
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
