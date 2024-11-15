#[derive(Debug)]
struct CubeSat {
    id: u64,
    mailbox: Mailbox,
}

#[derive(Debug)]
struct Mailbox {
    messages: Vec<Message>,
}


#[derive(Debug)]
struct Message {
    to: u64,
    content: String,
}
struct GroundStation;

impl GroundStation {
    fn send(&self, mailbox: &mut Mailbox, to: &CubeSat, msg: Message) {
        mailbox.post(to, msg); // 1
    }

    fn connect(&self, sat_id: u64) -> CubeSat {
        CubeSat { id: sat_id, mailbox: Mailbox { messages: vec![] }}
    }
}

impl CubeSat {
    fn recv(&self, mailbox: &mut Mailbox) -> Option<Message> { // 2
        mailbox.deliver(&self)
    }
}

impl Mailbox {
    fn post(&mut self, msg: Message) {
        self.messages.push(msg);            // 3
    }

    fn deliver( &mut self, recipient: &CubeSat ) -> Option<Message> { // 4
        for i in 0..self.messages.len() {
            if self.messages[i].to == recipient.id {
                let msg = self.messages.remove(i);
                return Some(msg);                                    // 5
            }
        }

        None                                    // 6
    }
}

fn fetch_sat_ids() -> Vec<u64> {
    vec![1,2,3]
}

fn main() {
    let base = GroundStation {};

    let sat_ids = fetch_sat_ids();

    for sat_id in sat_ids {
        let mut sat = base.connect(sat_id);

        base.send(&mut sat, Message::from("hello"));
    }

    println!("t0: {:?}", sat_a);

    base.send(&mut sat_a,
                    Message::from("hello there!")); // 1

    println!("t1: {:?}", sat_a);

    let msg = sat_a.recv();
    println!("t2: {:?}", sat_a);

    println!("msg: {:?}", msg);
}