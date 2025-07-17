use norgmill::renderer::parse_and_render_norg;

// Basic Timestamp Tests
#[test]
fn test_timestamp_basic_date() {
    let norg = r#"Meeting scheduled for @2023-12-15."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse basic timestamp");
    assert!(result.contains("<p>Meeting scheduled for"));
    assert!(result.contains("2023-12-15"));
    assert!(result.contains("</p>"));
    // Should contain time element or timestamp indicator
    assert!(result.contains("<time") || result.contains("datetime") || result.contains("timestamp"));
}

#[test]
fn test_timestamp_with_time() {
    let norg = r#"Appointment at @2023-12-15 14:30."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse timestamp with time");
    assert!(result.contains("<p>Appointment at"));
    assert!(result.contains("2023-12-15"));
    assert!(result.contains("14:30"));
    assert!(result.contains("</p>"));
    assert!(result.contains("<time") || result.contains("datetime") || result.contains("timestamp"));
}

#[test]
fn test_timestamp_with_seconds() {
    let norg = r#"Event starts at @2023-12-15 14:30:45."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse timestamp with seconds");
    assert!(result.contains("<p>Event starts at"));
    assert!(result.contains("2023-12-15"));
    assert!(result.contains("14:30:45"));
    assert!(result.contains("</p>"));
    assert!(result.contains("<time") || result.contains("datetime") || result.contains("timestamp"));
}

#[test]
fn test_timestamp_iso_format() {
    let norg = r#"ISO timestamp: @2023-12-15T14:30:45Z."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse ISO timestamp");
    assert!(result.contains("<p>ISO timestamp:"));
    assert!(result.contains("2023-12-15T14:30:45Z"));
    assert!(result.contains("</p>"));
    assert!(result.contains("<time") || result.contains("datetime") || result.contains("timestamp"));
}

#[test]
fn test_timestamp_with_timezone() {
    let norg = r#"Conference call @2023-12-15 14:30 +0200."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse timestamp with timezone");
    assert!(result.contains("<p>Conference call"));
    assert!(result.contains("2023-12-15"));
    assert!(result.contains("14:30"));
    assert!(result.contains("+0200"));
    assert!(result.contains("</p>"));
    assert!(result.contains("<time") || result.contains("datetime") || result.contains("timestamp"));
}

#[test]
fn test_timestamp_utc_timezone() {
    let norg = r#"UTC meeting @2023-12-15 14:30 UTC."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse UTC timestamp");
    assert!(result.contains("<p>UTC meeting"));
    assert!(result.contains("2023-12-15"));
    assert!(result.contains("14:30"));
    assert!(result.contains("UTC"));
    assert!(result.contains("</p>"));
    assert!(result.contains("<time") || result.contains("datetime") || result.contains("timestamp"));
}

// Different Date Formats Tests
#[test]
fn test_timestamp_american_format() {
    let norg = r#"American date format @12/15/2023."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse American date format");
    assert!(result.contains("<p>American date format"));
    assert!(result.contains("12/15/2023"));
    assert!(result.contains("</p>"));
    assert!(result.contains("<time") || result.contains("datetime") || result.contains("timestamp"));
}

#[test]
fn test_timestamp_european_format() {
    let norg = r#"European date format @15.12.2023."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse European date format");
    assert!(result.contains("<p>European date format"));
    assert!(result.contains("15.12.2023"));
    assert!(result.contains("</p>"));
    assert!(result.contains("<time") || result.contains("datetime") || result.contains("timestamp"));
}

#[test]
fn test_timestamp_short_year() {
    let norg = r#"Short year @23-12-15."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse short year timestamp");
    assert!(result.contains("<p>Short year"));
    assert!(result.contains("23-12-15"));
    assert!(result.contains("</p>"));
    assert!(result.contains("<time") || result.contains("datetime") || result.contains("timestamp"));
}

#[test]
fn test_timestamp_month_names() {
    let norg = r#"Month names @December 15, 2023."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse month names timestamp");
    assert!(result.contains("<p>Month names"));
    assert!(result.contains("December 15, 2023"));
    assert!(result.contains("</p>"));
    assert!(result.contains("<time") || result.contains("datetime") || result.contains("timestamp"));
}

#[test]
fn test_timestamp_short_month_names() {
    let norg = r#"Short month names @Dec 15, 2023."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse short month names timestamp");
    assert!(result.contains("<p>Short month names"));
    assert!(result.contains("Dec 15, 2023"));
    assert!(result.contains("</p>"));
    assert!(result.contains("<time") || result.contains("datetime") || result.contains("timestamp"));
}

// Relative Time Tests
#[test]
fn test_timestamp_relative_today() {
    let norg = r#"Meeting @today at 15:00."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse relative today timestamp");
    assert!(result.contains("<p>Meeting"));
    assert!(result.contains("today"));
    assert!(result.contains("15:00"));
    assert!(result.contains("</p>"));
    assert!(result.contains("<time") || result.contains("datetime") || result.contains("timestamp"));
}

#[test]
fn test_timestamp_relative_tomorrow() {
    let norg = r#"Deadline @tomorrow."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse relative tomorrow timestamp");
    assert!(result.contains("<p>Deadline"));
    assert!(result.contains("tomorrow"));
    assert!(result.contains("</p>"));
    assert!(result.contains("<time") || result.contains("datetime") || result.contains("timestamp"));
}

#[test]
fn test_timestamp_relative_yesterday() {
    let norg = r#"Completed @yesterday."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse relative yesterday timestamp");
    assert!(result.contains("<p>Completed"));
    assert!(result.contains("yesterday"));
    assert!(result.contains("</p>"));
    assert!(result.contains("<time") || result.contains("datetime") || result.contains("timestamp"));
}

#[test]
fn test_timestamp_relative_now() {
    let norg = r#"Updated @now."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse relative now timestamp");
    assert!(result.contains("<p>Updated"));
    assert!(result.contains("now"));
    assert!(result.contains("</p>"));
    assert!(result.contains("<time") || result.contains("datetime") || result.contains("timestamp"));
}

#[test]
fn test_timestamp_relative_days() {
    let norg = r#"Due @+3 days from now."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse relative days timestamp");
    assert!(result.contains("<p>Due"));
    assert!(result.contains("+3 days from now"));
    assert!(result.contains("</p>"));
    assert!(result.contains("<time") || result.contains("datetime") || result.contains("timestamp"));
}

#[test]
fn test_timestamp_relative_weeks() {
    let norg = r#"Meeting @+2 weeks."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse relative weeks timestamp");
    assert!(result.contains("<p>Meeting"));
    assert!(result.contains("+2 weeks"));
    assert!(result.contains("</p>"));
    assert!(result.contains("<time") || result.contains("datetime") || result.contains("timestamp"));
}

#[test]
fn test_timestamp_relative_months() {
    let norg = r#"Review @+6 months."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse relative months timestamp");
    assert!(result.contains("<p>Review"));
    assert!(result.contains("+6 months"));
    assert!(result.contains("</p>"));
    assert!(result.contains("<time") || result.contains("datetime") || result.contains("timestamp"));
}

// Time-only Tests
#[test]
fn test_timestamp_time_only_24h() {
    let norg = r#"Meeting @14:30."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse time-only 24h timestamp");
    assert!(result.contains("<p>Meeting"));
    assert!(result.contains("14:30"));
    assert!(result.contains("</p>"));
    assert!(result.contains("<time") || result.contains("datetime") || result.contains("timestamp"));
}

#[test]
fn test_timestamp_time_only_12h() {
    let norg = r#"Meeting @2:30 PM."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse time-only 12h timestamp");
    assert!(result.contains("<p>Meeting"));
    assert!(result.contains("2:30 PM"));
    assert!(result.contains("</p>"));
    assert!(result.contains("<time") || result.contains("datetime") || result.contains("timestamp"));
}

#[test]
fn test_timestamp_time_only_am() {
    let norg = r#"Breakfast @9:00 AM."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse time-only AM timestamp");
    assert!(result.contains("<p>Breakfast"));
    assert!(result.contains("9:00 AM"));
    assert!(result.contains("</p>"));
    assert!(result.contains("<time") || result.contains("datetime") || result.contains("timestamp"));
}

#[test]
fn test_timestamp_time_with_seconds_12h() {
    let norg = r#"Precise time @2:30:45 PM."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse precise 12h timestamp");
    assert!(result.contains("<p>Precise time"));
    assert!(result.contains("2:30:45 PM"));
    assert!(result.contains("</p>"));
    assert!(result.contains("<time") || result.contains("datetime") || result.contains("timestamp"));
}

// Multiple Timestamps Tests
#[test]
fn test_multiple_timestamps_same_paragraph() {
    let norg = r#"Meeting from @2023-12-15 14:00 to @2023-12-15 15:30."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse multiple timestamps");
    assert!(result.contains("<p>Meeting from"));
    assert!(result.contains("2023-12-15 14:00"));
    assert!(result.contains("to"));
    assert!(result.contains("2023-12-15 15:30"));
    assert!(result.contains("</p>"));
    // Should have two time elements
    let time_count = result.matches("<time").count();
    assert!(time_count >= 2 || result.matches("datetime").count() >= 2);
}

#[test]
fn test_multiple_timestamps_different_formats() {
    let norg = r#"Event starts @2023-12-15 and ends @December 20, 2023."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse multiple timestamp formats");
    assert!(result.contains("<p>Event starts"));
    assert!(result.contains("2023-12-15"));
    assert!(result.contains("and ends"));
    assert!(result.contains("December 20, 2023"));
    assert!(result.contains("</p>"));
}

#[test]
fn test_timestamps_in_list() {
    let norg = r#"- Meeting @2023-12-15 14:00
- Lunch @2023-12-15 12:30
- Presentation @2023-12-15 16:00"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse timestamps in list");
    assert!(result.contains("<ul>"));
    assert!(result.contains("<li>Meeting"));
    assert!(result.contains("2023-12-15 14:00"));
    assert!(result.contains("<li>Lunch"));
    assert!(result.contains("2023-12-15 12:30"));
    assert!(result.contains("<li>Presentation"));
    assert!(result.contains("2023-12-15 16:00"));
    assert!(result.contains("</ul>"));
}

#[test]
fn test_timestamps_in_heading() {
    let norg = r#"* Meeting Agenda @2023-12-15
** Morning Session @9:00 AM
** Afternoon Session @2:00 PM"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse timestamps in heading");
    assert!(result.contains("<h1>Meeting Agenda"));
    assert!(result.contains("2023-12-15"));
    assert!(result.contains("</h1>"));
    assert!(result.contains("<h2>Morning Session"));
    assert!(result.contains("9:00 AM"));
    assert!(result.contains("</h2>"));
    assert!(result.contains("<h2>Afternoon Session"));
    assert!(result.contains("2:00 PM"));
    assert!(result.contains("</h2>"));
}

#[test]
fn test_timestamps_in_quote() {
    let norg = r#"> The meeting is scheduled for @2023-12-15 14:00.
> Please confirm your attendance by @2023-12-14."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse timestamps in quote");
    assert!(result.contains("<blockquote>"));
    assert!(result.contains("The meeting is scheduled for"));
    assert!(result.contains("2023-12-15 14:00"));
    assert!(result.contains("Please confirm your attendance by"));
    assert!(result.contains("2023-12-14"));
    assert!(result.contains("</blockquote>"));
}

// Timestamp with Context Tests
#[test]
fn test_timestamp_with_duration() {
    let norg = r#"Meeting @2023-12-15 14:00 (2 hours)."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse timestamp with duration");
    assert!(result.contains("<p>Meeting"));
    assert!(result.contains("2023-12-15 14:00"));
    assert!(result.contains("(2 hours)"));
    assert!(result.contains("</p>"));
    assert!(result.contains("<time") || result.contains("datetime") || result.contains("timestamp"));
}

#[test]
fn test_timestamp_with_location() {
    let norg = r#"Conference @2023-12-15 09:00 at Conference Room A."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse timestamp with location");
    assert!(result.contains("<p>Conference"));
    assert!(result.contains("2023-12-15 09:00"));
    assert!(result.contains("at Conference Room A"));
    assert!(result.contains("</p>"));
    assert!(result.contains("<time") || result.contains("datetime") || result.contains("timestamp"));
}

#[test]
fn test_timestamp_with_description() {
    let norg = r#"@2023-12-15 14:00: Important team meeting."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse timestamp with description");
    assert!(result.contains("<p>"));
    assert!(result.contains("2023-12-15 14:00"));
    assert!(result.contains(": Important team meeting"));
    assert!(result.contains("</p>"));
    assert!(result.contains("<time") || result.contains("datetime") || result.contains("timestamp"));
}

#[test]
fn test_timestamp_recurring_pattern() {
    let norg = r#"Weekly meeting @every Monday 14:00."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse recurring timestamp");
    assert!(result.contains("<p>Weekly meeting"));
    assert!(result.contains("every Monday 14:00"));
    assert!(result.contains("</p>"));
    assert!(result.contains("<time") || result.contains("datetime") || result.contains("timestamp"));
}

// Timestamp in Different Contexts Tests
#[test]
fn test_timestamp_in_definition() {
    let norg = r#"$ Meeting
A scheduled event @2023-12-15 14:00 with the team."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse timestamp in definition");
    assert!(result.contains("<dl>"));
    assert!(result.contains("<dt>Meeting</dt>"));
    assert!(result.contains("<dd>A scheduled event"));
    assert!(result.contains("2023-12-15 14:00"));
    assert!(result.contains("with the team.</dd>"));
    assert!(result.contains("</dl>"));
}

#[test]
fn test_timestamp_in_footnote() {
    let norg = r#"^ Meeting Note
The meeting was scheduled for @2023-12-15 14:00."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse timestamp in footnote");
    assert!(result.contains("Meeting Note"));
    assert!(result.contains("The meeting was scheduled for"));
    assert!(result.contains("2023-12-15 14:00"));
}

#[test]
fn test_timestamp_in_table_cell() {
    let norg = r#": A1 : Meeting @2023-12-15 14:00
: B1 : Lunch @2023-12-15 12:30"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse timestamp in table cell");
    assert!(result.contains("<table>"));
    assert!(result.contains("<td>Meeting"));
    assert!(result.contains("2023-12-15 14:00"));
    assert!(result.contains("</td>"));
    assert!(result.contains("<td>Lunch"));
    assert!(result.contains("2023-12-15 12:30"));
    assert!(result.contains("</td>"));
    assert!(result.contains("</table>"));
}

#[test]
fn test_timestamp_with_todo() {
    let norg = r#"- ( ) Complete project @2023-12-15
- (x) Review code @2023-12-10"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse timestamp with TODO");
    assert!(result.contains("<ul>"));
    assert!(result.contains("Complete project"));
    assert!(result.contains("2023-12-15"));
    assert!(result.contains("Review code"));
    assert!(result.contains("2023-12-10"));
    assert!(result.contains("</ul>"));
}

// Date Range Tests
#[test]
fn test_timestamp_date_range() {
    let norg = r#"Conference @2023-12-15 to @2023-12-17."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse date range timestamp");
    assert!(result.contains("<p>Conference"));
    assert!(result.contains("2023-12-15"));
    assert!(result.contains("to"));
    assert!(result.contains("2023-12-17"));
    assert!(result.contains("</p>"));
}

#[test]
fn test_timestamp_time_range() {
    let norg = r#"Meeting @14:00 - @16:00."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse time range timestamp");
    assert!(result.contains("<p>Meeting"));
    assert!(result.contains("14:00"));
    assert!(result.contains("-"));
    assert!(result.contains("16:00"));
    assert!(result.contains("</p>"));
}

#[test]
fn test_timestamp_datetime_range() {
    let norg = r#"Workshop @2023-12-15 09:00 - @2023-12-15 17:00."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse datetime range timestamp");
    assert!(result.contains("<p>Workshop"));
    assert!(result.contains("2023-12-15 09:00"));
    assert!(result.contains("-"));
    assert!(result.contains("2023-12-15 17:00"));
    assert!(result.contains("</p>"));
}

// Special Date Values Tests
#[test]
fn test_timestamp_special_dates() {
    let norg = r#"Christmas @2023-12-25
New Year @2024-01-01
Valentine's Day @2024-02-14"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse special date timestamps");
    assert!(result.contains("Christmas"));
    assert!(result.contains("2023-12-25"));
    assert!(result.contains("New Year"));
    assert!(result.contains("2024-01-01"));
    assert!(result.contains("Valentine's Day"));
    assert!(result.contains("2024-02-14"));
}

#[test]
fn test_timestamp_weekdays() {
    let norg = r#"Meeting @Monday 14:00
Lunch @Friday 12:30"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse weekday timestamps");
    assert!(result.contains("Meeting"));
    assert!(result.contains("Monday 14:00"));
    assert!(result.contains("Lunch"));
    assert!(result.contains("Friday 12:30"));
}

#[test]
fn test_timestamp_with_markup() {
    let norg = r#"*Important meeting* @2023-12-15 14:00.
/Casual lunch/ @2023-12-16 12:30."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse timestamp with markup");
    assert!(result.contains("<strong>Important meeting</strong>"));
    assert!(result.contains("2023-12-15 14:00"));
    assert!(result.contains("<em>Casual lunch</em>"));
    assert!(result.contains("2023-12-16 12:30"));
}

// Error Cases and Edge Cases
#[test]
fn test_timestamp_invalid_date() {
    let norg = r#"Invalid date @2023-13-32."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse invalid date timestamp");
    assert!(result.contains("<p>Invalid date"));
    assert!(result.contains("2023-13-32"));
    assert!(result.contains("</p>"));
    // Should handle gracefully, either as timestamp or as text
}

#[test]
fn test_timestamp_malformed_format() {
    let norg = r#"Malformed @2023/12/15/14:30."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse malformed timestamp");
    assert!(result.contains("<p>Malformed"));
    assert!(result.contains("2023/12/15/14:30"));
    assert!(result.contains("</p>"));
}

#[test]
fn test_timestamp_empty() {
    let norg = r#"Empty timestamp @."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse empty timestamp");
    assert!(result.contains("<p>Empty timestamp"));
    assert!(result.contains("@"));
    assert!(result.contains("</p>"));
}

#[test]
fn test_timestamp_whitespace() {
    let norg = r#"Whitespace @  2023-12-15  14:30  ."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse timestamp with whitespace");
    assert!(result.contains("<p>Whitespace"));
    assert!(result.contains("2023-12-15"));
    assert!(result.contains("14:30"));
    assert!(result.contains("</p>"));
}

#[test]
fn test_timestamp_special_characters() {
    let norg = r#"Special chars @2023-12-15 & meeting."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse timestamp with special characters");
    assert!(result.contains("<p>Special chars"));
    assert!(result.contains("2023-12-15"));
    assert!(result.contains("&amp; meeting"));
    assert!(result.contains("</p>"));
}

#[test]
fn test_timestamp_unicode() {
    let norg = r#"Unicode meeting @2023-12-15 café."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse timestamp with unicode");
    assert!(result.contains("<p>Unicode meeting"));
    assert!(result.contains("2023-12-15"));
    assert!(result.contains("café"));
    assert!(result.contains("</p>"));
}

#[test]
fn test_timestamp_line_breaks() {
    let norg = r#"Meeting scheduled for
@2023-12-15 14:00
with the team."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse timestamp with line breaks");
    assert!(result.contains("Meeting scheduled for"));
    assert!(result.contains("2023-12-15 14:00"));
    assert!(result.contains("with the team"));
}

#[test]
fn test_timestamp_at_line_start() {
    let norg = r#"@2023-12-15 Meeting starts at this time."#;
    let result = parse_and_render_norg(norg).expect("Failed to parse timestamp at line start");
    assert!(result.contains("<p>"));
    assert!(result.contains("2023-12-15"));
    assert!(result.contains("Meeting starts at this time"));
    assert!(result.contains("</p>"));
}

#[test]
fn test_timestamp_at_line_end() {
    let norg = r#"Meeting is scheduled for @2023-12-15"#;
    let result = parse_and_render_norg(norg).expect("Failed to parse timestamp at line end");
    assert!(result.contains("<p>Meeting is scheduled for"));
    assert!(result.contains("2023-12-15"));
    assert!(result.contains("</p>"));
}