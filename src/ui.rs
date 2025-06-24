use ratatui::{
    Frame,
    layout::Constraint,
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, Cell, Row, Table, TableState},
};

use crate::types::{DirectoryView, EntryInfo};

pub fn draw_path_list(
    frame: &mut Frame,
    table_state: &mut TableState,
) {
    let path_env = std::env::var("PATH").unwrap_or_else(|_| "PATH not found".to_string());
    let path_entries: Vec<Row> = path_env
        .split(':')
        .enumerate()
        .map(|(i, path)| Row::new(vec![Cell::from((i + 1).to_string()), Cell::from(path)]))
        .collect();

    let instructions = Line::from(vec![
        " Navigate ".into(),
        "<↑↓>".blue().bold(),
        " Select ".into(),
        "<Enter>".blue().bold(),
        " Quit ".into(),
        "<Q> ".blue().bold(),
    ]);

    let path_table = Table::new(path_entries, [Constraint::Length(4), Constraint::Min(0)])
        .block(Block::bordered()
            .title(" PATH Entries ")
            .title_bottom(instructions.centered())
        )
        .header(Row::new(vec!["#", "Path"]).bold())
        .row_highlight_style(Style::default().bg(Color::Blue).fg(Color::White));

    frame.render_stateful_widget(path_table, frame.area(), table_state);
}

pub fn draw_directory_contents(
    frame: &mut Frame,
    table_state: &mut TableState,
    directory_view: &DirectoryView,
) {
    let contents_to_display = directory_view.get_display_contents();

    let entries: Vec<Row> = contents_to_display
        .iter()
        .enumerate()
        .map(|(i, entry)| {
            let info_text = match &entry.info {
                EntryInfo::SymlinkTarget(target) => format!(" -> {}", target),
                EntryInfo::Error(msg) => msg.clone(),
                EntryInfo::None => String::new(),
            };
            
            Row::new(vec![
                Cell::from((i + 1).to_string()),
                Cell::from(entry.name.clone()),
                Cell::from(if entry.is_symlink { "symlink" } else { "file" }),
                Cell::from(info_text)
            ])
        })
        .collect();

    let mut instructions = vec![
        " Navigate ".into(),
        "<↑↓>".blue().bold(),
        " Search ".into(),
        "</>".blue().bold(),
        " Back ".into(),
        "<Esc>".blue().bold(),
        " Quit ".into(),
        "<Q> ".blue().bold(),
    ];

    if directory_view.search_mode {
        instructions = vec![
            " Type to search, ".into(),
            "<Enter>".blue().bold(),
            " to confirm, ".into(),
            "<Esc>".blue().bold(),
            " to cancel ".into(),
        ];
    }

    let instructions_line = Line::from(instructions);

    let mut title = format!(" Directory: {} ", directory_view.current_path);
    if directory_view.search_mode {
        title = format!(" Search: {} ", directory_view.search_query);
    } else if !directory_view.search_query.is_empty() {
        title = format!(" Directory: {} (filtered: {}) ", directory_view.current_path, directory_view.search_query);
    }

    // Check if there are any error entries
    let has_errors = contents_to_display.iter().any(|entry| 
        matches!(entry.info, EntryInfo::Error(_))
    );
    
    let last_column_header = if has_errors { "Error" } else { "Link Target" };

    let dir_table = Table::new(entries, [
        Constraint::Length(4),
        Constraint::Min(20),
        Constraint::Length(8),
        Constraint::Min(0)
    ])
        .block(Block::bordered()
            .title(title)
            .title_bottom(instructions_line.centered())
        )
        .header(Row::new(vec!["#", "Name", "Type", last_column_header]).bold())
        .row_highlight_style(Style::default().bg(Color::Blue).fg(Color::White));

    frame.render_stateful_widget(dir_table, frame.area(), table_state);
}