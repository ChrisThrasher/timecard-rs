# timecard-rs

[![CI](https://github.com/ChrisThrasher/timecard-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/ChrisThrasher/timecard-rs/actions/workflows/ci.yml)

Calculates how much time was spent on various activities throughout the day.

Reimplementation of https://github.com/ChrisThrasher/timecard.

# Usage

```
$ timecard-rs --help
Time formatting can follow one of two patterns depending on the time it
represents. 8:00 a.m. can be formatted as "8:00am" or "8am". 12:30 p.m. is
formatted only as "12:30pm". "now" is interpreted as the current time.

Any activities named "-" will be ignored. This activity's durations are
reported as "off time" should they exist.

Usage: timecard-rs <args>...

Arguments:
  <args>...  Alternating times and activities

Options:
  -h, --help     Print help
  -V, --version  Print version
```

Starting with the time the first activity started, list all times that activities changed along with the activities that occurred between those times. If you spent 8:00 a.m. to 10:00 a.m. gardening then 10:00 a.m. to 11:00 a.m. reading, the command would look like this:

```
$ timecard-rs 8am gardening 10am reading 11am
reading     1.0 hours
gardening   2.0 hours

Total: 3.0 hours
```

If multiple chunks of time were spent on one activity, then include the additional chunks using the same label. Expanding on our previous example, lets add two more chunks for eating lunch and reading some more.

```
$ timecard-rs 8am gardening 10am reading 11am lunch 12:30pm reading 2pm
lunch       1.5 hours
reading     2.5 hours
gardening   2.0 hours

Total: 6.0 hours
```

Because the label `reading` appeared twice, its two durations were accumulated. This will be done for any labels which appear more than once.

To ignore certain periods of time so that they're not reported, name them `-`. This will exclude them from the printed totals.

```
$ timecard-rs 8am gardening 10am reading 11am lunch 12:30pm reading 2pm - 9pm reading 10pm
reading     3.5 hours
gardening   2.0 hours
lunch       1.5 hours

Total: 7.0 hours (7.0 hours off)
```

As a convenience "now" is interpreted as the current time. If you read until the time at which you ran the program (10:00 pm), you could simply do this:

```
$ timecard-rs 8am gardening 10am reading 11am lunch 12:30pm reading 2pm - 9pm reading now
gardening   2.0 hours
lunch       1.5 hours
reading     3.5 hours

Total: 7.0 hours (7.0 hours off)
```
