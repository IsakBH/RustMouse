use device_query::{DeviceQuery, DeviceState, Keycode};
use enigo::*;
use std::{thread, time::Duration};

fn main() {
    let mut enigo = Enigo::new();
    let device_state = DeviceState::new();

    println!("Trykk på Meta + LCtrl + LShift + Pil for å bevege musen");
    println!("Trykk på Meta + LAlt + Escape for å gå ut av programmet");

    loop {
        let keys: Vec<Keycode> = device_state.get_keys();

        // sjekker om brukeren trykker på Windows + LControl + LShift + Høyre pil
        if keys.contains(&Keycode::Meta)
            && keys.contains(&Keycode::LControl)
            && keys.contains(&Keycode::LShift)
            && keys.contains(&Keycode::Right)
        {
            // flytter musen 25 pixeler til høyre
            enigo.mouse_move_relative(25, 0);
        }

        // sjekker om brukeren trykker på Windows + LControl + LShift + Venstre pil
        if keys.contains(&Keycode::Meta)
            && keys.contains(&Keycode::LControl)
            && keys.contains(&Keycode::LShift)
            && keys.contains(&Keycode::Left)
        {
            // flytter musen 25 pixeler til venstre
            enigo.mouse_move_relative(-25, 0);
        }

        // sjekker om brukeren trykker på Windows + LControl + LShift + Nedover pil
        if keys.contains(&Keycode::Meta)
            && keys.contains(&Keycode::LControl)
            && keys.contains(&Keycode::LShift)
            && keys.contains(&Keycode::Down)
        {
            // flytter musen 25 pixeler nedover
            enigo.mouse_move_relative(0, 25);
        }

        // sjekker om brukeren trykker på Windows + LControl + LShift + Nedover pil
        if keys.contains(&Keycode::Meta)
            && keys.contains(&Keycode::LControl)
            && keys.contains(&Keycode::LShift)
            && keys.contains(&Keycode::Up)
        {
            // flytter musen 25 pixeler oppover
            enigo.mouse_move_relative(0, -25);
        }

        // sjekker etter hotkey for å gå ut av scriptet
        if keys.contains(&Keycode::Meta)
            && keys.contains(&Keycode::LAlt)
            && keys.contains(&Keycode::Escape)
        {
            println!("Exiting...");
            break;
        }

        if keys.contains(&Keycode::Meta)
            && keys.contains(&Keycode::LControl)
            && keys.contains(&Keycode::LShift)
            && keys.contains(&Keycode::RControl)
        {
            // trykker på venstre museknapp
            enigo.mouse_click(MouseButton::Left);
            thread::sleep(Duration::from_secs(1));
        }

        // liten sleep sånn at musen ikke beveger seg til en annen galakse når du trykker i et milisekund
        thread::sleep(Duration::from_millis(10));
    }
}
