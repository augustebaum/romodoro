use gloo_console::debug;
use gloo_timers::callback::{Interval, Timeout};
use time::{ext::NumericalDuration, Duration};
use yew::prelude::*;

mod time_utils;
use time_utils::{to_millis, to_string};

#[derive(Debug, Copy, Clone)]
enum State {
    Idle,
    InWork,
    Paused,
    InBreak,
}

struct PomoTimer {
    work_period_duration: Duration,
    break_period_duration: Duration,

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

    // fn set_bg_color(&mut self, color: Color) {
    //     self.canvas_color = color;
    //     self.timer_color = color + 15% brightness
    // }

    fn start_timer(&mut self, ctx: &Context<Self>, duration: Duration) {
        self.time_remaining = duration;
        self.timer = Some({
            let link = ctx.link().clone();
            // Stop the time out the moment *after* the last Tick,
            // so that the `time_remaining` can reach 0 seconds.
            // Otherwise we'd have to manually set `time_remaining`
            // to 0 when `Done` is sent, which seems like overkill.
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
            State::InWork | State::InBreak => (Msg::PauseTimer, "Pause timer"),
        };
        let onclick = ctx.link().callback(move |_| msg.clone());
        html! {
            <button id="timer_button" {onclick}>{ text }</button>
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
            work_period_duration: 2.seconds(),  // To customize
            break_period_duration: 5.seconds(), // To customize
            time_remaining: 0.seconds(),
            timer: None,
            interval: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        use Msg::*;
        use State::*;

        match (msg, self.state, self.previous_state) {
            (StartTimer, Idle, InBreak | Idle) => {
                debug!("Start work period!");
                self.set_state(State::InWork);
                self.start_timer(ctx, self.work_period_duration);
            }
            (StartTimer, Idle, InWork) => {
                debug!("Start break period!");
                self.set_state(State::InBreak);
                self.start_timer(ctx, self.break_period_duration);
            }
            (StartTimer, Paused, _) => {
                debug!("Unpause!");
                self.set_state(self.previous_state.clone());
                // Time when the timer was paused
                self.start_timer(ctx, self.time_remaining);
            }
            (PauseTimer, _, _) => {
                debug!("Pause!");
                self.set_state(State::Paused);
                self.interval = None;
                self.timer = None;
            }
            (Tick, _, _) => {
                debug!("Tick!");
                self.time_remaining -= 1.seconds();
            }
            (Done, _, _) => {
                debug!("Done!");
                self.set_state(State::Idle);
                self.interval = None;
            }
            (_, _, _) => {
                panic!("Message-state combination should not happen!")
            }
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
            // <div id="background" class={css!("background-color: #dd6000;")}>
            <div id="background" style="background-color: #dd6000;">
            <div id="timer_div">
                { self.timer_button(ctx) }
                // Debug
                // <p>{ format!("Current state: {:?}", self.state) }</p>
                // <p>{ format!("Previous state: {:?}", self.previous_state) }</p>
                <p id="time_remaining">{ to_string(self.time_remaining) }</p>
            </div>
            </div>
            // Yuck. Could we just put that in the HTML template?
            <footer>
                { "Inspired by " }<a href="https://pomofocus.io">{ "pomofocus.io" }</a>{ "." }
            </footer>
            </>
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
