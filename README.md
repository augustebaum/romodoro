# Romodoro -- Pomodoro in Rust
This is a webapp for a Pomodoro timer, similar to [Pomofocus](pomofocus.io) and others.
It is built in Rust and WebAssembly through the Yew framework.

## Features
- [X] Simple timer -- 5 seconds periods.
- [X] Simple Pomodoro counter -- 25 seconds work, then 5 seconds break.

## Later
- [ ] The start button should also function as the stop button (that
pauses the timer).
- [ ] After 4 work sessions, the break should be 15 seconds long.
- [ ] The user should be notified when a period has ended.
- [ ] The work and break periods should have configurable durations.
The defaults should be 25 minutes work, 5 minutes break.
- [ ] The work and break periods should be skippable.
- [ ] The work and break periods should be reset-able (one can start again from the
beginning of a period, or periods before that, in chronological order).
