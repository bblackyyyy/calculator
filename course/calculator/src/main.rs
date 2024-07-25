
use std::cell::RefCell;
use std::rc::Rc;
use slint::private_unstable_api::re_exports::MouseCursor::Default;
use slint::Weak;

slint::slint! {
import {  Button,VerticalBox} from "std-widgets.slint";

    export global CalcLogic{
        callback button-pressed(string);

    }

    component Button {
    in property <string> text;
        in property <brush> background: #333333;
    Rectangle {
        background: ta.pressed ? #3F3F3F : ta.has-hover ? #4d4d4d : background;
        animate background { duration: 100ms; }
        border-radius: 4px;
        border-width: 2px;
        border-color: self.background.darker(20%);
        ta := TouchArea {
                clicked => { CalcLogic.button-pressed(root.text);

                }
            }
        Text { text: root.text; }
    }
}







export component App inherits Window {
       title: "Calc";

        width : 300px;
        height: 400px;
        background: #1C1C1C;

in property <int> counter: 0;

GridLayout {
            padding: 20px;
            spacing: 10px;

Text { text:  counter;
                colspan: 3;

            }
            Row{
                Button { text: "1"; }
                Button { text: "2"; }
                Button { text: "3"; }
                Button { text: "+"; }
            }
            Row{
                Button { text: "4"; }
                Button { text: "5"; }
                Button { text: "6"; }
                Button { text: "-"; }
            }
            Row{
                Button { text: "7"; }
                Button { text: "8"; }
                Button { text: "9"; }
                Button { text: "*"; }
            }
            Row{
                Button { text: "0"; col: 1;}
                Button { text: "C"; col: 0; background: #FF0000;}
                Button { text: "="; col: 2; }
                Button { text: "/"; }
            }


        }
    }
}//end of slint

#[derive(Debug, Default)]
struct Calculator{
    prev_num: i32,
    current_num: i32,
    operator: slint::SharedString
}







fn main() {

    let app: App = App::new().unwrap();
    let weak: Weak<App> = app.as_weak();
    let state = Rc::new(RefCell::new(Calculator::default()));
    app.global::<CalcLogic>().on_button_pressed(move |counter| {
        let app = weak.unwrap();
        let mut state = state.borrow_mut();

        if let Ok(counter) = counter.parse::<i32>() {
            state.current_num *= 10;
            state.current_num += counter;
            app.set_counter(state.current_num);
            return;
        }
        if counter.as_str() == "=" {
            let result = match state.operator.as_str() {
                "+" => state.prev_num + state.current_num,
                "-" => state.prev_num - state.current_num,
                "*" => state.prev_num * state.current_num,
                "/" => state.prev_num / state.current_num,

                _ => return
            };
            app.set_counter(result);

            state.prev_num = result;
            state.current_num = 0;
            state.operator = slint::SharedString::new();
        }

        if counter.as_str() == "C" {
            state.prev_num = 0;
            state.current_num = 0;
            state.operator = slint::SharedString::new();
            app.set_counter(0);
            return;
        } else {
            state.operator = counter.clone();

            state.prev_num = app.get_counter();
            state.current_num = 0;
        }
    }
    );


    app.run().unwrap();
    println!("Hello Mysh");
}

