use color_eyre::Result;
use tracing::debug;

use crate::structures::Message;
use crate::subscriptions::WorldMap;
use crate::transport::ThreadPeerMap;

pub async fn handle_area_subscribe(
    message: Message,
    peer_map: &ThreadPeerMap,
    world_map: &mut WorldMap,
) -> Result<()> {
    let uuid = message.sender_uuid;
    let cube = match message.position {
        Some(pos) => pos,
        None => {
            debug!(
                "invalid AreaSubscribe from peer {}, missing position",
                &uuid
            );

            return Ok(());
        }
    };

    let area_map = world_map.get_mut(&message.world_name);
    area_map.add_subscription(uuid, cube);

    Ok(())
}
