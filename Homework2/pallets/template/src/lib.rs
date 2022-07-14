#![cfg_attr(not(feature = "std"), no_std)] // Ok

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;  

use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
use frame_support::inherent::Vec;
use frame_support::dispatch::fmt;

#[frame_support::pallet] 
pub mod pallet { 

pub use super::*;
	
	#[derive(TypeInfo, Default, Encode, Decode)]
	#[scale_info(skip_type_params(T))]
	pub struct Kitties<T: Config> {
    			pub dna: Vec<u8>,
    			pub price: u32,
    			pub gender: Gender,
    			pub owner: T::AccountId,
	}

	#[derive(TypeInfo, Encode ,Decode, Debug)]
	pub enum Gender {
		Male,
		Female,
	}

	pub type Id = u32;

	impl Default for Gender {
		fn default() -> Self {
			Gender::Male
		}
	}


	#[pallet::config]
	pub trait Config: frame_system::Config {																							
	/// Because this pallet emits events, it depends on the runtime's definition of an event.											
	type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;														
	}

	#[pallet::event]																													
	#[pallet::generate_deposit(pub(super) fn deposit_event)]																				
	pub enum Event<T: Config> {																											
		KittyStored(Vec<u8>, u32)																										
	}

	#[pallet::error]																																										
	pub enum Error<T> {																													
		KittyNotExist,
		NotKittyOwner,
		CantTransferToYoursefl,
	}

	#[pallet::pallet] 																													
	#[pallet::without_storage_info] 																									
	#[pallet::generate_store(pub(super) trait Store)] 																					
	pub struct Pallet<T>(_); 																											 

																

	#[pallet::storage]																													
	#[pallet::getter(fn all_kitty)]		 																									
	pub(super) type AllKitty<T: Config> = StorageMap<_, Blake2_128Concat, Vec<u8>, Kitties<T>, OptionQuery>;		
	
	#[pallet::storage] 
	#[pallet::getter(fn owner_kitty)]
	pub(super) type OwnerNft<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, Vec<Vec<u8>>, ValueQuery>;	

	#[pallet::storage] 																													
	#[pallet::getter(fn total_kitty)] 																										
	pub (super)type Total<T: Config> = StorageValue<_, Id, ValueQuery>; 	

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}
	

	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn create_kitty(origin: OriginFor<T>,dna: Vec<u8>, price: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let who = ensure_signed(origin)?;

			let gender = Self::gen_gender(dna.clone())?;

			let kitty = Kitties {
				dna: dna.clone(),
				price: price,
				gender: gender,
				owner: who.clone(),
			};

			// Check OwnerNft
			<OwnerNft<T>>::mutate(who.clone(),|abc| abc.push(dna.clone()));

			// Tatal Kitty		
			let mut current_id = <Total<T>>::get();
			current_id += 1;
			Total::<T>::put(current_id);

			// All Kitty
			<AllKitty<T>>::insert(dna.clone(), kitty);

			Self::deposit_event(Event::KittyStored(dna, price));

			Ok(())
		}
	
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
			pub fn transfer_kitty(origin: OriginFor<T>,to: T::AccountId, dna: Vec<u8>) -> DispatchResult {
		
			let who = ensure_signed(origin)?;

			// Check Kitty Exist ??
			let kitty_id = <AllKitty<T>>::get(&dna);
	
			ensure!(kitty_id.is_some(), <Error<T>>::KittyNotExist);

			// Check Kitty belong to ?? 
			let kitty_own = <OwnerNft<T>>::get(&who);

			ensure!(kitty_own.contains(&dna), <Error<T>>::NotKittyOwner);

			// Transfer
			ensure!(who != to, <Error<T>>::CantTransferToYoursefl);

			<OwnerNft<T>>::mutate(to.clone(), |abc| abc.push(dna.clone()));

			<OwnerNft<T>>::mutate(who.clone(), |abc| abc.retain(|cba| *cba != dna.clone()));	

			let mut kitty_transfer = <AllKitty<T>>::get(&dna).unwrap();

			kitty_transfer.owner = to.clone();

			<AllKitty<T>>::insert(dna.clone(), kitty_transfer);

			Ok(())
			}
		}

	impl<T> Pallet<T> {
			pub fn gen_gender(dna: Vec<u8>) -> Result<Gender, Error<T>> {
			let mut res = Gender::Female;
			if dna.len() % 2 == 0 {
				res = Gender::Male;
			}
			Ok(res)
		}
	}
}
