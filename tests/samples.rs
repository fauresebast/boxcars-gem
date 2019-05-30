use boxcars::{self, ParserBuilder};
use std::borrow::Cow;

#[test]
fn test_sample1() {
    let data = include_bytes!("../assets/replays/good/3d07e.replay");
    let replay = ParserBuilder::new(&data[..])
        .always_check_crc()
        .must_parse_network_data()
        .parse()
        .unwrap();

    let frames = replay.network_frames.unwrap().frames;

    // random usage of the API
    let new_count = frames
        .iter()
        .flat_map(|x| x.new_actors.iter())
        .filter(|x| x.actor_id.0 != x.object_id.0)
        .count();
    assert_eq!(4545, new_count);

    let sleeping_rigid_bodies = frames
        .iter()
        .flat_map(|x| x.updated_actors.iter())
        .filter(|act| match act.attribute {
            boxcars::Attribute::RigidBody(body) => body.sleeping,
            _ => false,
        })
        .count();

    assert_eq!(32, sleeping_rigid_bodies);

    let first_actor = frames
        .iter()
        .flat_map(|x| x.new_actors.iter())
        .find(|_| true)
        .unwrap();
    let first_update = frames
        .iter()
        .flat_map(|x| x.updated_actors.iter())
        .find(|_| true)
        .unwrap();

    let first_actor_id: boxcars::ActorId = first_actor.actor_id;
    assert_eq!(0, first_actor_id.0);

    let first_object_id: boxcars::ObjectId = first_actor.object_id;
    assert_eq!(26, first_object_id.0);

    let first_stream_id: boxcars::StreamId = first_update.stream_id;
    assert_eq!(31, first_stream_id.0);
}

fn extract_online_id(replay: &boxcars::Replay<'_>, user: &str) -> (u64, boxcars::attributes::RemoteId) {
    let (_, stats) = replay
        .properties
        .iter()
        .find(|(prop, _)| *prop == "PlayerStats")
        .unwrap();

    let online_id = match stats {
        boxcars::HeaderProp::Array(arr) => {
            let our_player = arr
                .iter()
                .find(|properties| {
                    properties
                        .iter()
                        .find(|(prop, val)| {
                            *prop == "Name"
                                && *val == boxcars::HeaderProp::Str(Cow::Borrowed(user))
                        })
                        .is_some()
                })
                .unwrap();

            let (_, online_id) = our_player
                .iter()
                .find(|(prop, _val)| *prop == "OnlineID")
                .unwrap();
            if let boxcars::HeaderProp::QWord(oid) = online_id {
                *oid
            } else {
                panic!("unexpected property");
            }
        }
        _ => panic!("Expected array"),
    };


    let frames = &replay.network_frames.as_ref().unwrap().frames;
    let reservation = frames
        .iter()
        .flat_map(|x| {
            x.updated_actors.iter().filter_map(|x| {
                if let boxcars::Attribute::Reservation(r) = &x.attribute {
                    if r.name.as_ref().map(|x| x == user).unwrap_or(false) {
                        Some(dbg!(&r.unique_id.remote_id))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
        })
        .last()
        .unwrap();

    (online_id, reservation.clone())
}

#[test]
fn test_long_psynet_id() {
    let data = include_bytes!("../assets/replays/good/d52eb.replay");
    let replay = ParserBuilder::new(&data[..])
        .always_check_crc()
        .must_parse_network_data()
        .parse()
        .unwrap();

    let (header_oid, network_oid) = extract_online_id(&replay, "FunFactJac");
    assert_eq!(15633594671552264637, header_oid);

    if let boxcars::attributes::RemoteId::PsyNet(psy) = network_oid {
        assert_eq!(header_oid, psy.online_id);
    } else {
        panic!("Needed psynet remote_id");
    }
}

#[test]
fn test_short_psynet_id() {
    let data = include_bytes!("../assets/replays/good/60dfe.replay");
    let replay = ParserBuilder::new(&data[..])
        .always_check_crc()
        .must_parse_network_data()
        .parse()
        .unwrap();

    let (header_oid, network_oid) = extract_online_id(&replay, "Shope");
    assert_eq!(18091002852234862424, header_oid);

    if let boxcars::attributes::RemoteId::PsyNet(psy) = network_oid {
        assert_eq!(header_oid, psy.online_id);
    } else {
        panic!("Needed psynet remote_id");
    }
}