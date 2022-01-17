use std::io;

use observer::*;

fn change_input_value(input: &mut InputObservable) {
    loop {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).expect("Failed read line");
        input.set_value(buf);
    }
}

pub fn main() {
    let mut input = InputObservable::new();
    let paragraph1 = ParagraphObserver::new().into();
    let paragraph2 = ParagraphObserver::new().into();
    let div1 = DivObserver::new().into();

    input.subscribe(&paragraph1);
    input.subscribe(&paragraph2);
    input.subscribe(&div1);

    input.add_event(Box::new(|state| {
        state.input_observable().notify();
    }));

    input.unsubscribe(&paragraph2);

    change_input_value(&mut input);
}
