# Romodoro
This is Pomodoro timer web-app, similar to [Pomofocus](https://pomofocus.io) and others.
It is built in Rust and WebAssembly through the Yew framework.

## Features
- [X] Simple timer -- 5 seconds periods.
- [X] Simple Pomodoro counter -- 25 seconds work, then 5 seconds break.
- [X] The start button should also function as the stop button (that
pauses the timer).
- [X] [Pomofocus](https://pomofocus.io/) should be given proper credit.
- [X] The site should have a colored background.

## Later
- [ ] The site's background color should change depending on the kind of session (work or break), like in
[Pomofocus](https://pomofocus.io/).
  - In progress: It seems like [stylist](https://github.com/futursolo/stylist-rs/issues/87) is the best
  option to manipulate CSS dynamically, but [I can't seem to figure out how to do it yet](https://github.com/futursolo/stylist-rs/issues/87).
- [ ] After 4 work sessions, the break should be 15 seconds long.
- [ ] The user should be notified when a period has ended.
- [ ] The work and break periods should have configurable durations.
The defaults should be 25 minutes work, 5 minutes break.
- [ ] The work and break periods should be skippable.
- [ ] The work and break periods should be reset-able (one can start again from the
beginning of a period, or periods before that, in chronological order).
- [ ] The footer should be right-justified.
- [ ] The footer should only be visible upon hover (and still available using
accessibility tools)?
- [ ] The footer content should be in `index.html`, not in the code.

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
- The timer `div` color should be related to the background color, but so far I can
only control the color of the `div`. Should the Yew component instead contain
the whole page (not just the `div`)? Maybe that would force me to put all the content
in the code (like the footer text)?
- How would one add a settings panel akin to that of [Pomofocus](https://pomofocus.io/)? Is that even
possible with Yew (yet)?
