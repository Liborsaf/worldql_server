use std::collections::{HashMap, HashSet};

use tracing::trace;
use uuid::Uuid;

use super::{CubeArea, ToCubeArea};

pub struct AreaMap {
    cube_size: u16,
    world_name: String,

    map: HashMap<CubeArea, HashSet<Uuid>>,
    empty_set: HashSet<Uuid>,
}

impl AreaMap {
    pub fn new(cube_size: u16, world_name: String) -> Self {
        Self {
            cube_size,
            world_name,

            map: HashMap::new(),
            empty_set: HashSet::new(),
        }
    }

    /// Returns `true` if the [`crate::transport::Peer`] corresponding to the given UUID
    /// is subscribed to the given area.
    pub fn is_peer_subscribed(&self, uuid: &Uuid, cube: impl ToCubeArea) -> bool {
        let cube = cube.to_cube_area(self.cube_size);
        let entry = self.map.get(&cube);

        match entry {
            None => false,
            Some(set) => set.contains(uuid),
        }
    }

    /// Returns a vector of [`crate::transport::Peer`] structs which are subscribed to the
    /// given area.
    pub fn get_subscribed_peers(&self, cube: impl ToCubeArea) -> impl Iterator<Item = Uuid> + '_ {
        let cube = cube.to_cube_area(self.cube_size);
        let entry = self.map.get(&cube);

        match entry {
            None => self.empty_set.iter().copied(),
            Some(set) => set.iter().copied(),
        }
    }

    /// If the subscription was added, `true` is returned.
    ///
    /// If the subscription was already present, `false` is returned
    pub fn add_subscription(&mut self, uuid: Uuid, cube: impl ToCubeArea) -> bool {
        let cube = cube.to_cube_area(self.cube_size);
        let entry = self.map.entry(cube).or_insert_with(Default::default);

        trace!(
            "peer {} subscribed to region {} in world {}",
            &uuid,
            &cube,
            &self.world_name
        );

        entry.insert(uuid)
    }

    /// Returns whether the subscription was removed.
    pub fn remove_subscription(&mut self, uuid: &Uuid, cube: impl ToCubeArea) -> bool {
        let cube = cube.to_cube_area(self.cube_size);

        // Early return if no subscriptions are present
        if !self.map.contains_key(&cube) {
            return false;
        }

        // Remove from HashSet
        trace!(
            "peer {} unsubscribed from region {} in world {}",
            &uuid,
            &cube,
            &self.world_name
        );

        let entry = self.map.entry(cube).or_insert_with(Default::default);
        let removed = entry.remove(uuid);

        // Remove HashSet from HashMap if empty
        if entry.is_empty() {
            self.map.remove(&cube);
        }

        removed
    }

    /// Completely removes a [`crate::transport::Peer`] from the map.
    ///
    /// Used in the event of a disconnect.
    pub fn remove_peer(&mut self, uuid: &Uuid) -> bool {
        let mut removed = false;
        for (area, peers) in &mut self.map {
            if peers.remove(uuid) {
                removed = true;
            }
        }

        removed
    }
}
