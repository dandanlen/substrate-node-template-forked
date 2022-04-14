# Basic Toaster!

This is a basic variant of decentralized toaster. But it is easily updatable to the bonus variants.

## Multislice
Basically delete all the logic that ensures that one Account can reserve only one slot in the toaster (delete everything that is connected to AccountIDWithSlot StorageMap).

## Fork
In this case we sould leave the AccountIDWithSlot StorageMap logic, but add a function that would check in case the Account had a slot reserved and then free it. Error hadling is obvious.

## Configuration
Any configuration can be done through genesis_config macro.
The logic however should change in case limit of slices per Account. Namely, we should should change the value in the AccountIDWithSlot StorageMap to some unsigned integer value e.g. u8. Then, we should add an error for overflow of this limit, and do a check in the reserve_slot function.
