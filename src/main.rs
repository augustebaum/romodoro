use gloo_console::debug;
use gloo_timers::callback::{Interval, Timeout};
use time::{ext::NumericalDuration, Duration};
use yew::prelude::*;
mod time_utils;
use time_utils::{to_millis, to_string};

#[derive(Debug, Clone)]
enum State {
    Idle,
    Work,
    Break,
    Paused,
}

struct PomoTimer {
    work_period: Duration,
    break_period: Duration,
    state: State,
    previous_state: State,
    time_remaining: Duration,
    timer: Option<Timeout>,     // Tracks time remaining
    interval: Option<Interval>, // Makes seconds tick
}

impl PomoTimer {
    fn set_state(&mut self, state: State) {
        self.previous_state = self.state.clone();
        self.state = state;
    }
    fn start_timer(&mut self, ctx: &Context<Self>, duration: Duration) {
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

    fn timer_button(&self, ctx: &Context<Self>) -> Html {
        let (msg, text) = match self.state {
            State::Idle | State::Paused => (Msg::StartTimer, "Start timer"),
            State::Work | State::Break => (Msg::PauseTimer, "Pause timer"),
        };
        html! {
            <button onclick={ ctx.link().callback(move |_| msg.clone()) }>
                { text }
            </button>
        }
    }
}

#[derive(Clone)]
enum Msg {
    StartTimer,
    PauseTimer,
    Tick,
    Done,
}

impl Component for PomoTimer {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            state: State::Idle,
            previous_state: State::Idle,
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
                    State::Idle => match self.previous_state {
                        State::Break | State::Idle => {
                            debug!("Start work period!");
                            self.set_state(State::Work);
                            self.work_period
                        }
                        State::Work => {
                            debug!("Start break period!");
                            self.set_state(State::Break);
                            self.break_period
                        }
                        _ => {
                            panic!("Should not happen!")
                        }
                    },
                    State::Paused => {
                        debug!("Unpause!");
                        self.set_state(self.previous_state.clone());
                        // Time when the timer was paused
                        self.time_remaining
                    }
                    _ => {
                        panic!("Should not happen!")
                    }
                };
                self.start_timer(ctx, duration);
                true
            }
            Msg::PauseTimer => {
                debug!("Pause!");
                self.set_state(State::Paused);
                self.interval = None;
                self.timer = None;
                true
            }
            Msg::Tick => {
                debug!("Tick!");
                self.time_remaining -= 1.seconds();
                true
            }
            Msg::Done => {
                debug!("Done!");
                self.set_state(State::Idle);
                self.interval = None;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                { self.timer_button(ctx) }
                // Debug
                // <p>{ format!("Current state: {:?}", self.state) }</p>
                // <p>{ format!("Previous state: {:?}", self.previous_state) }</p>
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
