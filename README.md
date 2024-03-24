[Programming Mentoring challenge](https://discord.com/channels/1130043756477960256/1130173129684160634/1218925373518254080)

# Description
  
Good news, Banca Tommaso PFM is expanding internationally! As a result, you have
some [Internationalization (i18n) and localization (l10n)](https://en.wikipedia.org/wiki/Internationalization_and_localization)
tasks to do. One of them is adding the ability to display dates in different formats.
We all know that ~~dd-mm-yyyy~~ YYYY-MM-DD is obviously the
[superior date format](https://dev-to-uploads.s3.amazonaws.com/i/6gnzyh285b31v6n202fg.png)
and we don't need others, but unfortunately not [all countries](https://en.wikipedia.org/wiki/List_of_date_formats_by_country)
have come to this conclusion yet. 
"Easy task" you think and proceed to add  [MomentJS](https://momentjs.com/)
to the project and open the PR.
A few hours later you find this comment on the PR:
 
> Nice job! Everything looks great but I have some concerns about the
introduction of moment in the project.
The only feature we are using is the date formatting, but this package is adding
~300Kb to the project. Also the authors are kind of
[deprecating](https://momentjs.com/docs/#/-project-status/) it. Are there other
alternatives out there? Can we implement the format function ourself?

You then decide to try to implement the function yourself.

# Tasks

Create a function that given a Date and a string format, returns a string
representation of the date, with the specified format.

A valid format is a string composed by 3 parts joined by a separator.

e.g. `mm/dd/yyyy`. In this example `/` is the separator and the 3 parts are
`mm`, `dd` and `yyyy`.

List of valid separators:

- `/` Oblique stroke (slash)
- `.` Full stop, dot or point (period)
- `-` Hyphen (dash)
- ` ` Space

List of valid parts:

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

Example: 

 `format(new Date(), 'dd mmmm yyyy') // 17 Mar 2024`
