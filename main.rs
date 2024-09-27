// this code is for the stable memory implementation with some function to use for the data manipulation.

use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::DefaultMemoryImpl;
use ic_stable_structures::StableBTreeMap;
use serde::Deserialize;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::ops::{Deref, DerefMut};
use ic_stable_structures::{storable::Bound,Storable};
use candid::{CandidType, Decode, Encode};
use std::borrow::Cow;
use crate::profile_creation::User;

// memory to save data.
pub type Memory = VirtualMemory<DefaultMemoryImpl>;
// hashmap or tree to save user data.
pub type UserProfiles = StableBTreeMap<String, User, Memory>;
// pub type UserMessages = StableBTreeMap<String,Candid<VecDeque<Message>>,Memory>;

const PROFILE_DATA: MemoryId = MemoryId::new(0);
// const MESSAGE_DATA: MemoryId = MemoryId::new(1);

pub struct State {

    pub user_profiles : UserProfiles,
    // pub user_messages : UserMessages,
    

}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    pub static STATE: RefCell<State> = RefCell::new(
        MEMORY_MANAGER.with(|mm| State {
            user_profiles:UserProfiles::init(mm.borrow().get(PROFILE_DATA)),
            // user_messages:UserMessages::init(mm.borrow().get(MESSAGE_DATA)),
        })
    );
}

// function to be use to read data.
pub fn read_state<R>(f: impl FnOnce(&State) -> R) -> R {
    STATE.with(|cell| f(&cell.borrow()))
}

// function to be use to add and change the user.
pub fn mutate_state<R>(f: impl FnOnce(&mut State) -> R) -> R {
    STATE.with(|cell| f(&mut cell.borrow_mut()))
}


pub fn get_profiledata_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(PROFILE_DATA))
}

// pub fn get_messagedata_memory() -> Memory {
//     MEMORY_MANAGER.with(|m| m.borrow().get(MESSAGE_DATA))
// }




impl State {
    pub fn new() -> Self {
        Self {
            
            user_profiles: init_file_contents(),
            // user_messages: post_file_contents()
        }
    }
}


impl Default for State {
    fn default() -> Self {
        State::new()
    }
}

pub fn init_file_contents() -> StableBTreeMap<String, User,Memory> {
    StableBTreeMap::init(get_profiledata_memory())

}

// pub fn post_file_contents() -> StableBTreeMap<String,Candid<VecDeque<Message>>,Memory> {
//     StableBTreeMap::init(get_messagedata_memory())
// }

impl Storable for User {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

#[derive(Default)]
pub struct Candid<T>(pub T)
where
    T: CandidType + for<'de> Deserialize<'de>;

impl<T> Storable for Candid<T>
where
    T: CandidType + for<'de> Deserialize<'de>,
{
    const BOUND: Bound = Bound::Unbounded;

    fn to_bytes(&self) -> Cow<'_, [u8]> {
        Cow::Owned(candid::encode_one(&self.0).expect("encoding should always succeed"))
    }

    fn from_bytes(bytes: Cow<'_, [u8]>) -> Self {
        Self(candid::decode_one(bytes.as_ref()).expect("decoding should succeed"))
    }
}

impl<T> Deref for Candid<T>
where
    T: CandidType + for<'de> Deserialize<'de>,
{
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> DerefMut for Candid<T>
where
    T: CandidType + for<'de> Deserialize<'de>,
{
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}
