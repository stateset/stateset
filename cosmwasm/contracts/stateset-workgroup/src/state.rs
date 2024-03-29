use cosmwasm_std::Addr;
use workgroup::TOTAL_KEY;
use cw_controllers::{Admin, Hooks};
use cw_storage_plus::{Item, SnapshotMap, Strategy};

pub const ADMIN: Admin = Admin::new("admin");
pub const HOOKS: Hooks = Hooks::new("workgroup-hooks");

pub const TOTAL: Item<u64> = Item::new(TOTAL_KEY);

pub const MEMBERS: SnapshotMap<&Addr, u64> = SnapshotMap::new(
    cw4::MEMBERS_KEY,
    cw4::MEMBERS_CHECKPOINTS,
    cw4::MEMBERS_CHANGELOG,
    Strategy::EveryBlock,
);
