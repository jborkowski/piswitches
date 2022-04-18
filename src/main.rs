#![allow(unreachable_code)]
use rouille::*;
use rust_gpiozero::*;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    // Create a new relay for power switch attached to Pin 23
    let power_sw = DigitalOutputDevice::new(23);
    power_sw.on();

    let reset_sw = DigitalOutputDevice::new(24);
    reset_sw.on();

    println!("Now listening on 0.0.0.0:80");

    rouille::start_server("0.0.0.0:80", move |request| {
        router!(request,
           (GET) (/) => {
               rouille::Response::html(HOME)
           },
           (POST) (/wakeup) => {
               sleep(Duration::from_millis(200));
               power_sw.off();
               sleep(Duration::from_millis(200));
               power_sw.on();
               rouille::Response::text("woke up")

            },
            (POST) (/reset) => {
               sleep(Duration::from_millis(500));
               reset_sw.off();
               sleep(Duration::from_millis(50));
               reset_sw.on();
               rouille::Response::text("reset")
            },
            (GET) (/panic) => {
               panic!("Oops!")
            },
            _ => rouille::Response::empty_404()
        )
    });
}

static HOME: &'static str = r#"
<html>
    <head>
        <title>Interact with fwooper</title>
        <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bulma@0.9.3/css/bulma.min.css">
    </head>
    <body>
    <form action="wakeup" method="POST" enctype="multipart/form-data">
        <p> <button class="button">Kick</button> </p>
    </form>
    </body>
</html>
"#;
