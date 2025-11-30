use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

use crate::app::App;

pub fn draw_settings(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    // Left: Settings list
    let theme_name = match app.settings.theme {
        crate::theme::ThemeVariant::System => "System",
        crate::theme::ThemeVariant::RosePineDawn => "Rose Pine Dawn",
        crate::theme::ThemeVariant::RosePine => "Rose Pine",
        crate::theme::ThemeVariant::GruvboxDark => "Gruvbox Dark",
        crate::theme::ThemeVariant::GruvboxLight => "Gruvbox Light",
        crate::theme::ThemeVariant::SolarizedDark => "Solarized Dark",
        crate::theme::ThemeVariant::SolarizedLight => "Solarized Light",
        crate::theme::ThemeVariant::Nord => "Nord",
        crate::theme::ThemeVariant::TokyoNight => "Tokyo Night",
        crate::theme::ThemeVariant::Monokai => "Monokai",
        crate::theme::ThemeVariant::Vesper => "Vesper",
        crate::theme::ThemeVariant::Everforest => "Everforest",
        crate::theme::ThemeVariant::CatppuccinLatte => "Catppuccin Latte",
        crate::theme::ThemeVariant::CatppuccinFrappe => "Catppuccin Frappé",
        crate::theme::ThemeVariant::CatppuccinMacchiato => "Catppuccin Macchiato",
        crate::theme::ThemeVariant::CatppuccinMocha => "Catppuccin Mocha",
    };
    let settings = [format!("Focus Duration: {} min", app.settings.focus_duration),
        format!("Short Break: {} min", app.settings.short_break_duration),
        format!("Long Break: {} min", app.settings.long_break_duration),
        format!("Theme: {}", theme_name)];
    let items: Vec<ListItem> = settings
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let is_selected = i == app.settings_selected && app.focus == crate::app::Focus::Left;
            let text = if is_selected { format!("→ {}", s) } else { format!("  {}", s) };
            let style = if is_selected {
                Style::default().fg(app.theme.highlight).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(app.theme.text)
            };
            ListItem::new(text).style(style)
        })
        .collect();
    let list = List::new(items)
        .block(Block::default().title_top(Line::from(" Settings ").style(Style::default().fg(app.theme.blocks)).centered()).borders(Borders::ALL).style(Style::default().fg(app.theme.blocks)));
    f.render_widget(list, chunks[0]);

    // Right: Adjustment
    let is_focused = app.focus == crate::app::Focus::Right;
    let right_items: Vec<ListItem> = match app.settings_selected {
        0 => {
            let mut style = Style::default().fg(app.theme.text);
            if is_focused {
                style = style.add_modifier(Modifier::BOLD);
            }
            vec![ListItem::new(Line::from(vec![
                Span::styled("  ", style),
                Span::styled(format!("{}", app.settings.focus_duration), Style::default().fg(app.theme.highlight)),
                Span::styled(" min", style),
            ])).style(style)]
        }
        1 => {
            let mut style = Style::default().fg(app.theme.text);
            if is_focused {
                style = style.add_modifier(Modifier::BOLD);
            }
            vec![ListItem::new(Line::from(vec![
                Span::styled("  ", style),
                Span::styled(format!("{}", app.settings.short_break_duration), Style::default().fg(app.theme.highlight)),
                Span::styled(" min", style),
            ])).style(style)]
        }
        2 => {
            let mut style = Style::default().fg(app.theme.text);
            if is_focused {
                style = style.add_modifier(Modifier::BOLD);
            }
            vec![ListItem::new(Line::from(vec![
                Span::styled("  ", style),
                Span::styled(format!("{}", app.settings.long_break_duration), Style::default().fg(app.theme.highlight)),
                Span::styled(" min", style),
            ])).style(style)]
        }
        3 => {
            let themes = ["System", "Rose Pine Dawn", "Rose Pine", "Gruvbox Dark", "Gruvbox Light", "Solarized Dark", "Solarized Light", "Nord", "Tokyo Night", "Monokai", "Vesper", "Everforest", "Catppuccin Latte", "Catppuccin Frappé", "Catppuccin Macchiato", "Catppuccin Mocha"];
            let current_index = match app.settings.theme {
                crate::theme::ThemeVariant::System => 0,
                crate::theme::ThemeVariant::RosePineDawn => 1,
                crate::theme::ThemeVariant::RosePine => 2,
                crate::theme::ThemeVariant::GruvboxDark => 3,
                crate::theme::ThemeVariant::GruvboxLight => 4,
                crate::theme::ThemeVariant::SolarizedDark => 5,
                crate::theme::ThemeVariant::SolarizedLight => 6,
                crate::theme::ThemeVariant::Nord => 7,
                crate::theme::ThemeVariant::TokyoNight => 8,
                crate::theme::ThemeVariant::Monokai => 9,
                crate::theme::ThemeVariant::Vesper => 10,
                crate::theme::ThemeVariant::Everforest => 11,
                crate::theme::ThemeVariant::CatppuccinLatte => 12,
                crate::theme::ThemeVariant::CatppuccinFrappe => 13,
                crate::theme::ThemeVariant::CatppuccinMacchiato => 14,
                crate::theme::ThemeVariant::CatppuccinMocha => 15,
            };
            themes.iter().enumerate().map(|(i, &theme)| {
                let prefix = if i == current_index { "→ " } else { "  " };
                let style = if i == current_index && is_focused {
                    Style::default().fg(app.theme.highlight).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(app.theme.text)
                };
                ListItem::new(format!("{}{}", prefix, theme)).style(style)
            }).collect()
        }
        _ => vec![],
    };
    let right_list = List::new(right_items)
        .block(Block::default().title_top(Line::from(" Adjust ").style(Style::default().fg(app.theme.blocks)).centered()).borders(Borders::ALL).style(Style::default().fg(app.theme.blocks)));
    f.render_widget(right_list, chunks[1]);
}