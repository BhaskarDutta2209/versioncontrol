#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::{
    decl_error, decl_event, decl_module, decl_storage, dispatch, ensure,
    StorageMap,
    traits::Get,
    RuntimeDebug,
    Hashable
};
use frame_system::ensure_signed;
use sp_std::vec::Vec;
// use sp_io::hashing::blake2_128;

// Defining the structure of the content
#[derive(Default, Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq)]
pub struct Content {
    title: String,        // Appropriate title of the content
    description: String,  // Appropriate description for the content
    metadata_uri: String, // metadataURI hosted in IPFS
    version: u32,         // Version number of the content
    is_forked: Option<Vec<u8>>
}

#[derive(Default, Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq)]
pub struct Contribution<AccountId> {
    account_id: AccountId,
    percentage_contribution: u8,
}

pub trait Config: frame_system::Config {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

decl_storage! {
    trait Store for Module<T: Config> as TemplateModule {

        Contents: map hasher(blake2_128_concat) Vec<u8> => (Content, Vec<Contribution<T::AccountId>>);

        pub Something get(fn something): Option<u32>; //TODO: To be deleted
        // // Store all the contents
        // pub Contents get(fn contents): map hasher(blake2_128_concat) u32 => Option<Content>;
        // // Used to get the next content id
        // pub NextContentId get(fn next_content_id): u32;
        // // Get vector of all the contributors associated with a particular Content
        // pub Contributors get(fn get_contributors): map hasher(blake2_128_concat) u32 => Vec<T::AccountId>;
        // // Double mapping to get the percentage contribution of an account in a particular asset
        // pub PercentageContribution get(fn percentage_contributed): double_map hasher(blake2_128_concat) T::AccountId, hasher(blake2_128_concat) u32 => Option<u8>;
    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Config>::AccountId,
    {
        ContentCreated(Vec<u8>, AccountId),     // [content_hash, account id]
        ContentMerged(u32, AccountId), // [content id, account id of the account updating the content]
        ContentForked(u32, u32, AccountId), // [content id of content forked, content id of new created content, account id creating the forke]
        SomethingStored(u32, AccountId),
    }
);

decl_error! {
    pub enum Error for Module<T: Config> {
        NoneValue,
        StorageOverflow,
        ContentAlreadyExist,
        NoContentPresent
    }
}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {

        type Error = Error<T>;

        fn deposit_event() = default;

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn do_something(origin, something: u32) -> dispatch::DispatchResult {

            let who = ensure_signed(origin)?;

            Something::put(something);

            Self::deposit_event(RawEvent::SomethingStored(something, who));

            Ok(())
        }

        #[weight = 10_000 + T::DbWeight::get().reads_writes(1,1)]
        pub fn cause_error(origin) -> dispatch::DispatchResult {
            let _who = ensure_signed(origin)?;

            match Something::get() {

                None => Err(Error::<T>::NoneValue)?,
                Some(old) => {

                    let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;

                    Something::put(new);
                    Ok(())
                },
            }
        }

        #[weight = 10_000 + T::DbWeight::get().reads_writes(1,1)]
        pub fn create_content(origin, title:String, description: String, metadata_uri: String) {

            // check proper sender
            let sender = ensure_signed(origin);

            // create object of the content
            let content = Content {
                title,
                description,
                metadata_uri,
                version: 1,
                is_forked: None
            };

            let mut contributors = Vec::new();
            contributors.push(Contribution{
                account_id: &sender,
                percentage_contribution: 100
            });

            let hash_of_content = content.blake2_128_concat();

            // Ensure that this hash is not already present
            ensure!(!Contents::<T>::contains_key(&hash_of_content), Error::<T>::ContentAlreadyExist);

            // Store the content
            Contents::<T>::insert(hash_of_content, (content, contributors));

            Self::deposit_event(RawEvent::ContentCreated(hash_of_content, sender));

        }
    }
}

// impl<T: Config> Module<T> {
//     fn get_next_content_id() -> sp_std::result::Result<u32, DispatchError> {
//         NextContentId::try_mutate(|next_id| -> sp_std::result::Result<u32, DispatchError> {
//             let current_id: u32 = *next_id;
//             *next_id = next_id
//                 .checked_add(1)
//                 .ok_or(Error::<T>::ContentIdOverflow)?;
//             Ok(current_id)
//         })
//     }
// }
