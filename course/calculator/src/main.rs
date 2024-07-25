use slint::Weak;

slint::slint! {
import {  Button,VerticalBox} from "std-widgets.slint";

    export global CalcLogic{
        callback button-pressed(string);

    }

    component Button {
    in property <string> text;
    Rectangle {
        background: ta.pressed ? #3F3F3F : ta.has-hover ? #4d4d4d : #333333;
        animate background { duration: 100ms; }
        border-radius: 4px;
        border-width: 2px;
        border-color: self.background.darker(20%);
        ta := TouchArea {}
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

Text { text:  counter; colspan: 3; }
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
                Button { text: "0";}
                Button { text: "="; col: 2; }
                Button { text: "/"; }
            }


        }
    }
}//end of slint








fn main() {
    let app: App = App::new().unwrap();
    let weak: Weak<App> = app.as_weak();



    app.run().unwrap();
    println!("Hello Mysh");
}

