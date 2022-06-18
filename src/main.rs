use gloo_console::debug;
use gloo_timers::callback::{Interval, Timeout};
// use std::time;
// use std::prelude::Trait;
use time::{ext::NumericalDuration, Duration};
use yew::prelude::*;

/// Convert duration to milliseconds.
///
/// For use with `gloo` `Interval`s and `Timeout`s.
///
/// Note this is not appropriate for durations
/// with non-integer number of milliseconds.
fn to_millis(period: Duration) -> u32 {
    period.whole_milliseconds() as u32
}

/// Convert duration to string in format `min:sec`.
///
/// # Examples
///
/// ```
/// use time::ext::NumericalDuration;
///
/// assert_eq!("1000:00", to_string(1_000.minutes()))
/// assert_eq!("16:40", to_string(1_000.seconds()))
/// assert_eq!("00:00", to_string(0.seconds()))
/// ```
fn to_string(period: Duration) -> String {
    let minutes = period.whole_minutes();
    let seconds = period.whole_seconds() % 60;
    format!("{:02}:{:02}", minutes, seconds)
}

enum State {
    Idle,
    Work,
    Break,
}

struct PomoTimer {
    work_period: Duration,
    break_period: Duration,
    state: State,
    time_remaining: Duration,
    timer: Option<Timeout>,     // Tracks end of time remaining
    interval: Option<Interval>, // Makes seconds tick
}

impl PomoTimer {
    fn set_timer(&mut self, ctx: &Context<Self>, duration: Duration) {
        self.time_remaining = duration;
        self.timer = Some({
            let link = ctx.link().clone();
            // Stop the time out the moment *after* the last Tick,
            // so that the `time_remaining` can reach 0 seconds.
            // Otherwise we'd have to manually set `time_remaining`
            // to 0 when `Done` is sent.
            // TODO: Is there are way other than those two hacky ones?
            Timeout::new(to_millis(duration) + 1, move || {
                link.send_message(Msg::Done)
            })
        });
        self.interval = Some({
            let link = ctx.link().clone();
            Interval::new(to_millis(1.seconds()), move || link.send_message(Msg::Tick))
        });
    }
}

enum Msg {
    StartTimer,
    Tick,
    Done,
}

impl Component for PomoTimer {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            state: State::Idle,
            work_period: 5.seconds(),
            break_period: 3.seconds(),
            time_remaining: 0.seconds(),
            timer: None,
            interval: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::StartTimer => {
                let duration = match self.state {
                    State::Idle | State::Break => {
                        debug!("Start work period!");
                        self.state = State::Work;
                        self.work_period
                    }
                    State::Work => {
                        debug!("Start break period!");
                        self.state = State::Break;
                        self.break_period
                    }
                };
                self.set_timer(ctx, duration);
                true
            }
            Msg::Tick => {
                debug!("Tick!");
                self.time_remaining -= 1.seconds();
                true
            }
            Msg::Done => {
                debug!("Done!");
                self.interval = None;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <button onclick={ ctx.link().callback(|_| Msg::StartTimer) }>
                    { "Start timer" }
                </button>
                <p>{ to_string(self.time_remaining) }</p>
            </div>
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {}

    fn destroy(&mut self, _ctx: &Context<Self>) {}
}

fn main() {
    yew::Renderer::<PomoTimer>::new().render();
}
