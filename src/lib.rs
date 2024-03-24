use std::error::Error;

use chrono::{Datelike, NaiveDate};
use enum_iterator::{all, Sequence};
use phf::phf_ordered_map;

/**
Returns a string representing the `date` in the specified `format`.
A valid format consists of any combination of sections separated by separators.

## List of valid separators

- `/` Oblique stroke (slash)
- `.` Full stop, dot or point (period)
- `-` Hyphen (dash)
- ` ` Space

## List of valid sections

- `yy` Two-digit year, e.g. `24`
- `yyyy` Four-digit year, e.g. `2024`
- `m` One-digit month for months below 10, e.g. `3`
- `mm` Two-digit month, e.g. `03`
- `mmm` Three-letter abbreviation for month, e.g. `Mar`
- `mmmm` Month spelled out in full, e.g. `March`
- `d` One-digit day of the month for days below 10, e.g. `2`
- `dd` Two-digit day of the month, e.g. `02`
- `ddd` Three-letter abbreviation for day of the week, e.g. `Fri`
- `dddd` Day of the week spelled out in full, e.g. `Friday`

There is no locale that is specifically followed, but the names for the days and
months are taken from
[The Unicode Common Locale Data Repository](https://github.com/unicode-org/cldr-json/blob/main/cldr-json/cldr-dates-modern/main/en/ca-gregorian.json)

 */
pub fn format(date: &NaiveDate, format: &str) -> Result<String, Box<dyn Error>> {
    let parts: Vec<FormatPart> = split_format(format)
        .iter()
        .map(|&v| FormatPart::try_from(v))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(parts
        .into_iter()
        .map(|v| match v {
            FormatPart::Section(section) => section.format(date),
            FormatPart::Separator(separator) => separator.value().to_string(),
        })
        .reduce(|acc, v| format!("{}{}", acc, v))
        .ok_or("No part found")?)
}

fn split_format(format: &str) -> Vec<&str> {
    let mut raw_sections = Vec::new();
    let mut last = 0;
    for (index, separator) in format.match_indices(|c: char| {
        all::<Separator>()
            .map(|v| v.value().to_string())
            .any(|v| v == c.to_string())
    }) {
        if last != index {
            raw_sections.push(&format[last..index]);
        }
        raw_sections.push(separator);
        last = index + separator.len();
    }
    if last < format.len() {
        raw_sections.push(&format[last..]);
    }

    raw_sections
}

#[derive(Debug, Sequence)]
enum FormatPart {
    Section(Section),
    Separator(Separator),
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Sequence)]
enum Section {
    YY,
    YYYY,
    M,
    MM,
    MMM,
    D,
    DD,
}

#[derive(Debug, Sequence)]
enum Separator {
    Slash,
    Period,
    Hyphen,
    Space,
}

impl TryFrom<&str> for FormatPart {
    type Error = Box<dyn Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        all::<FormatPart>()
            .find(|v| v.value() == value)
            .ok_or(format!("Part not supported: {}", value).into())
    }
}

impl FormatPart {
    fn value(&self) -> &str {
        match self {
            FormatPart::Section(section) => section.value(),
            FormatPart::Separator(separator) => separator.value(),
        }
    }
}

impl Section {
    fn value(&self) -> &str {
        match *self {
            Self::YY => "yy",
            Self::YYYY => "yyyy",
            Self::M => "m",
            Self::MM => "mm",
            Self::MMM => "mmm",
            Self::D => "d",
            Self::DD => "dd",
        }
    }

    fn format(&self, date: &NaiveDate) -> String {
        match self {
            Section::YY => format!("{:0>2}", date.year() % 100),
            Section::YYYY => format!("{}", date.year()),
            Section::M => format!("{}", date.month()),
            Section::MM => format!("{:0>2}", date.month()),
            Section::MMM => MONTHS_ABBREVIATED
                .get(&(date.month() as u8))
                .expect("month value found")
                .to_string(),
            Section::D => format!("{}", date.day()),
            Section::DD => format!("{:0>2}", date.day()),
        }
    }
}

impl Separator {
    fn value(&self) -> &str {
        match *self {
            Self::Slash => "/",
            Self::Period => ".",
            Self::Hyphen => "-",
            Self::Space => " ",
        }
    }
}

static MONTHS_ABBREVIATED: phf::OrderedMap<u8, &str> = phf_ordered_map! {
    1u8 => "Jan",
    2u8 => "Feb",
    3u8 => "Mar",
    4u8 => "Apr",
    5u8 => "May",
    6u8 => "Jun",
    7u8 => "Jul",
    8u8 => "Aug",
    9u8 => "Sep",
    10u8 => "Oct",
    11u8 => "Nov",
    12u8 => "Dec",
};

static MONTHS_WIDE: phf::OrderedMap<u8, &str> = phf_ordered_map! {
    1u8 => "January",
    2u8 => "February",
    3u8 => "March",
    4u8 => "April",
    5u8 => "May",
    6u8 => "June",
    7u8 => "July",
    8u8 => "August",
    9u8 => "September",
    10u8 => "October",
    11u8 => "November",
    12u8 => "December",
};

static DAYS_ABBREVIATED: phf::OrderedMap<u8, &str> = phf_ordered_map! {
    1u8 => "Mon",
    2u8 => "Tue",
    3u8 => "Wed",
    4u8 => "Thu",
    5u8 => "Fri",
    6u8 => "Sat",
    7u8 => "Sun",
};

static DAYS_WIDE: phf::OrderedMap<u8, &str> = phf_ordered_map! {
    1u8 => "Monday",
    2u8 => "Tuesday",
    3u8 => "Wednesday",
    4u8 => "Thursday",
    5u8 => "Friday",
    6u8 => "Saturday",
    7u8 => "Sunday",
};
