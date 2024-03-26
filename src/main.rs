use std::{thread::sleep,
    time::Duration};
fn main() {
    let mut target_time =16*60*60;
    let mut now =0;
    loop{
           println!(" remaining: {:?}   elapsed: {:?}",target_time,now);
           sleep(Duration::from_secs(1));
           now +=1;
           target_time -=1;
           if now == target_time{
            break;
           }
    }
}
// }
