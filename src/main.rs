use gix_protocol::fetch;
use gix_protocol::fetch::negotiate::one_round::State;
use gix_protocol::fetch::negotiate::{Action, Error, Round};
use gix_protocol::fetch::{handshake, Arguments, Context, Negotiate, Response};
use gix_protocol::transport::connect::Options;
use std::sync::atomic::AtomicBool;
use gix_hash::Kind;

struct Negotiator {}

impl Negotiate for Negotiator {
    fn mark_complete_and_common_ref(&mut self) -> Result<Action, Error> {
        Ok(Action::MustNegotiate {
            remote_ref_target_known: vec![],
        })
        // todo!("mark_complete_and_common_ref")
    }

    fn add_wants(&mut self, arguments: &mut Arguments, remote_ref_target_known: &[bool]) -> bool {
        let blob_id = const_hex::decode("a46bd72d1f660ee3f1d9482bb52fdd8b15683c4a").unwrap();
        arguments.want(gix_hash::oid::try_from_bytes(&blob_id).unwrap());
        true
    }

    fn one_round(
        &mut self,
        state: &mut State,
        arguments: &mut Arguments,
        previous_response: Option<&Response>,
    ) -> Result<(Round, bool), Error> {
        Ok((Round {
            haves_sent: 0,
            in_vain: 0,
            haves_to_send: 0,
            previous_response_had_at_least_one_in_common: false,
        }, true))
        // todo!("one_round {state:#?} {arguments:#?} {previous_response:#?}")
    }
}

fn main() {
    let progress = prodash::tree::Root::new();
    // let progress = prod
    let options = Options::default();
    let mut transport =
        gix_protocol::transport::client::connect("git@github.com:orf/gping.git", options).unwrap();

    let mut handshake = handshake(
        &mut transport,
        |r| todo!("Auth!"),
        vec![],
        &mut progress.add_child("Fetch"),
    )
    .unwrap();


    println!("Connected! {handshake:#?}");

    let mut negotiator = Negotiator {};
    let mut interrupt = AtomicBool::default();

    let context = Context {
        handshake: &mut handshake,
        transport: &mut transport,
        user_agent: ("yo", None),
        trace_packetlines: false,
    };

    let x = fetch(
        &mut negotiator,
        |reader, progress, should_interrupt| {
            let mut file = std::fs::File::create("output.pack").unwrap();
            std::io::copy(reader, &mut file)?;
            Ok::<_, std::io::Error>(true)
            // todo!("Progress!");
            // Ok::<_, std::io::Error>(true)
        },
        &mut progress.add_child("fetch"),
        &mut interrupt,
        context,
        gix_protocol::fetch::Options {
            shallow_file: Default::default(),
            shallow: &Default::default(),
            tags: Default::default(),
            reject_shallow_remote: false,
        },
    ).unwrap();

    let pack = gix_pack::data::File::at("output.pack", Kind::Sha1).unwrap();
    for item in pack.streaming_iter().unwrap() {
        let item = item.unwrap();
        println!("{:#?}", item.header);
    }
}
