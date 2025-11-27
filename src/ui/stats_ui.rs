use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::Line,
    widgets::{block::Title, BarChart, Block, Borders, Chart, Dataset, GraphType, List, ListItem, Paragraph, Sparkline},
    Frame,
};

use crate::app::App;

pub fn draw_stats(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    // Left: Stats categories and breakdown
    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(10), Constraint::Min(5)])
        .split(chunks[0]);

    let categories = vec![
        format!("Sessions: {}", app.statistics.total_sessions),
        format!("Minutes: {}", app.statistics.total_minutes),
        format!("Focus Sessions: {}", app.statistics.total_focus_sessions),
        format!("Minutes Focused: {}", app.statistics.total_focus_minutes),
        format!("Break Sessions: {}", app.statistics.total_break_sessions),
        format!("Minutes Resting: {}", app.statistics.total_break_minutes),
        format!("Grown Plants: {}", app.statistics.completed_plants),
        format!("Current Streak: {}", app.garden.current_streak),
        format!("Longest Streak: {}", app.garden.longest_streak),
    ];
    let items: Vec<ListItem> = categories
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let text = if i == app.stats_selected { format!("→ {}", s) } else { format!("  {}", s) };
            let style = if i == app.stats_selected {
                Style::default().fg(app.theme.highlight).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(app.theme.text)
            };
            ListItem::new(text).style(style)
        })
        .collect();
    let list = List::new(items)
        .block(Block::default().title(Title::from(Line::from(" Today's Stats ").style(Style::default().fg(app.theme.blocks))).alignment(Alignment::Center)).borders(Borders::ALL).style(Style::default().fg(app.theme.blocks)));
    f.render_widget(list, left_chunks[0]);

    // Breakdown
    let breakdown_content = match app.stats_selected {
        0 => format!("Focus: {}, Break: {}", app.statistics.total_focus_sessions, app.statistics.total_break_sessions),
        1 => format!("Focused: {}, Break: {}", app.statistics.total_focus_minutes, app.statistics.total_break_minutes),
        2 => {
            if app.statistics.recent_sessions.is_empty() {
                "No session data".to_string()
            } else {
                format!("Session distribution: {:?}", app.statistics.recent_sessions)
            }
        }
        3 => "Minutes focused distribution: N/A".to_string(),
        4 => "Break sessions distribution: N/A".to_string(),
        5 => "Minutes resting distribution: N/A".to_string(),
        6 => "Grown plants distribution: N/A".to_string(),
        7 => format!("Sessions distribution: {:?}", app.statistics.recent_sessions),
        8 => "Minutes distribution: N/A".to_string(),
        _ => "Breakdown not available".to_string(),
    };
    let breakdown = Paragraph::new(breakdown_content)
        .block(Block::default().title(Title::from(Line::from(" Breakdown ").style(Style::default().fg(app.theme.blocks))).alignment(Alignment::Center)).borders(Borders::ALL).style(Style::default().fg(app.theme.blocks)))
        .style(Style::default().fg(app.theme.text));
    f.render_widget(breakdown, left_chunks[1]);

    // Right: Chart
    match app.stats_selected {
        0 => {
            // Sparkline for Sessions
            let data: Vec<u64> = app.statistics.recent_sessions.iter().map(|&x| x as u64).collect();
            let sparkline = Sparkline::default()
                .block(Block::default().title(Line::from(" Sessions ").style(Style::default().fg(app.theme.blocks))).borders(Borders::ALL).style(Style::default().fg(app.theme.blocks)))
                .style(Style::default().fg(app.theme.gauge_running)) // gold
                .data(&data);
            f.render_widget(sparkline, chunks[1]);
        }
        1 => {
            // Chart for Minutes
            let data = vec![(0.0, 0.0), (1.0, app.statistics.total_minutes as f64)];
            let dataset = Dataset::default()
                .data(&data)
                .graph_type(GraphType::Line)
                .style(Style::default().fg(app.theme.gauge_paused)); // rose
            let chart = Chart::new(vec![dataset])
                .block(Block::default().title(Line::from(" Minutes ").style(Style::default().fg(app.theme.blocks))).borders(Borders::ALL).style(Style::default().fg(app.theme.blocks)))
                .x_axis(ratatui::widgets::Axis::default().bounds([0.0, 1.0]))
                .y_axis(ratatui::widgets::Axis::default().bounds([0.0, (app.statistics.total_minutes as f64).max(10.0)]));
            f.render_widget(chart, chunks[1]);
        }
        2 => {
            // Sparkline for Focus Sessions
            let mut data: Vec<u64> = app.statistics.recent_sessions.iter().map(|&x| x as u64).collect();
            if data.is_empty() { data = vec![0, 1, 2, 3, 4, 5]; }
            let sparkline = Sparkline::default()
                .block(Block::default().title(Line::from(" Focus Sessions ").style(Style::default().fg(app.theme.blocks))).borders(Borders::ALL).style(Style::default().fg(app.theme.blocks)))
                .style(Style::default().fg(app.theme.gauge_running)) // gold
                .data(&data);
            f.render_widget(sparkline, chunks[1]);
        }
        3 => {
            // Chart for Minutes Focused
            let data = vec![(0.0, 0.0), (1.0, app.statistics.total_focus_minutes as f64)];
            let dataset = Dataset::default()
                .data(&data)
                .graph_type(GraphType::Line)
                .style(Style::default().fg(app.theme.gauge_paused)); // rose
            let chart = Chart::new(vec![dataset])
                .block(Block::default().title(Line::from(" Minutes Focused ").style(Style::default().fg(app.theme.blocks))).borders(Borders::ALL).style(Style::default().fg(app.theme.blocks)))
                .x_axis(ratatui::widgets::Axis::default().bounds([0.0, 1.0]))
                .y_axis(ratatui::widgets::Axis::default().bounds([0.0, (app.statistics.total_focus_minutes as f64).max(10.0)]));
            f.render_widget(chart, chunks[1]);
        }
        4 => {
            // Sparkline for Break Sessions
            let data = vec![app.statistics.total_break_sessions as u64]; // single point, but use Sparkline
            let sparkline = Sparkline::default()
                .block(Block::default().title(Line::from(" Break Sessions ").style(Style::default().fg(app.theme.blocks))).borders(Borders::ALL).style(Style::default().fg(app.theme.blocks)))
                .style(Style::default().fg(app.theme.sparkline)) // blue
                .data(&data);
            f.render_widget(sparkline, chunks[1]);
        }
        5 => {
            // Chart for Minutes Resting
            let data = vec![(0.0, 0.0), (1.0, app.statistics.total_break_minutes as f64)];
            let dataset = Dataset::default()
                .data(&data)
                .graph_type(GraphType::Line)
                .style(Style::default().fg(app.theme.gauge_paused)); // rose
            let chart = Chart::new(vec![dataset])
                .block(Block::default().title(Line::from(" Minutes Resting ").style(Style::default().fg(app.theme.blocks))).borders(Borders::ALL).style(Style::default().fg(app.theme.blocks)))
                .x_axis(ratatui::widgets::Axis::default().bounds([0.0, 1.0]))
                .y_axis(ratatui::widgets::Axis::default().bounds([0.0, (app.statistics.total_break_minutes as f64).max(10.0)]));
            f.render_widget(chart, chunks[1]);
        }
        6 => {
            // BarChart for Grown Plants
            let data = vec![("Plants", app.statistics.completed_plants as u64)];
            let barchart = BarChart::default()
                .block(Block::default().title(Line::from(" Grown Plants ").style(Style::default().fg(app.theme.blocks))).borders(Borders::ALL).style(Style::default().fg(app.theme.blocks)))
                .data(&data)
                .bar_style(Style::default().fg(app.theme.highlight)) // love
                .value_style(Style::default().fg(app.theme.text));
            f.render_widget(barchart, chunks[1]);
        }
        7 => {
            // Chart for Current Streak
            let data = vec![(0.0, 0.0), (1.0, app.garden.current_streak as f64)];
            let dataset = Dataset::default()
                .data(&data)
                .graph_type(GraphType::Line)
                .style(Style::default().fg(app.theme.highlight)); // accent
            let chart = Chart::new(vec![dataset])
                .block(Block::default().title(Line::from(" Current Streak ").style(Style::default().fg(app.theme.blocks))).borders(Borders::ALL).style(Style::default().fg(app.theme.blocks)))
                .x_axis(ratatui::widgets::Axis::default().bounds([0.0, 1.0]))
                .y_axis(ratatui::widgets::Axis::default().bounds([0.0, (app.garden.current_streak as f64).max(5.0)]));
            f.render_widget(chart, chunks[1]);
        }
        8 => {
            // Paragraph for Longest Streak
            let text = format!("Longest Streak: {}", app.garden.longest_streak);
            let para = Paragraph::new(text)
                .block(Block::default().title(Line::from(" Longest Streak ").style(Style::default().fg(app.theme.blocks))).borders(Borders::ALL).style(Style::default().fg(app.theme.blocks)))
                .style(Style::default().fg(app.theme.text));
            f.render_widget(para, chunks[1]);
        }
        1 => {
            // Chart for Minutes Focused
            let data = vec![(0.0, 0.0), (1.0, app.statistics.total_minutes as f64)];
            let dataset = Dataset::default()
                .data(&data)
                .graph_type(GraphType::Line)
                .style(Style::default().fg(app.theme.gauge_paused)); // rose
            let chart = Chart::new(vec![dataset])
                .block(Block::default().title(Line::from(" Grown Plants ").style(Style::default().fg(app.theme.blocks))).borders(Borders::ALL).style(Style::default().fg(app.theme.blocks)))
                .x_axis(ratatui::widgets::Axis::default().bounds([0.0, 1.0]))
                .y_axis(ratatui::widgets::Axis::default().bounds([0.0, (app.statistics.completed_plants as f64).max(5.0)]));
            f.render_widget(chart, chunks[1]);
        }
        2 => {
            // Sparkline for Break Sessions
            let data = vec![0, 0, 0, 0, 0]; // dummy
            let sparkline = Sparkline::default()
                .block(Block::default().title(Line::from(" Break Sessions ").style(Style::default().fg(app.theme.blocks))).borders(Borders::ALL).style(Style::default().fg(app.theme.blocks)))
                .style(Style::default().fg(app.theme.sparkline)) // blue
                .data(&data);
            f.render_widget(sparkline, chunks[1]);
        }
        3 => {
            // Chart for Minutes Resting
            let data = vec![(0.0, 0.0), (1.0, 0.0)]; // dummy
            let dataset = Dataset::default()
                .data(&data)
                .graph_type(GraphType::Line)
                .style(Style::default().fg(app.theme.gauge_paused)); // rose
            let chart = Chart::new(vec![dataset])
                .block(Block::default().title(Line::from(" Minutes Resting ").style(Style::default().fg(app.theme.blocks))).borders(Borders::ALL).style(Style::default().fg(app.theme.blocks)))
                .x_axis(ratatui::widgets::Axis::default().bounds([0.0, 1.0]))
                .y_axis(ratatui::widgets::Axis::default().bounds([0.0, 10.0]));
            f.render_widget(chart, chunks[1]);
        }
        4 => {
            // BarChart for Grown Plants
            let data = vec![("Plants", app.statistics.completed_plants as u64)];
            let barchart = BarChart::default()
                .block(Block::default().title(Line::from(" Grown Plants ").style(Style::default().fg(app.theme.blocks))).borders(Borders::ALL).style(Style::default().fg(app.theme.blocks)))
                .data(&data)
                .bar_style(Style::default().fg(app.theme.highlight)) // love
                .value_style(Style::default().fg(app.theme.text));
            f.render_widget(barchart, chunks[1]);
        }
        5 => {
            // Chart for Current Streak
            let data = vec![(0.0, 0.0), (1.0, app.garden.current_streak as f64)];
            let dataset = Dataset::default()
                .data(&data)
                .graph_type(GraphType::Line)
                .style(Style::default().fg(app.theme.highlight)); // accent
            let chart = Chart::new(vec![dataset])
                .block(Block::default().title(Line::from(" Current Streak ").style(Style::default().fg(app.theme.blocks))).borders(Borders::ALL).style(Style::default().fg(app.theme.blocks)))
                .x_axis(ratatui::widgets::Axis::default().bounds([0.0, 1.0]))
                .y_axis(ratatui::widgets::Axis::default().bounds([0.0, (app.garden.current_streak as f64).max(5.0)]));
            f.render_widget(chart, chunks[1]);
        }
        6 => {
            // Paragraph for Longest Streak
            let text = format!("Longest Streak: {}", app.garden.longest_streak);
            let para = Paragraph::new(text)
                .block(Block::default().title(Line::from(" Longest Streak ").style(Style::default().fg(app.theme.blocks))).borders(Borders::ALL).style(Style::default().fg(app.theme.blocks)))
                .style(Style::default().fg(app.theme.text));
            f.render_widget(para, chunks[1]);
        }
        7 => {
            // Sparkline for Sessions
            let mut data: Vec<u64> = app.statistics.recent_sessions.iter().map(|&x| x as u64).collect();
            if data.is_empty() { data = vec![0, 1, 2, 3, 4, 5]; }
            let sparkline = Sparkline::default()
                .block(Block::default().title(Line::from(" Sessions ").style(Style::default().fg(app.theme.blocks))).borders(Borders::ALL).style(Style::default().fg(app.theme.blocks)))
                .style(Style::default().fg(app.theme.gauge_running)) // gold
                .data(&data);
            f.render_widget(sparkline, chunks[1]);
        }
        8 => {
            // Chart for Minutes
            let data = vec![(0.0, 0.0), (1.0, app.statistics.total_minutes as f64)];
            let dataset = Dataset::default()
                .data(&data)
                .graph_type(GraphType::Line)
                .style(Style::default().fg(app.theme.gauge_paused)); // rose
            let chart = Chart::new(vec![dataset])
                .block(Block::default().title(Line::from(" Minutes ").style(Style::default().fg(app.theme.blocks))).borders(Borders::ALL).style(Style::default().fg(app.theme.blocks)))
                .x_axis(ratatui::widgets::Axis::default().bounds([0.0, 1.0]))
                .y_axis(ratatui::widgets::Axis::default().bounds([0.0, (app.statistics.total_minutes as f64).max(10.0)]));
            f.render_widget(chart, chunks[1]);
        }
        _ => {}
    }
}