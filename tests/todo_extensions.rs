use norgmill::renderer::parse_and_render_norg;

// Basic TODO Status Tests
#[test]
fn test_todo_undone_basic() {
    let norg = r#"- ( ) Undone task"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse basic undone TODO");
    assert!(result.contains("<ul>"));
    assert!(result.contains("<li>"));
    assert!(result.contains("Undone task"));
    assert!(result.contains("</li>"));
    assert!(result.contains("</ul>"));
    // Should contain some form of checkbox or task indicator
    assert!(result.contains("checkbox") || result.contains("task") || result.contains("todo") || result.contains("☐"));
}

#[test]
fn test_todo_done_basic() {
    let norg = r#"- (x) Done task"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse basic done TODO");
    assert!(result.contains("<ul>"));
    assert!(result.contains("<li>"));
    assert!(result.contains("Done task"));
    assert!(result.contains("</li>"));
    assert!(result.contains("</ul>"));
    // Should contain some form of checked checkbox or completed task indicator
    assert!(result.contains("checked") || result.contains("completed") || result.contains("done") || result.contains("☑") || result.contains("✓"));
}

#[test]
fn test_todo_pending_basic() {
    let norg = r#"- (-) Pending task"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse basic pending TODO");
    assert!(result.contains("<ul>"));
    assert!(result.contains("<li>"));
    assert!(result.contains("Pending task"));
    assert!(result.contains("</li>"));
    assert!(result.contains("</ul>"));
    // Should contain some form of pending indicator
    assert!(result.contains("pending") || result.contains("waiting") || result.contains("⏳") || result.contains("-"));
}

#[test]
fn test_todo_cancelled_basic() {
    let norg = r#"- (_) Cancelled task"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse basic cancelled TODO");
    assert!(result.contains("<ul>"));
    assert!(result.contains("<li>"));
    assert!(result.contains("Cancelled task"));
    assert!(result.contains("</li>"));
    assert!(result.contains("</ul>"));
    // Should contain some form of cancelled indicator
    assert!(result.contains("cancelled") || result.contains("canceled") || result.contains("✗") || result.contains("_"));
}

#[test]
fn test_todo_urgent_basic() {
    let norg = r#"- (!) Urgent task"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse basic urgent TODO");
    assert!(result.contains("<ul>"));
    assert!(result.contains("<li>"));
    assert!(result.contains("Urgent task"));
    assert!(result.contains("</li>"));
    assert!(result.contains("</ul>"));
    // Should contain some form of urgent indicator
    assert!(result.contains("urgent") || result.contains("important") || result.contains("!") || result.contains("⚠"));
}

#[test]
fn test_todo_question_basic() {
    let norg = r#"- (?) Question or uncertain task"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse basic question TODO");
    assert!(result.contains("<ul>"));
    assert!(result.contains("<li>"));
    assert!(result.contains("Question or uncertain task"));
    assert!(result.contains("</li>"));
    assert!(result.contains("</ul>"));
    // Should contain some form of question indicator
    assert!(result.contains("question") || result.contains("uncertain") || result.contains("?") || result.contains("❓"));
}

#[test]
fn test_todo_on_hold_basic() {
    let norg = r#"- (=) On hold task"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse basic on hold TODO");
    assert!(result.contains("<ul>"));
    assert!(result.contains("<li>"));
    assert!(result.contains("On hold task"));
    assert!(result.contains("</li>"));
    assert!(result.contains("</ul>"));
    // Should contain some form of on hold indicator
    assert!(result.contains("hold") || result.contains("paused") || result.contains("=") || result.contains("⏸"));
}

#[test]
fn test_todo_recurring_basic() {
    let norg = r#"- (+) Recurring task"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse basic recurring TODO");
    assert!(result.contains("<ul>"));
    assert!(result.contains("<li>"));
    assert!(result.contains("Recurring task"));
    assert!(result.contains("</li>"));
    assert!(result.contains("</ul>"));
    // Should contain some form of recurring indicator
    assert!(result.contains("recurring") || result.contains("repeat") || result.contains("+") || result.contains("🔄"));
}

// Multiple TODO Items Tests
#[test]
fn test_multiple_todo_mixed_statuses() {
    let norg = r#"- ( ) First undone task
- (x) First done task
- (-) First pending task
- (_) First cancelled task
- (!) First urgent task
- (?) First question task
- (=) First on hold task
- (+) First recurring task"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse multiple TODO items with mixed statuses");
    assert!(result.contains("<ul>"));
    assert!(result.contains("First undone task"));
    assert!(result.contains("First done task"));
    assert!(result.contains("First pending task"));
    assert!(result.contains("First cancelled task"));
    assert!(result.contains("First urgent task"));
    assert!(result.contains("First question task"));
    assert!(result.contains("First on hold task"));
    assert!(result.contains("First recurring task"));
    assert!(result.contains("</ul>"));
}

#[test]
fn test_todo_with_markup() {
    let norg = r#"- (x) Completed task with *bold* and /italic/ text
- ( ) Undone task with `code` and _underline_
- (!) Urgent task with {https://example.com}[link]"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse TODO items with markup");
    assert!(result.contains("<ul>"));
    assert!(result.contains("Completed task with <strong>bold</strong> and <em>italic</em> text"));
    assert!(result.contains("Undone task with <code>code</code> and <u>underline</u>"));
    assert!(result.contains("Urgent task with"));
    assert!(result.contains("href=\"https://example.com\""));
    assert!(result.contains("link</a>"));
    assert!(result.contains("</ul>"));
}

#[test]
fn test_todo_multiline_content() {
    let norg = r#"- ( ) This is a multi-line task
  that spans several lines
  and contains detailed information
- (x) This is another task
  with multiple lines of content"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse TODO items with multiline content");
    assert!(result.contains("<ul>"));
    assert!(result.contains("This is a multi-line task"));
    assert!(result.contains("that spans several lines"));
    assert!(result.contains("and contains detailed information"));
    assert!(result.contains("This is another task"));
    assert!(result.contains("with multiple lines of content"));
    assert!(result.contains("</ul>"));
}

#[test]
fn test_todo_nested_lists() {
    let norg = r#"- (x) Main task completed
  - ( ) Sub-task 1 undone
  - (x) Sub-task 2 done
  - (-) Sub-task 3 pending
- ( ) Another main task
  - (!) Urgent sub-task
  - (?) Question sub-task"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse nested TODO lists");
    assert!(result.contains("<ul>"));
    assert!(result.contains("Main task completed"));
    assert!(result.contains("Sub-task 1 undone"));
    assert!(result.contains("Sub-task 2 done"));
    assert!(result.contains("Sub-task 3 pending"));
    assert!(result.contains("Another main task"));
    assert!(result.contains("Urgent sub-task"));
    assert!(result.contains("Question sub-task"));
    assert!(result.contains("</ul>"));
}

#[test]
fn test_todo_mixed_with_regular_list() {
    let norg = r#"- Regular list item
- (x) Completed task
- Another regular item
- ( ) Undone task
- Final regular item"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse TODO items mixed with regular list");
    assert!(result.contains("<ul>"));
    assert!(result.contains("Regular list item"));
    assert!(result.contains("Completed task"));
    assert!(result.contains("Another regular item"));
    assert!(result.contains("Undone task"));
    assert!(result.contains("Final regular item"));
    assert!(result.contains("</ul>"));
}

// TODO in Different Contexts Tests
#[test]
fn test_todo_in_ordered_list() {
    let norg = r#"~ (x) First completed task
~ ( ) Second undone task
~ (-) Third pending task"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse TODO items in ordered list");
    assert!(result.contains("<ol>"));
    assert!(result.contains("First completed task"));
    assert!(result.contains("Second undone task"));
    assert!(result.contains("Third pending task"));
    assert!(result.contains("</ol>"));
}

#[test]
fn test_todo_in_paragraph() {
    let norg = r#"This paragraph contains a TODO item: (x) inline completed task.
Another paragraph with ( ) inline undone task."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse TODO items in paragraph");
    assert!(result.contains("<p>This paragraph contains a TODO item:"));
    assert!(result.contains("inline completed task.</p>"));
    assert!(result.contains("<p>Another paragraph with"));
    assert!(result.contains("inline undone task.</p>"));
}

#[test]
fn test_todo_in_heading() {
    let norg = r#"* Project Status (x) Completed
** Task List ( ) In Progress"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse TODO items in heading");
    assert!(result.contains("<h1>Project Status"));
    assert!(result.contains("Completed</h1>"));
    assert!(result.contains("<h2>Task List"));
    assert!(result.contains("In Progress</h2>"));
}

#[test]
fn test_todo_in_quote() {
    let norg = r#"> This quote contains a TODO:
> - (x) Review the documentation
> - ( ) Update the examples"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse TODO items in quote");
    assert!(result.contains("<blockquote>"));
    assert!(result.contains("This quote contains a TODO:"));
    assert!(result.contains("Review the documentation"));
    assert!(result.contains("Update the examples"));
    assert!(result.contains("</blockquote>"));
}

// Extended TODO Status Tests
#[test]
fn test_todo_partial_complete() {
    let norg = r#"- (/) Partially completed task"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse partial complete TODO");
    assert!(result.contains("<ul>"));
    assert!(result.contains("Partially completed task"));
    assert!(result.contains("</ul>"));
    // Should contain some form of partial completion indicator
    assert!(result.contains("partial") || result.contains("progress") || result.contains("/") || result.contains("◐"));
}

#[test]
fn test_todo_custom_status() {
    let norg = r#"- (>) Started task
- (<) Delegated task
- (o) Review needed
- (i) Information needed"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse custom status TODO items");
    assert!(result.contains("<ul>"));
    assert!(result.contains("Started task"));
    assert!(result.contains("Delegated task"));
    assert!(result.contains("Review needed"));
    assert!(result.contains("Information needed"));
    assert!(result.contains("</ul>"));
}

#[test]
fn test_todo_numeric_priorities() {
    let norg = r#"- (1) High priority task
- (2) Medium priority task
- (3) Low priority task"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse numeric priority TODO items");
    assert!(result.contains("<ul>"));
    assert!(result.contains("High priority task"));
    assert!(result.contains("Medium priority task"));
    assert!(result.contains("Low priority task"));
    assert!(result.contains("</ul>"));
}

#[test]
fn test_todo_letter_categories() {
    let norg = r#"- (A) Category A task
- (B) Category B task
- (C) Category C task"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse letter category TODO items");
    assert!(result.contains("<ul>"));
    assert!(result.contains("Category A task"));
    assert!(result.contains("Category B task"));
    assert!(result.contains("Category C task"));
    assert!(result.contains("</ul>"));
}

// TODO with Dates and Times Tests
#[test]
fn test_todo_with_due_date() {
    let norg = r#"- ( ) Task with due date <2023-12-31>
- (x) Completed task with due date <2023-11-15>"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse TODO items with due dates");
    assert!(result.contains("<ul>"));
    assert!(result.contains("Task with due date"));
    assert!(result.contains("Completed task with due date"));
    assert!(result.contains("</ul>"));
    // Should contain date information
    assert!(result.contains("2023-12-31") || result.contains("2023-11-15"));
}

#[test]
fn test_todo_with_start_date() {
    let norg = r#"- ( ) Task with start date >2023-12-01<
- (-) Pending task with start date >2023-11-01<"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse TODO items with start dates");
    assert!(result.contains("<ul>"));
    assert!(result.contains("Task with start date"));
    assert!(result.contains("Pending task with start date"));
    assert!(result.contains("</ul>"));
    // Should contain date information
    assert!(result.contains("2023-12-01") || result.contains("2023-11-01"));
}

#[test]
fn test_todo_with_timestamp() {
    let norg = r#"- (x) Completed task @2023-11-15 14:30
- ( ) Scheduled task @2023-12-01 09:00"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse TODO items with timestamps");
    assert!(result.contains("<ul>"));
    assert!(result.contains("Completed task"));
    assert!(result.contains("Scheduled task"));
    assert!(result.contains("</ul>"));
    // Should contain timestamp information
    assert!(result.contains("2023-11-15") || result.contains("2023-12-01"));
}

// TODO with Priority and Tags Tests
#[test]
fn test_todo_with_priority() {
    let norg = r#"- ( ) High priority task #priority:high
- (x) Medium priority task #priority:medium
- (-) Low priority task #priority:low"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse TODO items with priority");
    assert!(result.contains("<ul>"));
    assert!(result.contains("High priority task"));
    assert!(result.contains("Medium priority task"));
    assert!(result.contains("Low priority task"));
    assert!(result.contains("</ul>"));
    // Should contain priority information
    assert!(result.contains("priority") || result.contains("high") || result.contains("medium") || result.contains("low"));
}

#[test]
fn test_todo_with_tags() {
    let norg = r#"- ( ) Work task #work #project
- (x) Personal task #personal #health
- (!) Urgent task #urgent #important"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse TODO items with tags");
    assert!(result.contains("<ul>"));
    assert!(result.contains("Work task"));
    assert!(result.contains("Personal task"));
    assert!(result.contains("Urgent task"));
    assert!(result.contains("</ul>"));
    // Should contain tag information
    assert!(result.contains("work") || result.contains("project") || result.contains("personal") || result.contains("health"));
}

#[test]
fn test_todo_with_assignee() {
    let norg = r#"- ( ) Task assigned to @john
- (x) Task assigned to @jane
- (-) Task assigned to @team"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse TODO items with assignees");
    assert!(result.contains("<ul>"));
    assert!(result.contains("Task assigned to"));
    assert!(result.contains("</ul>"));
    // Should contain assignee information
    assert!(result.contains("john") || result.contains("jane") || result.contains("team"));
}

// Complex TODO Tests
#[test]
fn test_todo_complex_with_everything() {
    let norg = r#"- (x) Complete project documentation #work #docs @john <2023-11-30> @2023-11-15 10:00
  This task involves:
  - ( ) Write user guide
  - (x) Create API documentation
  - (-) Review technical specifications
  - (!) Fix urgent documentation bugs"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse complex TODO with everything");
    assert!(result.contains("<ul>"));
    assert!(result.contains("Complete project documentation"));
    assert!(result.contains("This task involves:"));
    assert!(result.contains("Write user guide"));
    assert!(result.contains("Create API documentation"));
    assert!(result.contains("Review technical specifications"));
    assert!(result.contains("Fix urgent documentation bugs"));
    assert!(result.contains("</ul>"));
}

#[test]
fn test_todo_project_structure() {
    let norg = r#"* Project: Website Redesign (/) 60% Complete

- (x) Planning Phase
  - (x) Research competitors
  - (x) Create wireframes
  - (x) Design mockups

- (/) Development Phase
  - (x) Set up development environment
  - ( ) Implement homepage
  - ( ) Implement about page
  - (-) Implement contact form

- ( ) Testing Phase
  - ( ) Unit testing
  - ( ) Integration testing
  - ( ) User acceptance testing

- ( ) Deployment Phase
  - ( ) Deploy to staging
  - ( ) Deploy to production
  - ( ) Monitor performance"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse project structure with TODOs");
    assert!(result.contains("<h1>Project: Website Redesign"));
    assert!(result.contains("60% Complete</h1>"));
    assert!(result.contains("Planning Phase"));
    assert!(result.contains("Research competitors"));
    assert!(result.contains("Development Phase"));
    assert!(result.contains("Testing Phase"));
    assert!(result.contains("Deployment Phase"));
}

// Error Cases and Edge Cases
#[test]
fn test_todo_invalid_status() {
    let norg = r#"- (z) Invalid status task
- (123) Numeric status task
- (abc) Multi-char status task"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse TODO items with invalid status");
    assert!(result.contains("<ul>"));
    assert!(result.contains("Invalid status task"));
    assert!(result.contains("Numeric status task"));
    assert!(result.contains("Multi-char status task"));
    assert!(result.contains("</ul>"));
}

#[test]
fn test_todo_malformed_missing_closing_paren() {
    let norg = r#"- (x Malformed task without closing paren
- ( ) Valid task after malformed"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse malformed TODO item");
    assert!(result.contains("<ul>"));
    assert!(result.contains("Malformed task without closing paren"));
    assert!(result.contains("Valid task after malformed"));
    assert!(result.contains("</ul>"));
}

#[test]
fn test_todo_malformed_missing_opening_paren() {
    let norg = r#"- x) Malformed task without opening paren
- ( ) Valid task after malformed"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse malformed TODO item");
    assert!(result.contains("<ul>"));
    assert!(result.contains("Malformed task without opening paren"));
    assert!(result.contains("Valid task after malformed"));
    assert!(result.contains("</ul>"));
}

#[test]
fn test_todo_empty_status() {
    let norg = r#"- () Empty status task
- ( ) Valid undone task"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse TODO item with empty status");
    assert!(result.contains("<ul>"));
    assert!(result.contains("Empty status task"));
    assert!(result.contains("Valid undone task"));
    assert!(result.contains("</ul>"));
}

#[test]
fn test_todo_whitespace_in_status() {
    let norg = r#"- (  ) Whitespace in status
- ( x ) Whitespace around status"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse TODO item with whitespace in status");
    assert!(result.contains("<ul>"));
    assert!(result.contains("Whitespace in status"));
    assert!(result.contains("Whitespace around status"));
    assert!(result.contains("</ul>"));
}

#[test]
fn test_todo_unicode_status() {
    let norg = r#"- (✓) Unicode check status
- (✗) Unicode cross status
- (★) Unicode star status"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse TODO item with unicode status");
    assert!(result.contains("<ul>"));
    assert!(result.contains("Unicode check status"));
    assert!(result.contains("Unicode cross status"));
    assert!(result.contains("Unicode star status"));
    assert!(result.contains("</ul>"));
}

#[test]
fn test_todo_with_special_characters() {
    let norg = r#"- ( ) Task with & < > " ' special characters
- (x) Completed task with special chars & symbols"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse TODO items with special characters");
    assert!(result.contains("<ul>"));
    assert!(result.contains("Task with &amp; &lt; &gt; &quot; &#x27; special characters"));
    assert!(result.contains("Completed task with special chars &amp; symbols"));
    assert!(result.contains("</ul>"));
}

#[test]
fn test_todo_with_line_breaks() {
    let norg = r#"- ( ) Task with
  line breaks in
  the content
- (x) Another task
  with breaks"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse TODO items with line breaks");
    assert!(result.contains("<ul>"));
    assert!(result.contains("Task with"));
    assert!(result.contains("line breaks in"));
    assert!(result.contains("the content"));
    assert!(result.contains("Another task"));
    assert!(result.contains("with breaks"));
    assert!(result.contains("</ul>"));
}