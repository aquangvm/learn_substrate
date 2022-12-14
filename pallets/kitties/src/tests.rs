use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn test_create() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1); // resolve error https://substrate.stackexchange.com/questions/4511/test-panic-with-randomnessrandom-attempt-to-subtract-with-overflow
		assert_eq!(MaxKittyOwned::get(), 1);

		// create kitti
		assert_ok!(MyKitti::create_kitty(RuntimeOrigin::signed(1), 100));
		assert_eq!(<KittyId<Test>>::get(), 1);

		//  owned kitty
		let kitties_owned = <KittiesOwned<Test>>::get(1);
		assert_eq!(kitties_owned.len(), 1);

	})
}

#[test]
fn test_transfer (){
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		assert_ok!(Mykitti::create_kitty(RuntimeOrigin::signed(1), 100));
		let kitties_owned = <KittiesOwned<Test>>::get(1);
		assert_eq!(kitties_owned.len(), 1);
        let kitty_dna = *kitties_owned.last().unwrap();

		assert_ok!(MyKitti::transfer(RuntimeOrigin::signed(1), 2, kitty_dna));

		let owned_old = <KittiesOwned<Test>>::get(1);
		assert_eq!(owned_old.len(), 0);
		let owned_new = <KittiesOwned<Test>>::get(2);
		assert_eq!(owned_new.len(), 1);

		
		let kitty_dna = *kitties_owned_2.last().unwrap();
		let kitty = <Kitties<Test>>::get(kitty_dna).unwrap();
		assert_eq!(kitty.owner, 2);
	})



}
