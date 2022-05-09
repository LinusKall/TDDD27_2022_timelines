use yew::prelude::*;
//use chrono::Utc;
use chrono::prelude::*;
//use gloo::console::log;

#[function_component(Calender)]
pub fn calender() -> Html {
    let time: DateTime<Local> = Local::now();
    let current_date = time.day();
    let _current_day = time.weekday();
    let from_monday = time.weekday().number_from_monday();
    let mon = current_date - from_monday + 1;
    //let weekdays = use_state(|| Vec::new());
    let weekdays = [mon, mon+1, mon+2, mon+3, mon+5, mon+6];
    let week = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];

    html! {
        <div>
            <h2>{time.year()}{"-"}{time.month()}{"-"}{time.day()}</h2>
            <div>{"Current time: "}{time.hour()}{":"}{time.minute()}{"."}{time.second()}</div>
            <div>{"Week: "}{week.iter().collect::<Html>()}</div>
            <div>{"Week: "}{weekdays.iter().collect::<Html>()}</div>
            <h2>{"Layout"}</h2>
            <div>{"Header"}</div>
            <div>{"Days"}</div>
            <div>{"Cells"}</div>
            <div id="current_date"></div>
            <h2>{"Calender related"}</h2>
            <input type="time"/>
            <input type="datetime-local"/>
            <input type="date"/>
            <input type="month"/>
            <input type="week"/>

            <h2>{"Time"}</h2>
            <time datetime="1914-12-20"/>
            <time datetime="1914-12-20 08:00"/>
            <time datetime="08:00"/>

            <div>{time.to_rfc3339()}</div>
            <CheckYear/>
        </div>
    }
}

#[function_component(CheckYear)]
pub fn _check_year() -> Html {
    let _leapyear = {
        let curr_year = Local::now().year();
        if curr_year%2 == 0 && curr_year%100 == 0 && curr_year%400 == 0 {
            true
        } else {
           false
        };
    };

    html! {
        <leapyear/>
    }
}
