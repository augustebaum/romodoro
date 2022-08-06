# Romodoro
This is Pomodoro timer web-app, similar to [Pomofocus](pomofocus.io) and others.
It is built in Rust and WebAssembly through the Yew framework.

## Features
- [X] Simple timer -- 5 seconds periods.
- [X] Simple Pomodoro counter -- 25 seconds work, then 5 seconds break.
- [X] The start button should also function as the stop button (that
pauses the timer).

## Later
- [ ] [pomofocus.io](https://pomofocus.io/) should be given proper credit.
- [ ] The site should have a colored background, with the color depending on the kind of session (like in
[pomofocus.io](https://pomofocus.io/)).
- [ ] After 4 work sessions, the break should be 15 seconds long.
- [ ] The user should be notified when a period has ended.
- [ ] The work and break periods should have configurable durations.
The defaults should be 25 minutes work, 5 minutes break.
- [ ] The work and break periods should be skippable.
- [ ] The work and break periods should be reset-able (one can start again from the
beginning of a period, or periods before that, in chronological order).

## Developing
Follow the [Yew setup process](https://yew.rs/docs/getting-started/introduction)
and run the app locally with
```sh
trunk serve
```

## Implementation notes
### Overview
- The timer is modelled by a Yew component named `PomoTimer`.
- The timer is controlled via `Msg`s, so that each kind of `Msg`s elicits different behaviour from `PomoTimer`.
- `PomoTimer` is in one of the states given in the `State` enum.
A certain `Msg` can elicit different behaviour depending on the `State` `PomoTimer` is in.

### Properties (could be automatically checked or proved)
- `PomoTimer` state can never go from `Idle` to `Paused`.
- A `Tick` or `Done` can never occur when `PomoTimer` state is `Idle` or `Paused`.

### Questions
- Is the `Tick` message necessary, or could we just have the
`interval` decrement `time_remaining` directly?
