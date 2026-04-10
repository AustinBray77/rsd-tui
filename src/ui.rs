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
    theme::Theme,
};

const TITLE: &'static str = "RSD-TUI";

pub fn render_ui(frame: &mut Frame<'_>, state: &AppState, theme: &Theme) {
    let alert_st: Style = theme.alert.clone().into();
    let info_st: Style = theme.info.clone().into();
    let main_st: Style = theme.main.clone().into();
    let highlight_st: Style = theme.highlight.clone().into();
    let border_st: Style = theme.border.clone().into();

    let title: Line = Span::styled(TITLE, theme.title.clone()).into();

    let content: Box<[Line]> = match state {
        AppState::Login { psd: _psd, message } => {
            let password_prompt: Line<'_> = Span::styled("Enter password: ", main_st).into();

            match message {
                Some(UserMessage::Error(err)) => {
                    [password_prompt, Span::styled(err, alert_st).into()].into()
                }
                Some(UserMessage::Info(info)) => {
                    [password_prompt, Span::styled(info, info_st).into()].into()
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
                main_st.clone(),
            );

            let mut lines: Vec<Line<'_>> = Vec::new();
            lines.reserve_exact(COMMAND_COUNT + if let Some(_) = message { 2 } else { 1 });

            lines.push(account_heading.into());

            COMMAND_STRS
                .iter()
                .enumerate()
                .map(|(index, command_str): (usize, &&str)| {
                    let style = if index == *command as usize {
                        highlight_st.clone()
                    } else {
                        main_st.clone()
                    };

                    Span::styled(command_str.to_string(), style)
                })
                .map(Into::<Line<'_>>::into)
                .for_each(|line| lines.push(line));

            match message {
                Some(UserMessage::Error(err)) => lines.push(Span::styled(err, alert_st).into()),
                Some(UserMessage::Info(info)) => lines.push(Span::styled(info, info_st).into()),
                _ => {}
            }

            lines.into()
        }
        AppState::MainScreen {
            clipboard: _clipboard,
            accounts,
            selected_command: None,
            hovering,
            message,
        } => accounts
            .iter()
            .enumerate()
            .map(|(index, account): (usize, &Account)| {
                let style = if index == *hovering {
                    highlight_st.clone()
                } else {
                    main_st.clone()
                };

                Span::styled(format!("{}) {}", index, account), style)
            })
            .map(Into::<Line>::into)
            .chain([match message {
                Some(UserMessage::Error(err)) => Span::styled(err, alert_st).into(),
                Some(UserMessage::Info(info)) => Span::styled(info, info_st).into(),
                _ => Span::default(),
            }
            .into()])
            .collect::<Box<[Line]>>(),
        AppState::Exit => [Span::styled("Exiting...", Style::default()).into()].into(),
    };

    let text = Text::from([[title].as_slice(), content.iter().as_slice()].concat());

    frame.render_widget(
        Paragraph::new(text).block(Block::bordered().border_style(border_st)),
        frame.area(),
    );

    //frame.render_widget("Enter Password:", frame.area());
}
