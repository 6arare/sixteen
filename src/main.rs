use crossterm::{terminal::{ClearType,EnterAlternateScreen,LeaveAlternateScreen,enable_raw_mode,disable_raw_mode,Clear},
    execute,};

use std::{io::{Result,stdout},
    thread::sleep,
    time::Duration};
use chrono::{TimeDelta,Local};
fn main() ->Result<()>{
    enable_raw_mode()?;
    let mut stdout=stdout();
    // let terminal =Terminal::new();
execute!(stdout,EnterAlternateScreen)?;

let mut now =Local::now();
let target_time =now+ TimeDelta::try_seconds(6).unwrap();
while now < target_time{
    println!(" target time: {:?}   now : {:?}",target_time,now);
    now =now + TimeDelta::try_seconds(1).unwrap();
    //    target_time =target_time- TimeDelta::try_seconds(1).unwrap();
    sleep(Duration::from_secs(1));
    Clear(ClearType::All);
}
disable_raw_mode()?;
execute!(stdout,LeaveAlternateScreen)?;
Ok(())
}
// }
