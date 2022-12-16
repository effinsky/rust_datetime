use chrono::prelude::*;

#[derive(Debug)]
pub struct DateTimeRange {
  pub start: String,
  pub end: String,
}

pub struct DateTimeTup(DateTime<FixedOffset>, DateTime<FixedOffset>);

impl DateTimeRange {
  pub fn parse(self) -> DateTimeTup {
    let start = DateTime::parse_from_rfc3339(&self.start)
      .expect("expected a parsable timestamp");
    let end = DateTime::parse_from_rfc3339(&self.end)
      .expect("expected a parsable timestamp");

    DateTimeTup(start, end)
  }
}

struct PrepOpts {
  should_reset_time: bool,
}

fn main() {
  let prom = DateTimeRange {
    start: String::from("2022-12-02T00:00:00Z"),
    end: String::from("2022-12-02T00:00:00Z"),
  };

  let prep_result = prep_promise_fields(
    prom,
    PrepOpts {
      should_reset_time: true,
    },
  );

  println!("preparation result: {:?}", prep_result);
}

fn prep_promise_fields(
  prom: DateTimeRange,
  prep_opts: PrepOpts,
) -> DateTimeRange {
  let DateTimeTup(start, end) = prom.parse();

  if prep_opts.should_reset_time {
    reset_time(&start);
    reset_time(&end);
  }

  let start_utc = Utc
    .with_ymd_and_hms(
      start.year(),
      start.month(),
      start.day(),
      start.hour(),
      start.minute(),
      start.second(),
    )
    .unwrap();
  let end_utc = Utc
    .with_ymd_and_hms(
      end.year(),
      end.month(),
      end.day(),
      end.hour(),
      end.minute(),
      end.second(),
    )
    .unwrap();

  DateTimeRange {
    start: start_utc.to_string(),
    end: if start_utc == end_utc {
      String::from("")
    } else {
      end_utc.to_string()
    },
  }
}

fn reset_time(t: &DateTime<FixedOffset>) -> DateTime<FixedOffset> {
  t.with_year(t.year())
    .unwrap()
    .with_month(t.month())
    .unwrap()
    .with_day(t.day())
    .unwrap()
    .with_hour(0)
    .unwrap()
    .with_minute(0)
    .unwrap()
    .with_second(0)
    .unwrap()
    .with_timezone(&t.timezone())
}
