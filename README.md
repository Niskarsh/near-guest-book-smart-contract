# Problem Statement
Create a react app that can interact with the smart contract described below:
- It should allow users to connect their near testnet wallet with app.
- Once connected, it should allow users to send messages to smart contract, and attach near tokens(attaching near tokens is optional - dependent on user)
- It should show latest 10 messages sent to smart contract and least 10 messages sent by signed in user.
- Web App Design is up to developer. App should implement responsive design.



# Near Blockchain
App to leave feedback while leaving hotel. Can attach near tokens to messages as well.
This Guest book smart contract lives in NEAR blockchain(TESTNET for now). Clone and test with testnet accounts or just have fun with deployed link.

Contract address: guest-book2.niskarsh31.testnet

Explore it here: https://testnet.nearblocks.io/address/guest-book2.niskarsh31.testnet

# Functions exposed by smart contract

## add_message(message: String)
Allows user to add new message. Can attach additional near tokens as donation as well.

## get_messages(offset: String, limit: String)
Gets messages based on offset and limit.

Offset: 0 , Limit: 10 => Gives last 10 messages

Offset: 1 , Limit: 10 => Gives 2nd set of 10 messages from the end.

Although values are integer, contract expects them as String. Offest: 0, and limit: 10 will be the only case needed for this assignment.

## get_premium_messages(offset: String, limit: String)
Gets premium messages based on offset and limit.

Premium messages are messages which have near tokens attached to them.

Offset: 0 , Limit: 10 => Gives last 10 messages

Offset: 1 , Limit: 10 => Gives 2nd set of 10 messages from the end.

Although values are integer, contract expects them as String. Offest: 0, and limit: 10 will be the only case needed for this assignment.

## highest_donation()
Gives highest donation provided.

## messages_by_signed_in_user()
Gives all messages from signed in user(User whose wallet to connected to app at the time of call).

# References
- https://docs.near.org/build/web3-apps/quickstart
- https://docs.near.org/build/smart-contracts/quickstart
- https://docs.near.org/tutorials/examples/count-near
- https://docs.near.org/tools/near-cli
